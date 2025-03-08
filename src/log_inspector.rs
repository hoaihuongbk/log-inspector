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
}
