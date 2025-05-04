use serde::{Deserialize, Serialize};

/// Information about a model
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ModelInfo {
    /// Model ID
    pub id: String,

    /// Display name
    pub display_name: String,

    /// Maximum context window size
    pub max_tokens: u32,

    /// Provider name
    pub provider: String,

    /// Optional pricing information
    pub pricing: Option<ModelPricing>,
}

/// Pricing information for a model
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ModelPricing {
    /// Cost per million input tokens
    pub input_cost_per_million_tokens: f64,

    /// Cost per million output tokens
    pub output_cost_per_million_tokens: f64,
}
