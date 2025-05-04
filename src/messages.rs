use crate::models::ModelInfo;
use crate::tool_choice::ToolChoice;
use mcp_protocol::tool::{Tool, ToolContent};
use serde::{Deserialize, Serialize};

/// Different types of content that can be in a message
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum MessageContent {
    Text {
        text: String,
    },

    ToolUse {
        id: String,
        name: String,
        input: serde_json::Value,
    },

    ToolResult {
        tool_use_id: String,
        content: Vec<ToolContent>,
        is_error: Option<bool>,
    },
}

/// A single message in a conversation with Claude
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    /// Role of the message sender (user, assistant, system)
    pub role: Role,

    /// Content of the message as vector of MessageContent objects
    pub content: Vec<MessageContent>,
}

/// Role of the message sender
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Role {
    #[serde(rename = "user")]
    User,

    #[serde(rename = "assistant")]
    Assistant,

    #[serde(rename = "system")]
    System,
}

impl Message {
    /// Create a new message with structured content
    pub fn new_structured(role: impl Into<String>, content: Vec<MessageContent>) -> Self {
        Self {
            role: match role.into().as_str() {
                "user" => Role::User,
                "assistant" => Role::Assistant,
                "system" => Role::System,
                _ => panic!("Invalid role"),
            },
            content,
        }
    }
}

/// Request to generate a completion from Claude
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CompletionRequest {
    /// The model to use
    pub model: String,

    /// List of messages in the conversation
    pub messages: Vec<Message>,

    /// Maximum number of tokens to generate
    pub max_tokens: u32,

    /// Temperature parameter (0.0 to 1.0)
    pub temperature: Option<f32>,

    /// System prompt to use
    pub system: Option<String>,

    /// Tools to make available to the model
    pub tools: Option<Vec<Tool>>,

    /// Tool choice configuration
    pub tool_choice: Option<ToolChoice>,

    /// Whether to disable parallel tool use
    pub disable_parallel_tool_use: Option<bool>,
}

/// Information about token usage
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Usage {
    pub input_tokens: u32,

    pub output_tokens: u32,
}

/// Response from a completion request
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CompletionResponse {
    /// Generated content blocks
    pub content: Vec<MessageContent>,

    /// ID of the message
    pub id: String,

    /// Model used for generation
    pub model: String,

    // always "assistant"
    pub role: Role,

    pub stop_reason: StopReason,

    pub stop_sequence: Option<String>,

    /// Message type
    #[serde(rename = "type")]
    pub message_type: String,

    /// Token usage information
    pub usage: Usage,
}

/// Reason why generation stopped
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum StopReason {
    /// Generation stopped because the end of a turn was reached
    #[serde(rename = "end_turn")]
    EndTurn,

    /// Generation stopped because the maximum token limit was reached
    #[serde(rename = "max_tokens")]
    MaxTokens,

    /// Generation stopped because a stop sequence was encountered
    #[serde(rename = "stop_sequence")]
    StopSequence,

    /// Generation stopped because a tool was used
    #[serde(rename = "tool_use")]
    ToolUse,

    /// Other reason
    #[serde(rename = "other")]
    Other(String),
}

/// Request format for the anthropic-proxy actor
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ProxyRequest {
    ListModels,

    GenerateCompletion { request: CompletionRequest },
}

/// Response format from the anthropic-proxy actor
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ProxyResponse {
    /// List of available models
    ListModels { models: Vec<ModelInfo> },

    /// Generated completion
    Completion { completion: CompletionResponse },

    /// Error response
    Error { error: String },
}
