use extism::{convert::Json, Manifest, Plugin, Wasm};
use serde::{Deserialize, Serialize};
use simplelog::{error, warn};
use std::{collections::HashMap, path::PathBuf};

use crate::plugin_config::{CliPluginConfig, PluginData, PluginLocation};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct LintConfig {
    lints: Vec<LintData>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LintData {
    pub name: String,
    lint: String,
    #[serde(default)]
    args: HashMap<String, serde_json::Value>,
    pub warning: Option<String>,
    pub error: Option<String>,
    compared_lint: Option<String>,
    compared_args: Option<HashMap<String, serde_json::Value>>,
}

impl LintData {
    fn convert_to_plugin_lint(&self) -> Option<plugin_core::Lint> {
        Some(plugin_core::Lint {
            name: self.name.clone(),
            lint: self.lint.clone(),
            args: self.args.clone(),
            output: if self.warning.is_some() {
                plugin_core::LintResult::Warning(self.warning.clone().unwrap())
            } else {
                plugin_core::LintResult::Error(self.error.clone().unwrap())
            },
        })
    }

    pub fn convert_to_plugin_lint_with_compared(
        &self,
    ) -> Option<(plugin_core::Lint, plugin_core::Lint)> {
        Some((
            plugin_core::Lint {
                name: self.name.clone(),
                lint: self.lint.clone(),
                args: self.args.clone(),
                output: if self.warning.is_some() {
                    plugin_core::LintResult::Warning(self.warning.clone().unwrap())
                } else {
                    plugin_core::LintResult::Error(self.error.clone().unwrap())
                },
            },
            plugin_core::Lint {
                name: self.name.clone(),
                lint: self.compared_lint.clone()?,
                args: self.compared_args.clone().unwrap_or_default(),
                output: if self.warning.is_some() {
                    plugin_core::LintResult::Warning(self.warning.clone().unwrap())
                } else {
                    plugin_core::LintResult::Error(self.error.clone().unwrap())
                },
            },
        ))
    }

    fn validate(&self) -> anyhow::Result<()> {
        if self.name.is_empty() {
            return Err(anyhow::anyhow!("Lint name is empty"));
        }
        if self.lint.is_empty() {
            return Err(anyhow::anyhow!("Lint query is empty"));
        }
        if self.warning.is_none() && self.error.is_none() {
            return Err(anyhow::anyhow!(
                "Lint {} has no warning or error message",
                self.name
            ));
        }
        if self.warning.is_some() && self.error.is_some() {
            return Err(anyhow::anyhow!(
                "Lint {} has both warning and error message",
                self.name
            ));
        }
        Ok(())
    }

    pub fn validate_compared_lints(&self) -> anyhow::Result<()> {
        self.validate()?;
        if self.compared_lint.is_none() {
            return Err(anyhow::anyhow!(
                "The compared lint for {} doesn't exist",
                self.name
            ));
        }
        Ok(())
    }
}

pub fn wasm_main(config: PathBuf) -> anyhow::Result<()> {
    let args: CliPluginConfig = serde_yaml::from_str(std::fs::read_to_string(config)?.as_str())?;
    let plugin_data = PluginData::from_cli_plugin_config(args)?;
    let lints = merge_lints(&plugin_data)?;
    let wasm = match plugin_data.plugin {
        PluginLocation::Path(path) => Wasm::file(path),
        PluginLocation::Url(url) => Wasm::url(url),
    };
    let manifest = Manifest::new([wasm]).with_allowed_path(plugin_data.directory, "contents");
    let mut plugin = Plugin::new(manifest, [], true).unwrap();
    let res = plugin.call::<Option<&str>, ()>("new", None);
    if res.is_err() {
        eprintln!("Failed to initialize plugin: {:?}", res.err().unwrap());
        return Err(anyhow::anyhow!("Failed to initialize plugin"));
    }
    run_lints(lints, plugin)?;
    Ok(())
}

pub fn merge_lints(plugin: &PluginData) -> anyhow::Result<Vec<LintData>> {
    let mut lints = vec![];
    for lints_path in &plugin.lints_paths {
        let lints_file = std::fs::read_to_string(lints_path)?;
        let file_lints: LintConfig = serde_yaml::from_str(&lints_file)?;
        lints.extend(file_lints.lints);
    }

    for url in &plugin.urls {
        let lints_file = reqwest::blocking::get(url)?.text()?;
        let file_lints: LintConfig = serde_json::from_str(&lints_file)?;
        lints.extend(file_lints.lints);
    }
    let mut valid = true;
    for lint in &lints {
        if let Err(e) = lint.validate() {
            eprintln!("{}", e);
            valid = false;
        }
    }
    if !valid {
        return Err(anyhow::anyhow!("Lint config is not valid"));
    }
    Ok(lints)
}

fn run_lints(lints: Vec<LintData>, mut plugin: Plugin) -> anyhow::Result<()> {
    let mut passes = true;
    for lint in lints {
        if lint.compared_lint.is_some() {
            let (main_lint, compared_lint) = lint.convert_to_plugin_lint_with_compared().unwrap();
            let result =
                plugin.call::<Json<plugin_core::Lint>, String>("lint_single", Json(main_lint))?;
            let mut invalid_result: Vec<serde_json::Value> = Vec::new();
            let lint_results: Vec<serde_json::Value> = serde_json::from_str(&result)?;
            let result = plugin
                .call::<Json<plugin_core::Lint>, String>("lint_single", Json(compared_lint))?;
            let compared_lint_results: Vec<serde_json::Value> = serde_json::from_str(&result)?;
            for result in &lint_results {
                if !compared_lint_results.contains(result) {
                    invalid_result.push(result.clone());
                }
            }
            for result in compared_lint_results {
                if !lint_results.contains(&result) {
                    invalid_result.push(result.clone());
                }
            }
            if !invalid_result.is_empty() {
                match (lint.warning, lint.error) {
                    (None, Some(err)) => {
                        error!("{}", err);
                        passes = false;
                    }
                    (Some(warn), None) => {
                        warn!("{}", warn);
                    }
                    _ => unreachable!("Lint has both or no warning and error message"),
                }
                println!("{}", serde_json::to_string_pretty(&invalid_result).unwrap());
            }
        } else {
            let result = plugin.call::<Json<plugin_core::Lint>, String>(
                "lint_single",
                Json(lint.convert_to_plugin_lint().unwrap()),
            );
            match result {
                Ok(result) => {
                    let lint_results: Vec<serde_json::Value> = serde_json::from_str(&result)?;
                    if !lint_results.is_empty() {
                        match (lint.warning, lint.error) {
                            (None, Some(err)) => {
                                error!("{}", err);
                                passes = false;
                            }
                            (Some(warn), None) => {
                                warn!("{}", warn);
                            }
                            _ => unreachable!("Lint has both or no warning and error message"),
                        }
                        println!("{}", serde_json::to_string_pretty(&lint_results).unwrap());
                    }
                }
                Err(err) => {
                    error!("Error in the plugin running lint: {}", lint.name);
                    error!("{}", err);
                    passes = false;
                }
            }
        }
    }
    if !passes {
        return Err(anyhow::anyhow!("Linting failed"));
    }
    Ok(())
}
