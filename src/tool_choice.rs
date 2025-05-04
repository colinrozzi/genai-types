use serde::{Deserialize, Serialize};

/// Tool choice configuration
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum ToolChoice {
    /// Model decides whether to use tools
    #[serde(rename = "auto")]
    Auto,

    /// Force model to use a specific tool
    #[serde(rename = "tool")]
    Tool {
        /// Name of the tool to use
        name: String,
    },

    /// Force model to use any available tool
    #[serde(rename = "any")]
    Any,

    /// Force model not to use tools
    #[serde(rename = "none")]
    None,
}
