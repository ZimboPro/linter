use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Lint {
    pub name: String,
    pub lint: String,
    #[serde(default)]
    pub args: HashMap<String, serde_json::Value>,
}
