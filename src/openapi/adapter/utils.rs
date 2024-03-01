use merge_yaml_hash::MergeYamlHash;
use simplelog::{debug, error, info, warn};
use std::{ffi::OsStr, io::Read, path::PathBuf};

/// Finds all the files with the extension in the directory recursively
pub fn find_files(path: &std::path::Path, extension: &OsStr) -> Vec<PathBuf> {
    debug!("Finding files in {:?}", path);
    let mut files = Vec::new();
    for entry in path.read_dir().expect("Failed to read directory").flatten() {
        if entry.path().is_dir() {
            debug!("Found directory {:?}", entry.path());
            files.append(&mut find_files(&entry.path(), extension));
        } else if entry.path().extension() == Some(extension) {
            debug!("Found file {:?}", entry.path());
            files.push(entry.path());
        }
    }
    files
}

/// Gets a file's contents
pub fn open_file(filename: PathBuf) -> String {
    let mut file = std::fs::File::open(filename).expect("Couldn't find or open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Couldn't read the contents of the file");
    contents
}

pub fn merge(files: Vec<String>) -> String {
    let mut hash = MergeYamlHash::new();
    debug!("Merging OpenAPI documents");
    for file in files {
        debug!("Merging file {:?}", file);
        hash.merge(&file);
    }

    hash.to_string()
}

pub struct Operator {
    pub method: String,
    pub summary: Option<String>,
    pub description: Option<String>,
    pub tags: Vec<String>,
    pub aws: Option<AmazonApigatewayIntegration>,
}

pub struct AmazonApigatewayIntegration {
    r_type: String,
    http_method: String,
    uri: String,
    pass_through_behavior: String,
    timeout_in_millis: usize,
    trigger: String,
    arn: String,
}
