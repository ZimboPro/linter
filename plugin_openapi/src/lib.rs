use extism_pdk::*;
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

#[plugin_fn]
pub fn new() -> FnResult<()> {
    Ok(())
}
