mod client;
pub mod error;
pub mod models;

pub use crate::models::chat::{
    ChatChoice, ChatCompletionRequest, ChatCompletionResponse, ChatMessage,
};
pub use client::OpenAIClient;
pub use error::Error;
