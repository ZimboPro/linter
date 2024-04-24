use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Lint {
    pub name: String,
    pub lint: String,
    pub output: LintResult,
    #[serde(default)]
    pub args: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LintResult {
    Warning(String),
    Error(String),
}

impl Default for LintResult {
    fn default() -> Self {
        LintResult::Error("No output".to_string())
    }
}
