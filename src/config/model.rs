use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use simplelog::error;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    pub lints: Vec<Lint>,
}

impl Config {
    pub fn validate(&self) -> Result<(), String> {
        let mut valid = true;
        for lint in &self.lints {
            if let Err(e) = lint.validate() {
                error!("{}", e);
                valid = false;
            }
        }
        if !valid {
            return Err("Lint config is not valid".to_string());
        }
        Ok(())
    }

    pub fn has_api_lints(&self) -> bool {
        self.lints.iter().any(|l| l.api.is_some())
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Lint {
    pub name: String,
    pub terraform: Option<String>,
    pub api: Option<String>,
    pub error: String,
    #[serde(default)]
    pub tf_args: HashMap<String, serde_json::Value>,
    #[serde(default)]
    pub oa_args: HashMap<String, serde_json::Value>,
}

impl Lint {
    pub fn validate(&self) -> Result<(), String> {
        if self.terraform.is_none() && self.api.is_none() {
            return Err(format!("Lint {} has no terraform or api query", self.name));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Check {
    IsEmpty,
    IsEqual,
    AtLeastOne,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate() {
        let config = Config {
            lints: vec![Lint {
                name: "test".to_string(),
                terraform: Some(".".into()),
                api: None,
                error: "test".to_string(),
                tf_args: HashMap::new(),
                oa_args: HashMap::new(),
            }],
        };
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_validate_no_terraform_or_api() {
        let config = Config {
            lints: vec![Lint {
                name: "test".to_string(),
                terraform: None,
                api: None,
                error: "test".to_string(),
                tf_args: HashMap::new(),
                oa_args: HashMap::new(),
            }],
        };
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_has_api_lints() {
        let config = Config {
            lints: vec![Lint {
                name: "test".to_string(),
                terraform: None,
                api: Some(".".into()),
                error: "test".to_string(),
                tf_args: HashMap::new(),
                oa_args: HashMap::new(),
            }],
        };
        assert!(config.has_api_lints());
    }

    #[test]
    fn test_has_no_api_lints() {
        let config = Config {
            lints: vec![Lint {
                name: "test".to_string(),
                terraform: Some(".".into()),
                api: None,
                error: "test".to_string(),
                tf_args: HashMap::new(),
                oa_args: HashMap::new(),
            }],
        };
        assert!(!config.has_api_lints());
    }
}
