use std::error::{Error as StdError, Error};

use async_trait::async_trait;
use futures::StreamExt;
use langchain_rust::{
    chain::{Chain, ConversationalRetrieverChainBuilder},
    document_loaders::{Loader, TextLoader},
    llm::{OpenAI,OpenAIConfig, OpenAIModel},
    memory::SimpleMemory,
    prompt::HumanMessagePromptTemplate,
    schemas::{Document, Message, Retriever},
    message_formatter,
    prompt_args,
    fmt_message,
    fmt_template,
    template_jinja2,
};

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

        LogInspector {
            client
        }
    }

    // Inside analyze function, add these debug points:
    pub async fn analyze(
        &self,
        log_path: &str,
        question: &str,
    ) -> Result<String, Box<dyn StdError>> {
        // After loading documents
        let loader = TextLoader::new(log_path);
        let mut docs_stream = loader.load().await?;

        let mut docs = Vec::new();
        while let Some(doc) = docs_stream.next().await {
            docs.push(doc?);
        }

        let retriever = MemoryRetriever::new(docs);


        let prompt = message_formatter![
    fmt_message!(Message::new_system_message(
        "You are a log analysis expert. Return structured insights in the exact format specified."
    )),
    fmt_template!(HumanMessagePromptTemplate::new(
        template_jinja2!("
            Analyze these logs and return in this exact format:
            Status: Return one of these error codes (comma-separated if multiple, max 3): SUCCESS, USER_CODE_ERROR, SCALING_ERROR, SPARK_ERROR, SPARK_OOM_ERROR, NETWORK_ERROR, PERMISSION_ERROR, UNKNOWN_ERROR

            Then list exactly 3 key points with specific metrics where available:
            - For timeouts: include duration (ms/s)
            - For memory issues: include usage values (MB/GB)
            - For connection errors: include retry counts or failure duration
            - For performance issues: include specific thresholds or values

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
        let result = chain.invoke(input_variables).await;
        result.map_err(|e| Box::new(e) as Box<dyn StdError>)
    }
}
