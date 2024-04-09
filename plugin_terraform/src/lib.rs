mod adapter_impl;
mod edges;
mod properties;
mod vertex;

pub mod model;
#[cfg(test)]
mod tests;
pub mod utils;

use std::{
    path::Path,
    sync::{Arc, OnceLock},
};

pub use adapter_impl::HclAdapter;
use extism_pdk::{error, plugin_fn, FnResult, Json};
use plugin_core::{convert_to_args, from_field_value, Lint, PluginErrors};
use trustfall::execute_query;
pub use vertex::Vertex;

static ADAPTER: OnceLock<Arc<HclAdapter>> = OnceLock::new();

#[plugin_fn]
pub fn new() -> FnResult<()> {
    ADAPTER.get_or_init(|| Arc::new(HclAdapter::new(Path::new("contents"))));

    Ok(())
}

#[plugin_fn]
pub fn lint_all(Json(lints): Json<Vec<Lint>>) -> FnResult<()> {
    let adapter = ADAPTER.get().expect("adapter not initialized").clone();
    let mut passes = true;
    let schema = HclAdapter::schema();
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
        Err(PluginErrors::PluginLintsFailed("Terraform".to_string()).into())
    }
}

#[plugin_fn]
pub fn lint_single(Json(lint): Json<Lint>) -> FnResult<String> {
    let adapter = ADAPTER.get().expect("adapter not initialized").clone();
    let schema = HclAdapter::schema();
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
