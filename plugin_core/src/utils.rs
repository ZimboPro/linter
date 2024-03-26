use std::{
    collections::{BTreeMap, HashMap},
    ffi::OsStr,
    io::Read,
    path::PathBuf,
    sync::Arc,
};
use trustfall::FieldValue;

pub fn find_files(path: &std::path::Path, extension: &OsStr) -> Vec<PathBuf> {
    let mut files = Vec::new();
    for entry in path.read_dir().expect("Failed to read directory").flatten() {
        if entry.path().is_dir() {
            files.append(&mut find_files(&entry.path(), extension));
        } else if entry.path().extension() == Some(extension) {
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

pub fn convert_to_args(args: HashMap<String, serde_json::Value>) -> BTreeMap<Arc<str>, FieldValue> {
    if !args.is_empty() {
        let v: BTreeMap<Arc<str>, FieldValue> = args
            .iter()
            .map(|(k, v)| (Arc::from(k.as_str()), from_json_value(v)))
            .collect();

        v
    } else {
        BTreeMap::new()
    }
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
