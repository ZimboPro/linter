use std::{collections::BTreeMap, path::PathBuf, sync::Arc};

use serde::{Deserialize, Serialize};
use simplelog::error;
use trustfall::FieldValue;

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lint {
    pub name: String,
    pub description: String,
    pub terraform: Option<String>,
    pub api: Option<String>,
    pub error: String,
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
                description: "test".to_string(),
                terraform: Some(".".into()),
                api: None,
                error: "test".to_string(),
            }],
        };
        assert_eq!(config.validate().is_ok(), true);
    }

    #[test]
    fn test_validate_no_terraform_or_api() {
        let config = Config {
            lints: vec![Lint {
                name: "test".to_string(),
                description: "test".to_string(),
                terraform: None,
                api: None,
                error: "test".to_string(),
            }],
        };
        assert_eq!(config.validate().is_err(), true);
    }

    #[test]
    fn test_has_api_lints() {
        let config = Config {
            lints: vec![Lint {
                name: "test".to_string(),
                description: "test".to_string(),
                terraform: None,
                api: Some(".".into()),
                error: "test".to_string(),
            }],
        };
        assert_eq!(config.has_api_lints(), true);
    }

    #[test]
    fn test_has_no_api_lints() {
        let config = Config {
            lints: vec![Lint {
                name: "test".to_string(),
                description: "test".to_string(),
                terraform: Some(".".into()),
                api: None,
                error: "test".to_string(),
            }],
        };
        assert_eq!(config.has_api_lints(), false);
    }
}
