#[derive(Debug, thiserror::Error)]
pub enum PluginErrors {
    #[error("Lint for the {0} plugin failed")]
    PluginLintsFailed(String),
    #[error("Source file(s) not found with structure: {0}")]
    FilesNotFound(String),
    #[error("Error in the plugin: {0}")]
    PluginError(String),
}
