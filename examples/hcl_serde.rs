use std::{ffi::OsStr, path::PathBuf};

fn find_files(path: PathBuf) -> Vec<PathBuf> {
    let mut files = Vec::new();
    for entries in path.read_dir().expect("Failed to get dir contents") {
        if let Ok(entry) = entries {
            if entry.path().is_dir() {
                files.extend(find_files(entry.path()));
            } else if entry.path().is_file() && entry.path().extension() == Some(OsStr::new("tf")) {
                files.push(entry.path().clone());
            }
        }
    }
    files
}

fn main() -> anyhow::Result<()> {
    let files = find_files("./test_files".into());

    for file in files {
        let contents = std::fs::read_to_string(&file)?;
        let value: serde_json::Value = hcl::from_str(&contents)?;
        let json_file = file.clone().with_extension("json");
        std::fs::write(json_file, serde_json::to_string_pretty(&value)?)?;
    }
    Ok(())
}
