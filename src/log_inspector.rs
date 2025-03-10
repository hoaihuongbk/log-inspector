use std::error::{Error as StdError, Error};
use std::time::Instant;

use async_trait::async_trait;
use futures::StreamExt;
use langchain_rust::{
    chain::{Chain, ConversationalRetrieverChainBuilder},
    document_loaders::{Loader, TextLoader},
    fmt_message, fmt_template,
    llm::{OpenAI, OpenAIConfig, OpenAIModel},
    memory::SimpleMemory,
    message_formatter,
    prompt::HumanMessagePromptTemplate,
    prompt_args,
    schemas::{Document, Message, Retriever},
    template_jinja2,
    text_splitter::TokenSplitter,
};
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};

pub struct MemoryRetriever {
    docs: Vec<Document>,
}

impl MemoryRetriever {
    pub fn new(docs: Vec<Document>) -> Self {
        Self { docs }
    }
}

#[async_trait]
impl Retriever for MemoryRetriever {
    async fn get_relevant_documents(&self, _query: &str) -> Result<Vec<Document>, Box<dyn Error>> {
        Ok(self.docs.clone())
    }
}

pub struct LogInspector {
    client: OpenAI<OpenAIConfig>,
}

impl LogInspector {
    pub fn new(api_key: String, host: String) -> Self {
        let config = OpenAIConfig::new()
            .with_api_key(api_key)
            .with_api_base(host);

        let client = OpenAI::new(config).with_model(OpenAIModel::Gpt35.to_string());

        LogInspector { client }
    }

    // Inside analyze function, add these debug points:
    pub async fn analyze(
        &self,
        log_path: &str,
        question: &str,
    ) -> Result<String, Box<dyn StdError>> {

        // Set start time
        let start = Instant::now();

        // Create spinner
        let spinner = ProgressBar::new_spinner();
        spinner.set_style(
            ProgressStyle::default_spinner()
                .tick_chars("⣾⣽⣻⢿⡿⣟⣯⣷")
                .template("{spinner:.cyan} {msg} ({percent}%)")
                .unwrap()
        );


        spinner.set_message("Loading log file...");
        let splitter = TokenSplitter::default();
        let loader = TextLoader::new(log_path);
        let mut docs_stream = loader.load_and_split(splitter).await?;

        spinner.set_message("Processing documents...");
        let mut docs = Vec::new();
        while let Some(doc) = docs_stream.next().await {
            docs.push(doc?);
        }

        let retriever = MemoryRetriever::new(docs);

        let prompt = message_formatter![
    fmt_message!(Message::new_system_message(
        "You are a log analysis expert. Return response in this exact format with no deviations."
    )),
    fmt_template!(HumanMessagePromptTemplate::new(
        template_jinja2!("
            Analyze these logs and return in this exact format:

            ERROR_CODES: [list up to 3 codes separated by comma]
            SUMMARY: [one sentence overview]
            METRICS:
            - [first metric with specific values]
            - [second metric with specific values]
            - [third metric with specific values]

            Use these error codes only:
            SUCCESS, USER_CODE_ERROR, SCALING_ERROR, SPARK_ERROR, SPARK_OOM_ERROR, NETWORK_ERROR, PERMISSION_ERROR, UNKNOWN_ERROR

            Each metric must include specific values:
            - For timeouts: duration (ms/s)
            - For memory: usage (MB/GB)
            - For connections: retry counts
            - For performance: thresholds/values

            Context:
            {{context}}

            Question: {{question}}
        ", "context", "question"
    )))
];

        let chain = ConversationalRetrieverChainBuilder::new()
            .llm(self.client.clone())
            .rephrase_question(true)
            .retriever(retriever)
            .memory(SimpleMemory::new().into())
            .prompt(prompt)
            .build()
            .expect("Error building ConversationalChain");

        let input_variables = prompt_args! {
            "question" => question,
        };

        // Before chain invocation
        spinner.set_message("Analyzing logs...");
        let result = chain.invoke(input_variables).await?;

        // Split the result into error code and points
        let lines: Vec<&str> = result.lines().collect();

        // Clear the spinner
        spinner.finish_and_clear();

        // Set end time
        let duration = start.elapsed();

        let mut formatted = String::new();

        // Vibrant header colors
        formatted.push_str(&format!("🔍 {}\n", "Log Analysis Report".bright_magenta().bold())); // Bright pink
        formatted.push_str(&format!("📄 File: {}\n", log_path.bright_blue())); // Bright blue
        formatted.push_str(&format!("⏱️ Completed in {:.2}s\n\n", duration.as_secs_f64()));

        // Color labels with vibrant colors
        for line in result.lines() {
            if line.starts_with("ERROR_CODES:") {
                let (label, content) = line.split_once(':').unwrap_or((line, ""));
                formatted.push_str(&format!("{}:{}\n",
                                            label.magenta().bold(), // Pink
                                            content
                ));
            } else if line.starts_with("SUMMARY:") {
                let (label, content) = line.split_once(':').unwrap_or((line, ""));
                formatted.push_str(&format!("{}:{}\n",
                                            label.bright_blue().bold(), // Bright blue
                                            content
                ));
            } else if line.starts_with("METRICS:") {
                let (label, content) = line.split_once(':').unwrap_or((line, ""));
                formatted.push_str(&format!("{}:{}\n",
                                            label.cyan().bold(), // Cyan
                                            content
                ));
            } else {
                formatted.push_str(&format!("{}\n", line));
            }
        }



        Ok(formatted)
    }

}
