use std::sync::{Arc, OnceLock};
use trustfall::execute_query;

use extism_pdk::*;
use plugin_core::{convert_to_args, from_field_value, Lint, PluginErrors};
use serde::{Deserialize, Serialize};

mod adapter_impl;
mod edges;
mod entrypoints;
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
    ADAPTER.get_or_init(|| Arc::new(OpenApiAdapter::new()));

    Ok(())
}

#[plugin_fn]
pub fn lint_all(Json(lints): Json<Vec<Lint>>) -> FnResult<()> {
    let adapter = ADAPTER.get().expect("adapter not initialized").clone();
    let mut passes = true;
    let schema = OpenApiAdapter::schema();
    for lint in lints {
        let variables = convert_to_args(lint.args);

        let mut terraform_lint = Vec::new();
        for data_item in execute_query(schema, adapter.clone().to_owned(), &lint.lint, variables)
            .expect("not a legal query")
        {
            let transparent: serde_json::Value = data_item
                .into_iter()
                .map(|(k, v)| (k.to_string(), from_field_value(&v)))
                .collect();
            terraform_lint.push(transparent);
        }
        if !terraform_lint.is_empty() {
            error!("Check failed: {} {:#?}", lint.name, terraform_lint);
            passes = false;
        }
    }
    if passes {
        Ok(())
    } else {
        Err(PluginErrors::PluginLintsFailed("OpenAPI".to_string()).into())
    }
}

#[plugin_fn]
pub fn lint_single(Json(lint): Json<Lint>) -> FnResult<String> {
    let adapter = ADAPTER.get().expect("adapter not initialized").clone();
    let schema = OpenApiAdapter::schema();
    let variables = convert_to_args(lint.args);
    let mut lint_results = Vec::new();
    for data_item in execute_query(schema, adapter.clone().to_owned(), &lint.lint, variables)
        .expect("not a legal query")
    {
        let transparent: serde_json::Value = data_item
            .into_iter()
            .map(|(k, v)| (k.to_string(), from_field_value(&v)))
            .collect();
        lint_results.push(transparent);
    }
    Ok(serde_json::to_string(&lint_results)?)
}
