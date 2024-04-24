use std::path::PathBuf;

use extism::{convert::Json, Manifest, Plugin, Wasm};
use simplelog::{error, warn};

use crate::{
    plugin_config::{ComparePluginConfig, PluginData, PluginLocation},
    wasm_main::merge_lints,
};

pub fn compare_lints_main(config: PathBuf) -> anyhow::Result<()> {
    let args: ComparePluginConfig =
        serde_yaml::from_str(std::fs::read_to_string(config)?.as_str())?;
    let plugin_data = PluginData::from_cli_compare_plugin_config(args.main)?;
    let lints = merge_lints(&plugin_data)?;
    let wasm = match plugin_data.plugin {
        PluginLocation::Path(path) => Wasm::file(path),
        PluginLocation::Url(url) => Wasm::url(url),
    };
    let manifest = Manifest::new([wasm]).with_allowed_path(plugin_data.directory, "contents");
    let mut main_plugin = Plugin::new(manifest, [], true).unwrap();
    let res = main_plugin.call::<Option<&str>, ()>("new", None);
    if res.is_err() {
        return Err(anyhow::anyhow!("Failed to initialize plugin"));
    }
    let plugin_data = PluginData::from_cli_compare_plugin_config(args.secondary)?;
    let wasm = match plugin_data.plugin {
        PluginLocation::Path(path) => Wasm::file(path),
        PluginLocation::Url(url) => Wasm::url(url),
    };
    let manifest = Manifest::new([wasm]).with_allowed_path(plugin_data.directory, "contents");
    let mut secondary_plugin = Plugin::new(manifest, [], true).unwrap();
    let res = secondary_plugin.call::<Option<&str>, ()>("new", None);
    if res.is_err() {
        return Err(anyhow::anyhow!("Failed to initialize plugin"));
    }
    for lint in &lints {
        lint.validate_compared_lints()?;
    }
    let mut passes = true;
    for lint in lints {
        let l = lint.convert_to_plugin_lint_with_compared().unwrap();
        let result =
            main_plugin.call::<Json<plugin_core::Lint>, String>("lint_single", Json(l.0))?;
        let main_results: Vec<serde_json::Value> = serde_json::from_str(&result)?;
        let result =
            secondary_plugin.call::<Json<plugin_core::Lint>, String>("lint_single", Json(l.1))?;
        let secondary_results: Vec<serde_json::Value> = serde_json::from_str(&result)?;

        let mut invalid_result: Vec<serde_json::Value> = Vec::new();
        for result in &main_results {
            if !secondary_results.contains(result) {
                invalid_result.push(result.clone());
            }
        }
        for result in secondary_results {
            if !main_results.contains(&result) {
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
    }
    if !passes {
        return Err(anyhow::anyhow!("Linting failed"));
    }
    Ok(())
}
