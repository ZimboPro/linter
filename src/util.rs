use std::{
    ffi::OsStr,
    fs,
    path::{Components, PathBuf},
};

/// Pop path components from the front of the path component iterator, then try the read again.
fn path_compensating_read(mut iter: Components<'_>, tries_remaining: i64) -> Result<String, ()> {
    match iter.next() {
        Some(_) => match fs::read_to_string(iter.as_path()) {
            Ok(content) => Ok(content),
            Err(_) => {
                if tries_remaining > 0 {
                    path_compensating_read(iter, tries_remaining - 1)
                } else {
                    Err(())
                }
            }
        },
        None => Err(()),
    }
}

pub(super) fn read_file(path: &str) -> String {
    match fs::read_to_string(path) {
        Ok(content) => content,
        Err(e) => {
            // Maybe the user is too deep in the directory tree.
            // Try skipping some components from the front of the path.
            let path = PathBuf::from(path);
            path_compensating_read(path.components(), 3)
                .map_err(|_| e)
                .expect("failed to read file")
        }
    }
}

pub fn find_files(path: PathBuf, extension: &str) -> Vec<PathBuf> {
    let mut files = Vec::new();
    for entries in path.read_dir().expect("Failed to get dir contents") {
        if let Ok(entry) = entries {
            if entry.path().is_dir() {
                files.extend(find_files(entry.path(), extension));
            } else if entry.path().is_file()
                && entry.path().extension() == Some(OsStr::new(extension))
            {
                files.push(entry.path().clone());
            }
        }
    }
    files
}
