use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CliPluginConfig {
    /// Path to the plugin.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// Url to the plugin.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Directory containing the files to be linted. Defaults to the current directory.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory: Option<PathBuf>,
    /// Paths to the lints files.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lints_paths: Option<Vec<PathBuf>>,
    /// Urls to the lints files.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub urls: Option<Vec<String>>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ComparePluginConfig {
    pub main: CliComparePluginConfig,
    pub secondary: CliComparePluginConfig,
    /// Paths to the lints files.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lints_paths: Option<Vec<PathBuf>>,
    /// Urls to the lints files.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub urls: Option<Vec<String>>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CliComparePluginConfig {
    /// Path to the plugin.
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<String>,
    /// Url to the plugin.
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<String>,
    /// Directory containing the files to be linted. Defaults to the current directory.
    #[serde(skip_serializing_if = "Option::is_none")]
    directory: Option<PathBuf>,
}

pub enum PluginLocation {
    Path(PathBuf),
    Url(String),
}

pub struct PluginData {
    pub plugin: PluginLocation,
    pub directory: PathBuf,
    pub lints_paths: Vec<PathBuf>,
    pub urls: Vec<String>,
}

impl PluginData {
    fn new(
        plugin: PluginLocation,
        directory: PathBuf,
        lints_paths: Vec<PathBuf>,
        urls: Vec<String>,
    ) -> Self {
        Self {
            plugin,
            directory,
            lints_paths,
            urls,
        }
    }

    pub fn from_cli_plugin_config(cli_plugin_config: CliPluginConfig) -> anyhow::Result<Self> {
        let plugin = if let Some(path) = cli_plugin_config.path {
            PluginLocation::Path(PathBuf::from(path))
        } else if let Some(url) = cli_plugin_config.url {
            PluginLocation::Url(url)
        } else {
            return Err(anyhow::anyhow!("No plugin path or url provided"));
        };

        Ok(Self::new(
            plugin,
            cli_plugin_config
                .directory
                .unwrap_or(std::env::current_dir().expect("Failed to get current dir")),
            cli_plugin_config.lints_paths.unwrap_or_default(),
            cli_plugin_config.urls.unwrap_or_default(),
        ))
    }

    pub fn from_cli_compare_plugin_config(
        cli_plugin_config: CliComparePluginConfig,
    ) -> anyhow::Result<Self> {
        let plugin = if let Some(path) = cli_plugin_config.path {
            PluginLocation::Path(PathBuf::from(path))
        } else if let Some(url) = cli_plugin_config.url {
            PluginLocation::Url(url)
        } else {
            return Err(anyhow::anyhow!("No plugin path or url provided"));
        };

        Ok(Self::new(
            plugin,
            cli_plugin_config
                .directory
                .unwrap_or(std::env::current_dir().expect("Failed to get current dir")),
            Default::default(),
            Default::default(),
        ))
    }
}

struct ComparePluginData {
    main: PluginData,
    secondary: PluginData,
}

impl ComparePluginData {
    pub fn from_compare_cli_plugin_config(config: ComparePluginConfig) -> anyhow::Result<Self> {
        let main: PluginData = PluginData::from_cli_compare_plugin_config(config.main)?;
        let secondary: PluginData = PluginData::from_cli_compare_plugin_config(config.secondary)?;

        Ok(Self { main, secondary })
    }
}
