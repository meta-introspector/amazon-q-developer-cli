use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::api_client::model::{ChatResponseStream, ConversationState, ChatMessage};

#[derive(Clone, Debug)]
pub struct GeminiClient {
    client: Client,
    api_key: String,
}

impl GeminiClient {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
        }
    }

    pub async fn send_message(
        &self,
        conversation: ConversationState,
    ) -> Result<Vec<ChatResponseStream>, reqwest::Error> {
        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/gemini-pro:generateContent?key={}",
            self.api_key
        );

        let mut contents = Vec::new();
        if let Some(history) = conversation.history {
            for message in history {
                contents.push(message.into());
            }
        }
        contents.push(conversation.user_input_message.into());

        let request_body = GeminiRequest { contents };

        let response = self
            .client
            .post(&url)
            .json(&request_body)
            .send()
            .await?
            .json::<GeminiResponse>()
            .await?;

        let mut chat_responses = Vec::new();
        for candidate in response.candidates {
            for part in candidate.content.parts {
                chat_responses.push(ChatResponseStream::AssistantResponseEvent {
                    content: part.text,
                });
            }
        }

        Ok(chat_responses)
    }
}

#[derive(Serialize)]
struct GeminiRequest {
    contents: Vec<Content>,
}

#[derive(Serialize)]
struct Content {
    role: String,
    parts: Vec<Part>,
}

impl From<ChatMessage> for Content {
    fn from(message: ChatMessage) -> Self {
        match message {
            ChatMessage::UserInputMessage(message) => Content {
                role: "user".to_string(),
                parts: vec![Part {
                    text: message.content,
                }],
            },
            ChatMessage::AssistantResponseMessage(message) => Content {
                role: "model".to_string(),
                parts: vec![Part {
                    text: message.content,
                }],
            },
        }
    }
}

impl From<crate::api_client::model::UserInputMessage> for Content {
    fn from(message: crate::api_client::model::UserInputMessage) -> Self {
        Content {
            role: "user".to_string(),
            parts: vec![Part {
                text: message.content,
            }],
        }
    }
}

#[derive(Serialize)]
struct Part {
    text: String,
}

#[derive(Deserialize)]
struct GeminiResponse {
    candidates: Vec<Candidate>,
}

#[derive(Deserialize)]
struct Candidate {
    content: ContentResponse,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ContentResponse {
    parts: Vec<PartResponse>,
    role: String,
}

#[derive(Deserialize)]
struct PartResponse {
    text: String,
}

