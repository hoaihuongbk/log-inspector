use crate::error_types::ErrorType;
use crate::openai_client::OpenAIClient;
use std::error::Error as StdError;

pub struct LogInspector {
    client: OpenAIClient,
}

impl LogInspector {
    pub fn new(api_key: String, host: String) -> Self {
        LogInspector {
            client: OpenAIClient::new(api_key, host),
        }
    }

    pub async fn error_classify(&self, log_content: &str) -> Result<String, Box<dyn StdError>> {
        // let prompt = r#"
        //     Classify the errors in these logs into the following categories only:
        //     - USER_CODE_ERROR: User code related issues
        //     - SCALING_ERROR: Resource scaling issues
        //     - SPARK_ERROR: General Spark runtime errors
        //     - SPARK_OOM_ERROR: Spark out of memory errors
        //     - NETWORK_ERROR: Network connectivity issues
        //     - PERMISSION_ERROR: Access and permission issues
        //     - UNKNOWN_ERROR: Errors that don't fit other categories
        //
        //     List the error codes from highest to lowest probability (left to right).
        //     Return only the comma-separated list, maximum 3 codes, no other text.
        // "#;

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
            Provide a summary in this exact format:
            First line: Brief overview of the log situation in one or two sentences.
            Then list up to 3 key points with specific metrics where available:
            - For timeouts: include duration (ms/s)
            - For memory issues: include usage values (MB/GB)
            - For connection errors: include retry counts or failure duration
            - For performance issues: include specific thresholds or values

            Use "-" (hyphen) for each point. Include only metrics that appear in the logs.
        "#;

        self.client.chat(prompt, log_content).await
    }
}
