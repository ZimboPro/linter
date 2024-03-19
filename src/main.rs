use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
    sync::Arc,
};

use anyhow::Ok;
use clap::Parser;
use linter::{
    config::model::Lint,
    hcl::adapter::HclAdapter,
    openapi::adapter::OpenApiAdapter,
    util::{from_field_value, from_json_value},
};

use simplelog::{
    error, info, warn, Color, ColorChoice, ConfigBuilder, Level, LevelFilter, TermLogger,
    TerminalMode,
};
use trustfall::{execute_query, FieldValue};

#[derive(Debug, Parser)]
pub struct Args {
    /// Config files with lint queries
    #[clap(short, long)]
    pub config: Vec<PathBuf>,
    /// Folder containing terraform files
    #[clap(short, long)]
    pub terraform: PathBuf,
    /// OpenAPI file or folder containing OpenAPI files
    #[clap(short, long)]
    pub api: Option<PathBuf>,
    /// Verbose mode
    #[clap(short, long)]
    pub verbose: bool,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let level = if args.verbose {
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    };
    let config = ConfigBuilder::new()
        .set_level_color(Level::Debug, Some(Color::Cyan))
        .set_level_color(Level::Info, Some(Color::Blue))
        .set_level_color(Level::Warn, Some(Color::Yellow))
        .set_level_color(Level::Error, Some(Color::Magenta))
        .set_level_color(Level::Trace, Some(Color::Green))
        .set_time_level(LevelFilter::Off)
        .build();
    TermLogger::init(level, config, TerminalMode::Stdout, ColorChoice::Auto).unwrap();
    lint_main(args)?;
    Ok(())
}

fn lint_main(args: Args) -> anyhow::Result<()> {
    let merged_configs = merge_lint_files(&args.config)?;
    info!("config is valid");

    if merged_configs.has_api_lints() && args.api.is_none() {
        warn!("config has api lints but no api file was provided");
        std::process::exit(1);
    }
    if let Some(api) = &args.api {
        lint_terraform_and_api(&args.terraform, api, &merged_configs.lints)?;
    } else {
        lint_terraform(&args.terraform, &merged_configs.lints)?;
    }
    info!("All the tests passed");
    Ok(())
}

/// Merge and validate the lint files
fn merge_lint_files(configs: &Vec<PathBuf>) -> anyhow::Result<linter::config::model::Config> {
    let mut merged_configs = linter::config::model::Config::default();
    for config in configs {
        let config = std::fs::read_to_string(config).expect("could not read config file");
        let config: linter::config::model::Config =
            serde_yaml::from_str(&config).expect("could not parse config file");
        if let Err(e) = config.validate() {
            warn!("{}", e);
            return Err(anyhow::anyhow!(
                "Some of the lints do not have any lint attached to it"
            ));
        }
        merged_configs.lints.extend(config.lints);
    }
    if let Err(e) = merged_configs.validate() {
        warn!("{}", e);
        return Err(anyhow::anyhow!(
            "Some of the lints do not have any lint attached to it"
        ));
    }
    Ok(merged_configs)
}

fn lint_terraform_and_api(tf: &PathBuf, api: &Path, lints: &Vec<Lint>) -> anyhow::Result<()> {
    let hcl_adapter = Arc::new(HclAdapter::new(tf));
    let hcl_schema = HclAdapter::schema();
    let oa_adapter = Arc::new(OpenApiAdapter::new(api.to_path_buf()));
    let oa_schema = OpenApiAdapter::schema();

    let mut passes = true;

    for lint in lints {
        if let (Some(terraform), Some(openapi)) = (&lint.terraform, &lint.api) {
            let variables: BTreeMap<Arc<str>, FieldValue> = if !lint.tf_args.is_empty() {
                let v = lint
                    .tf_args
                    .iter()
                    .map(|(k, v)| (Arc::from(k.as_str()), from_json_value(v)))
                    .collect();

                v
            } else {
                BTreeMap::new()
            };
            let mut terraform_lint = Vec::new();
            for data_item in execute_query(
                hcl_schema,
                hcl_adapter.clone(),
                terraform,
                variables.clone(),
            )
            .expect("not a legal query")
            {
                let transparent: serde_json::Value = data_item
                    .into_iter()
                    .map(|(k, v)| (k.to_string(), from_field_value(&v)))
                    .collect();
                terraform_lint.push(transparent);
            }
            let variables: BTreeMap<Arc<str>, FieldValue> = if !lint.oa_args.is_empty() {
                let v = lint
                    .oa_args
                    .iter()
                    .map(|(k, v)| (Arc::from(k.as_str()), from_json_value(v)))
                    .collect();

                v
            } else {
                BTreeMap::new()
            };
            let mut openapi_lint = Vec::new();
            for data_item in
                execute_query(oa_schema, oa_adapter.clone(), openapi, variables.clone())
                    .expect("not a legal query")
            {
                let transparent: serde_json::Value = data_item
                    .into_iter()
                    .map(|(k, v)| (k.to_string(), from_field_value(&v)))
                    .collect();
                openapi_lint.push(transparent);
            }
            let tf = terraform_lint
                .iter()
                .all(|item| openapi_lint.contains(item));
            let oa = openapi_lint
                .iter()
                .all(|item| terraform_lint.contains(item));
            if !tf || !oa {
                error!("Check failed: {}", lint.name);
                println!(
                    "\nTerraform results {}",
                    serde_json::to_string_pretty(&terraform_lint).unwrap()
                );
                println!(
                    "\nOpenAPI results {}",
                    serde_json::to_string_pretty(&openapi_lint).unwrap()
                );
                passes = false;
            }
        } else if let Some(terraform) = &lint.terraform {
            let variables: BTreeMap<Arc<str>, FieldValue> = if !lint.tf_args.is_empty() {
                let v = lint
                    .tf_args
                    .iter()
                    .map(|(k, v)| (Arc::from(k.as_str()), from_json_value(v)))
                    .collect();

                v
            } else {
                BTreeMap::new()
            };
            let mut terraform_lint = Vec::new();
            for data_item in execute_query(
                hcl_schema,
                hcl_adapter.clone(),
                terraform,
                variables.clone(),
            )
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
        } else if let Some(api) = &lint.api {
            let variables: BTreeMap<Arc<str>, FieldValue> = if !lint.oa_args.is_empty() {
                let v = lint
                    .oa_args
                    .iter()
                    .map(|(k, v)| (Arc::from(k.as_str()), from_json_value(v)))
                    .collect();

                v
            } else {
                BTreeMap::new()
            };
            let mut api_lint = Vec::new();
            for data_item in execute_query(oa_schema, oa_adapter.clone(), api, variables.clone())
                .expect("not a legal query")
            {
                let transparent: serde_json::Value = data_item
                    .into_iter()
                    .map(|(k, v)| (k.to_string(), from_field_value(&v)))
                    .collect();
                api_lint.push(transparent);
            }
            if !api_lint.is_empty() {
                error!("Check failed: {} {:#?}", lint.name, api_lint);
                passes = false;
            }
        } else {
            unreachable!();
        }
    }
    if passes {
        Ok(())
    } else {
        Err(anyhow::anyhow!("Linting tests failed"))
    }
}

fn lint_terraform(tf: &PathBuf, lints: &Vec<Lint>) -> anyhow::Result<()> {
    let mut passes = true;
    for lint in lints {
        if let Some(terraform) = &lint.terraform {
            let hcl_adapter = Arc::new(HclAdapter::new(tf));
            let hcl_schema = HclAdapter::schema();
            let variables: BTreeMap<Arc<str>, FieldValue> = std::collections::BTreeMap::new();
            let mut terraform_lint = Vec::new();
            for data_item in execute_query(
                hcl_schema,
                hcl_adapter.clone(),
                terraform,
                variables.clone(),
            )
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
    }
    if passes {
        Ok(())
    } else {
        Err(anyhow::anyhow!("Linting tests failed"))
    }
}
