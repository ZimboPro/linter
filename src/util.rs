use std::{
    ffi::OsStr,
    fs,
    path::{Components, PathBuf},
};

use trustfall::FieldValue;

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
    for entry in path
        .read_dir()
        .expect("Failed to get dir contents")
        .flatten()
    {
        if entry.path().is_dir() {
            files.extend(find_files(entry.path(), extension));
        } else if entry.path().is_file() && entry.path().extension() == Some(OsStr::new(extension))
        {
            files.push(entry.path().clone());
        }
    }
    files
}

pub fn find_files_ignore_dir(path: PathBuf, extension: &str, folder: &str) -> Vec<PathBuf> {
    let mut files = Vec::new();
    for entries in path.read_dir().expect("Failed to get dir contents") {
        if let Ok(entry) = entries {
            if entry.path().is_dir() && !entry.path().ends_with(folder) {
                files.extend(find_files_ignore_dir(entry.path(), extension, folder));
            } else if entry.path().is_file()
                && entry.path().extension() == Some(OsStr::new(extension))
            {
                files.push(entry.path().clone());
            }
        }
    }
    files
}

pub fn from_field_value(value: &FieldValue) -> serde_json::Value {
    match value {
        FieldValue::Null => serde_json::Value::Null,
        FieldValue::Int64(val) => val.to_owned().into(),
        FieldValue::Uint64(val) => val.to_owned().into(),
        FieldValue::Float64(val) => val.to_owned().into(),
        FieldValue::String(val) => val.to_string().into(),
        FieldValue::Boolean(val) => val.to_owned().into(),
        FieldValue::Enum(val) => val.to_string().into(),
        FieldValue::List(val) => {
            let mut list = serde_json::Value::Array(Vec::new());
            for item in val.iter() {
                list.as_array_mut().unwrap().push(from_field_value(item));
            }
            list
        }
        _ => todo!(),
    }
}

pub fn from_json_value(value: &serde_json::Value) -> FieldValue {
    match value {
        serde_json::Value::Null => FieldValue::Null,
        serde_json::Value::Bool(val) => FieldValue::Boolean(*val),
        serde_json::Value::Number(val) => {
            if val.is_i64() {
                FieldValue::Int64(val.as_i64().unwrap())
            } else if val.is_u64() {
                FieldValue::Uint64(val.as_u64().unwrap())
            } else {
                FieldValue::Float64(val.as_f64().unwrap())
            }
        }
        serde_json::Value::String(val) => FieldValue::String(val.to_string().into()),
        serde_json::Value::Array(val) => {
            let mut list = Vec::new();
            for item in val.iter() {
                list.push(from_json_value(item));
            }
            FieldValue::List(list.into())
        }
        _ => todo!(),
    }
}
