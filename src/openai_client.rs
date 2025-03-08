use reqwest;
use serde::{Deserialize, Serialize};
use std::error::Error as StdError;

#[derive(Debug, Serialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Debug, Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<Message>,
    temperature: f32,
}

#[derive(Debug, Deserialize)]
struct ChatCompletion {
    id: String,
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: CompletionMessage,
}

#[derive(Debug, Deserialize)]
struct CompletionMessage {
    content: String,
}

pub struct OpenAIClient {
    api_key: String,
    host: String,
    client: reqwest::Client,
}

impl OpenAIClient {
    pub fn new(api_key: String, host: String) -> Self {
        OpenAIClient {
            api_key,
            host,
            client: reqwest::Client::new(),
        }
    }

    pub async fn chat(&self, prompt: &str, content: &str) -> Result<String, Box<dyn StdError>> {
        let messages = vec![
            Message {
                role: "system".to_string(),
                content: prompt.to_string(),
            },
            Message {
                role: "user".to_string(),
                content: content.to_string(),
            },
        ];

        let request = ChatRequest {
            model: "gpt-3.5-turbo".to_string(),
            messages,
            temperature: 0.3,
        };

        let response = self
            .client
            .post(format!("{}/v1/chat/completions", self.host))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request)
            .send()
            .await?;

        let completion: ChatCompletion = response.json().await?;
        Ok(completion.choices[0].message.content.clone())
    }
}
