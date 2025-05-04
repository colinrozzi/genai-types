// GenAI API Types Library meant for use by actors in the Theater

pub mod errors;
pub mod messages;
pub mod models;
pub mod tool_choice;

// Re-export main types for convenience
pub use errors::GenAiError;
pub use messages::{
    CompletionRequest, CompletionResponse, Message, MessageContent, ProxyRequest, ProxyResponse,
    Usage,
};
pub use models::{ModelInfo, ModelPricing};
pub use tool_choice::ToolChoice;
