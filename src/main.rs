use std::path::PathBuf;

use anyhow::Ok;
use clap::{Args, Parser};

mod compare_main;
mod plugin_config;
mod wasm_main;

use compare_main::compare_lints_main;

use serde::{Deserialize, Serialize};
use simplelog::{Color, ColorChoice, ConfigBuilder, Level, LevelFilter, TermLogger, TerminalMode};

use wasm_main::wasm_main;

#[derive(Debug, Parser, Serialize, Deserialize)]
pub struct CliArgs {
    /// Config files with lint queries
    #[clap(short, long)]
    pub config: Vec<PathBuf>,
    /// Folder containing terraform files
    #[clap(short, long)]
    pub terraform: PathBuf,
    /// Directory containing files that need to be linted
    #[clap(short, long)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<PathBuf>,
    /// Verbose mode
    #[clap(short, long)]
    pub verbose: bool,
}

#[derive(Debug, Parser)]
enum LinterCommands {
    Lint(CLI),
    Compare(CLI),
    // TODO
    // Test,
    // Output,
}

#[derive(Debug, Default, Args, Serialize, Deserialize)]
#[command(version, about, long_about = None)]
pub struct CLI {
    /// Verbose mode
    #[clap(short, long)]
    pub verbose: bool,
    /// Config file to use. Defaults to linter_config.yaml
    #[clap(short, long)]
    pub config: Option<PathBuf>,
    /// Directory containing files that need to be linted. Defaults to the current directory.
    #[clap(short, long)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<PathBuf>,
}

struct FinalCli {
    verbose: bool,
    config: PathBuf,
    dir: PathBuf,
}

impl FinalCli {
    fn new(cli: CLI) -> anyhow::Result<Self> {
        let config = cli
            .config
            .unwrap_or_else(|| PathBuf::from("linter_config.yaml"));
        let dir = cli.dir.unwrap_or_else(|| std::env::current_dir().unwrap());
        Ok(Self {
            verbose: cli.verbose,
            config,
            dir,
        })
    }

    fn validate(&self) -> anyhow::Result<()> {
        if !self.config.exists() {
            return Err(anyhow::anyhow!("Config file does not exist"));
        }
        if !self.dir.exists() {
            return Err(anyhow::anyhow!("Directory does not exist"));
        }
        Ok(())
    }
}

// impl CliArgs {
//     pub fn validate(&self) -> anyhow::Result<()> {
//         if !self.terraform.exists() {
//             return Err(anyhow::anyhow!("Terraform folder does not exist"));
//         }
//         if let Some(api) = &self.api {
//             if !api.exists() {
//                 return Err(anyhow::anyhow!("OpenAPI file does not exist"));
//             }
//         }
//         // if self.terraform.is_none() && self.api.is_none() {
//         //     return Err(anyhow::anyhow!("Either terraform or api file must be provided"));
//         // }
//         for config in &self.config {
//             if !config.exists() {
//                 return Err(anyhow::anyhow!("Config file does not exist"));
//             }
//         }
//         Ok(())
//     }
// }

// Based off article https://steezeburger.com/2023/03/rust-hierarchical-configuration/
// fn figment_layered_impl() -> anyhow::Result<CLI> {
//     let conf: CLI = Figment::new()
//         .merge(Yaml::file("linter.yaml"))
//         .merge(Env::prefixed("LINTER_"))
//         .merge(Serialized::defaults(CLI::parse()))
//         .extract()?;
//     Ok(conf)
// }

fn init_logger(verbose: bool) -> anyhow::Result<()> {
    let level = if verbose {
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
    Ok(())
}

fn main() -> anyhow::Result<()> {
    // let args = figment_layered_impl()?;
    // let args = FinalCli::new(args)?;
    // args.validate()?;
    let args = LinterCommands::parse();
    match args {
        LinterCommands::Lint(cli) => {
            init_logger(cli.verbose)?;
            let args = FinalCli::new(cli)?;
            args.validate()?;
            wasm_main(args.config)?;
        }
        LinterCommands::Compare(cli) => {
            init_logger(cli.verbose)?;
            let args = FinalCli::new(cli)?;
            args.validate()?;
            compare_lints_main(args.config)?;
        } // LinterCommands::Test => todo!(),
          // LinterCommands::Output => todo!(),
    }

    // lint_main(args)?;
    Ok(())
}

// fn lint_main(args: CliArgs) -> anyhow::Result<()> {
//     let merged_configs = merge_lint_files(&args.config)?;
//     info!("config is valid");

//     if merged_configs.has_api_lints() && args.api.is_none() {
//         warn!("config has api lints but no api file was provided");
//         std::process::exit(1);
//     }
//     if let Some(api) = &args.api {
//         lint_terraform_and_api(&args.terraform, api, &merged_configs.lints)?;
//         // let wasm = Wasm::file("./target/wasm32-wasi/debug/plugin-openapi.wasm");
//         // let manifest = Manifest::new([wasm]).with_allowed_path(api, "contents");
//         // let mut plugin = Plugin::new(&manifest, [], true).unwrap();
//         // let manifest = Manifest::new([wasm]);
//     } else {
//         lint_terraform(&args.terraform, &merged_configs.lints)?;
//     }
//     info!("All the tests passed");
//     Ok(())
// }

// /// Merge and validate the lint files
// fn merge_lint_files(configs: &Vec<PathBuf>) -> anyhow::Result<linter::config::model::Config> {
//     let mut merged_configs = linter::config::model::Config::default();
//     for config in configs {
//         let config = std::fs::read_to_string(config).expect("could not read config file");
//         let config: linter::config::model::Config =
//             serde_yaml::from_str(&config).expect("could not parse config file");
//         if let Err(e) = config.validate() {
//             warn!("{}", e);
//             return Err(anyhow::anyhow!(
//                 "Some of the lints do not have any lint attached to it"
//             ));
//         }
//         merged_configs.lints.extend(config.lints);
//     }
//     if let Err(e) = merged_configs.validate() {
//         warn!("{}", e);
//         return Err(anyhow::anyhow!(
//             "Some of the lints do not have any lint attached to it"
//         ));
//     }
//     Ok(merged_configs)
// }

// fn lint_terraform_and_api(tf: &PathBuf, api: &Path, lints: &Vec<Lint>) -> anyhow::Result<()> {
//     let hcl_adapter = Arc::new(HclAdapter::new(tf));
//     let hcl_schema = HclAdapter::schema();
//     // let oa_adapter = Arc::new(OpenApiAdapter::new(api.to_path_buf()));
//     // let oa_schema = OpenApiAdapter::schema();
//     let wasm = Wasm::file("./target/wasm32-wasi/debug/plugin_openapi.wasm");
//     let manifest = Manifest::new([wasm]).with_allowed_path(api, "contents");
//     let mut plugin = Plugin::new(manifest, [], true).unwrap();
//     let res = plugin.call::<Option<&str>, ()>("new", None);
//     if res.is_err() {
//         return Err(anyhow::anyhow!("Failed to initialize plugin"));
//     }

//     let mut passes = true;

//     for lint in lints {
//         if let (Some(terraform), Some(_openapi)) = (&lint.terraform, &lint.api) {
//             let variables: BTreeMap<Arc<str>, FieldValue> = if !lint.tf_args.is_empty() {
//                 let v = lint
//                     .tf_args
//                     .iter()
//                     .map(|(k, v)| (Arc::from(k.as_str()), from_json_value(v)))
//                     .collect();

//                 v
//             } else {
//                 BTreeMap::new()
//             };
//             let mut terraform_lint = Vec::new();
//             for data_item in execute_query(
//                 hcl_schema,
//                 hcl_adapter.clone(),
//                 terraform,
//                 variables.clone(),
//             )
//             .expect("not a legal query")
//             {
//                 let transparent: serde_json::Value = data_item
//                     .into_iter()
//                     .map(|(k, v)| (k.to_string(), from_field_value(&v)))
//                     .collect();
//                 terraform_lint.push(transparent);
//             }
//             let _variables: BTreeMap<Arc<str>, FieldValue> = if !lint.oa_args.is_empty() {
//                 let v = lint
//                     .oa_args
//                     .iter()
//                     .map(|(k, v)| (Arc::from(k.as_str()), from_json_value(v)))
//                     .collect();

//                 v
//             } else {
//                 BTreeMap::new()
//             };
//             // let mut openapi_lint = Vec::new();
//             // for data_item in
//             //     execute_query(oa_schema, oa_adapter.clone(), openapi, variables.clone())
//             //         .expect("not a legal query")
//             // {
//             //     let transparent: serde_json::Value = data_item
//             //         .into_iter()
//             //         .map(|(k, v)| (k.to_string(), from_field_value(&v)))
//             //         .collect();
//             //     openapi_lint.push(transparent);
//             // }
//             let result = plugin.call::<Json<plugin_core::Lint>, String>(
//                 "lint_single",
//                 Json(lint.convert_to_oai_lint().unwrap()),
//             )?;
//             let openapi_lint: Vec<serde_json::Value> = serde_json::from_str(&result)?;
//             let mut invalid_result: Vec<serde_json::Value> = Vec::new();

//             for result in &terraform_lint {
//                 if !openapi_lint.contains(result) {
//                     invalid_result.push(result.clone());
//                 }
//             }
//             for result in openapi_lint {
//                 if !terraform_lint.contains(&result) {
//                     invalid_result.push(result.clone());
//                 }
//             }
//             if !invalid_result.is_empty() {
//                 error!("Check failed: {}", lint.name);
//                 println!(
//                     "\nTerraform results {}",
//                     serde_json::to_string_pretty(&invalid_result).unwrap()
//                 );
//                 passes = false;
//             }
//         } else if let Some(terraform) = &lint.terraform {
//             let variables: BTreeMap<Arc<str>, FieldValue> = if !lint.tf_args.is_empty() {
//                 let v = lint
//                     .tf_args
//                     .iter()
//                     .map(|(k, v)| (Arc::from(k.as_str()), from_json_value(v)))
//                     .collect();

//                 v
//             } else {
//                 BTreeMap::new()
//             };
//             let mut terraform_lint = Vec::new();
//             for data_item in execute_query(
//                 hcl_schema,
//                 hcl_adapter.clone(),
//                 terraform,
//                 variables.clone(),
//             )
//             .expect("not a legal query")
//             {
//                 let transparent: serde_json::Value = data_item
//                     .into_iter()
//                     .map(|(k, v)| (k.to_string(), from_field_value(&v)))
//                     .collect();
//                 terraform_lint.push(transparent);
//             }
//             if !terraform_lint.is_empty() {
//                 error!("Check failed: {} {:#?}", lint.name, terraform_lint);
//                 passes = false;
//             }
//         } else if let Some(_api) = &lint.api {
//             // let variables: BTreeMap<Arc<str>, FieldValue> = if !lint.oa_args.is_empty() {
//             //     let v = lint
//             //         .oa_args
//             //         .iter()
//             //         .map(|(k, v)| (Arc::from(k.as_str()), from_json_value(v)))
//             //         .collect();

//             //     v
//             // } else {
//             //     BTreeMap::new()
//             // };
//             // let mut api_lint = Vec::new();
//             // for data_item in execute_query(oa_schema, oa_adapter.clone(), api, variables.clone())
//             //     .expect("not a legal query")
//             // {
//             //     let transparent: serde_json::Value = data_item
//             //         .into_iter()
//             //         .map(|(k, v)| (k.to_string(), from_field_value(&v)))
//             //         .collect();
//             //     api_lint.push(transparent);
//             // }
//             let result = plugin.call::<Json<plugin_core::Lint>, String>(
//                 "lint_single",
//                 Json(lint.convert_to_oai_lint().unwrap()),
//             )?;
//             let api_lint: Vec<serde_json::Value> = serde_json::from_str(&result)?;
//             if !api_lint.is_empty() {
//                 error!("Check failed: {} {:#?}", lint.name, api_lint);
//                 passes = false;
//             }
//         } else {
//             unreachable!();
//         }
//     }
//     if passes {
//         Ok(())
//     } else {
//         Err(anyhow::anyhow!("Linting tests failed"))
//     }
// }

// fn lint_terraform(tf: &PathBuf, lints: &Vec<Lint>) -> anyhow::Result<()> {
//     let mut passes = true;
//     for lint in lints {
//         if let Some(terraform) = &lint.terraform {
//             let hcl_adapter = Arc::new(HclAdapter::new(tf));
//             let hcl_schema = HclAdapter::schema();
//             let variables: BTreeMap<Arc<str>, FieldValue> = std::collections::BTreeMap::new();
//             let mut terraform_lint = Vec::new();
//             for data_item in execute_query(
//                 hcl_schema,
//                 hcl_adapter.clone(),
//                 terraform,
//                 variables.clone(),
//             )
//             .expect("not a legal query")
//             {
//                 let transparent: serde_json::Value = data_item
//                     .into_iter()
//                     .map(|(k, v)| (k.to_string(), from_field_value(&v)))
//                     .collect();
//                 terraform_lint.push(transparent);
//             }
//             if !terraform_lint.is_empty() {
//                 error!("Check failed: {} {:#?}", lint.name, terraform_lint);
//                 passes = false;
//             }
//         }
//     }
//     if passes {
//         Ok(())
//     } else {
//         Err(anyhow::anyhow!("Linting tests failed"))
//     }
// }
