use crate::openai_client::OpenAIClient;
use std::error::{Error as StdError, Error};
// Add at the top with other imports

// use langchain_rust::{
//     chain::{conversation::ConversationalRetrievalQAChain, Chain},
//     document_loaders::text::TextLoader,
//     embeddings::openai::OpenAiEmbeddings,
//     memory::conversation::ConversationMemory,
//     vectorstores::qdrant::QdrantStore,
// };
use futures::StreamExt;  // Add this import at the top
use langchain_rust::{
    add_documents,
    chain::{Chain, ConversationalChain, ConversationalRetrieverChainBuilder},
    embedding::openai::OpenAiEmbedder,
    llm::{OpenAI, OpenAIModel},
    memory::SimpleMemory,
    prompt_args,
    document_loaders::{Loader, TextLoader},
    llm::{Config, OpenAIConfig},
    message_formatter,
    fmt_message, fmt_template,
    template_jinja2,
    schemas::{Document, Message, Retriever},
    prompt::HumanMessagePromptTemplate,
    // vectorstore::{sqlite_vec::StoreBuilder, Retriever, VectorStore},
};
use std::collections::HashMap;
use async_trait::async_trait;
// use async_openai:: config::{Config,OpenAIConfig};

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
    client: OpenAIClient,
    config: OpenAIConfig,
}

impl LogInspector {
    pub fn new(api_key: String, host: String) -> Self {
        LogInspector {
            client: OpenAIClient::new(api_key.clone(), host.clone()),
            config: OpenAIConfig::new()
                .with_api_key(api_key.clone())
                .with_api_base(host.clone()),
        }
    }

    pub async fn error_classify(&self, log_content: &str) -> Result<String, Box<dyn StdError>> {
        let prompt = r#"
            Analyze the logs and return codes from these categories:
            - SUCCESS: Successful execution without errors
            - USER_CODE_ERROR
            - SCALING_ERROR
            - SPARK_ERROR
            - SPARK_OOM_ERROR
            - NETWORK_ERROR
            - PERMISSION_ERROR
            - UNKNOWN_ERROR

            Rules:
            1. If execution was successful, return only "SUCCESS"
            2. Otherwise, return up to 3 error codes from highest to lowest probability (left to right)

            Return only the comma-separated list, no other text.
        "#;

        self.client.chat(prompt, log_content).await
    }

    pub async fn summarize(&self, log_content: &str) -> Result<String, Box<dyn StdError>> {
        let prompt = r#"
            Summarize the log in this format:
            One or two sentences describing the overall situation.

            Then list up to 3 key points with specific metrics where available:
            - For timeouts: include duration (ms/s)
            - For memory issues: include usage values (MB/GB)
            - For connection errors: include retry counts or failure duration
            - For performance issues: include specific thresholds or values

            Use "-" (hyphen) for each point. Include only metrics that appear in the logs.
            Do not include any labels or prefixes in your response.
        "#;

        self.client.chat(prompt, log_content).await
    }

    // Inside analyze function, add these debug points:
    pub async fn analyze(&self, log_path: &str, question: &str) -> Result<String, Box<dyn StdError>> {
        println!("Starting analysis with log_path: {}", log_path);

        // After loading documents
        let loader = TextLoader::new(log_path);
        let mut docs_stream = loader.load().await?;
        println!("Loaded document stream");

        let mut docs = Vec::new();
        while let Some(doc) = docs_stream.next().await {
            docs.push(doc?);
        }
        println!("Collected {} documents", docs.len());

        // Before creating LLM
        println!("OpenAI Config: {:?}", self.config);

        let llm = OpenAI::new(self.config.clone())
            .with_model(OpenAIModel::Gpt35.to_string());
        println!("Created LLM instance");

        let retriever = MemoryRetriever::new(docs);
        println!("Created retriever");

        // Before chain creation
        println!("Building chain with question: {}", question);

        // Update the prompt to explicitly include context
//         let prompt = message_formatter![
//     fmt_message!(Message::new_system_message("Act as a Spark Master")),
//     fmt_template!(HumanMessagePromptTemplate::new(
//         template_jinja2!("
//             Log Content:
//             {{context}}
//
//             Question: {{question}}
//         ", "context", "question"
//     )))
// ];

    //     let prompt = message_formatter![
    //     fmt_message!(Message::new_system_message(
    //         "You are a log analysis expert. Always provide direct, actionable insights from the logs."
    //     )),
    //     fmt_template!(HumanMessagePromptTemplate::new(
    //         template_jinja2!("
    //             Analyze these logs and provide key insights:
    //             {{context}}
    //
    //             Question: {{question}}
    //
    //             Focus on:
    //             - Error messages and exceptions
    //             - Stack traces
    //             - Performance metrics
    //             - System state changes
    //
    //             Provide a clear, direct response based on the log content.
    //         ", "context", "question"
    //         )))
    // ];

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
            .llm(llm)
            .rephrase_question(true)
            .retriever(retriever)
            .memory(SimpleMemory::new().into())
            .prompt(prompt)
            .build()
            .expect("Error building ConversationalChain");
        println!("Chain built successfully");

        let input_variables = prompt_args! {
        "question" => question,
    };
        println!("Prepared input variables: {:?}", input_variables);

        // Before chain invocation
        println!("Invoking chain");
        let result = chain.invoke(input_variables).await;
        // println!("Chain result: {:?}", result);
        // if let Ok(result) = result {
        //     println!("Result: {:?}", result);
        // }

        result.map_err(|e| Box::new(e) as Box<dyn StdError>)
    }
}
