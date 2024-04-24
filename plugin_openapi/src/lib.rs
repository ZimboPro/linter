use std::sync::{Arc, OnceLock};
use trustfall::{execute_query, Schema};

use extism_pdk::*;
use plugin_core::{convert_to_args, from_field_value, Lint, PluginErrors};
use serde::{Deserialize, Serialize};

mod adapter_impl;
mod edges;
mod properties;
mod utils;
mod vertex;

#[cfg(test)]
mod tests;

pub use adapter_impl::OpenApiAdapter;
pub use vertex::Vertex;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AvailableFiles {
    pub files: Vec<FilteredFile>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct FilteredFile {
    pub name: String,
    pub path: std::path::PathBuf,
    pub contents: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Files {
    extensions: Vec<String>,
    ignored_dirs: Vec<String>,
}

#[plugin_fn]
pub fn requested_files() -> FnResult<Json<Files>> {
    let files = Files {
        extensions: vec![String::from("yml"), String::from("yaml")],
        ..Default::default()
    };
    Ok(Json(files))
}

static ADAPTER: OnceLock<Arc<OpenApiAdapter>> = OnceLock::new();

#[plugin_fn]
pub fn new() -> FnResult<()> {
    let s = OpenApiAdapter::new()?;
    ADAPTER.get_or_init(|| Arc::new(s));

    Ok(())
}

#[plugin_fn]
pub fn lint_all(Json(lints): Json<Vec<Lint>>) -> FnResult<()> {
    let adapter = ADAPTER.get().expect("adapter not initialized").clone();
    let mut passes = true;
    let schema = OpenApiAdapter::schema();
    for lint in lints {
        match run_lint(lint.clone(), &adapter, schema) {
            Ok(terraform_lint) => {
                if !terraform_lint.is_empty() {
                    match lint.output {
                        plugin_core::LintResult::Warning(ref message) => {
                            warn!("{}", message);
                        }
                        plugin_core::LintResult::Error(ref message) => {
                            error!("{}", message);
                            passes = false;
                        }
                    }
                    println!("{}", serde_json::to_string_pretty(&terraform_lint).unwrap());
                }
            }
            Err(e) => {
                error!("Error in the plugin running lint: {}", e);
                return Err(e.into());
            }
        }
    }
    if passes {
        Ok(())
    } else {
        Err(PluginErrors::PluginLintsFailed("OpenAPI".to_string()).into())
    }
}

fn run_lint(
    lint: Lint,
    adapter: &Arc<OpenApiAdapter>,
    schema: &Schema,
) -> Result<Vec<serde_json::Value>, PluginErrors> {
    let variables = convert_to_args(lint.args);
    let mut lint_results = Vec::new();
    let iter_result = execute_query(schema, adapter.clone().to_owned(), &lint.lint, variables);
    match iter_result {
        Ok(iter) => {
            for data_item in iter {
                let transparent: serde_json::Value = data_item
                    .into_iter()
                    .map(|(k, v)| (k.to_string(), from_field_value(&v)))
                    .collect();
                lint_results.push(transparent);
            }
            Ok(lint_results)
        }
        Err(e) => {
            error!("Error in the plugin running lint: {}", lint.name);
            Err(PluginErrors::PluginError(e.to_string()))
        }
    }
}

#[plugin_fn]
pub fn lint_single(Json(lint): Json<Lint>) -> FnResult<String> {
    let adapter = ADAPTER.get().expect("adapter not initialized").clone();
    let schema = OpenApiAdapter::schema();

    match run_lint(lint, &adapter, schema) {
        Ok(lint_results) => Ok(serde_json::to_string(&lint_results)?),
        Err(e) => Err(e.into()),
    }
}
