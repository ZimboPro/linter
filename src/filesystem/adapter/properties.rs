use sha256::digest;
use std::os::windows::fs::MetadataExt;

use trustfall::{
    provider::{
        resolve_property_with, AsVertex, ContextIterator, ContextOutcomeIterator, ResolveInfo,
    },
    FieldValue,
};

use super::vertex::Vertex;

pub(super) fn resolve_file_property<'a, V: AsVertex<Vertex> + 'a>(
    contexts: ContextIterator<'a, V>,
    property_name: &str,
    _resolve_info: &ResolveInfo,
) -> ContextOutcomeIterator<'a, V, FieldValue> {
    match property_name {
        "Hash" => resolve_property_with(contexts, |vertex: &Vertex| match vertex {
            Vertex::File(path) => {
                let bytes = std::fs::read(path).unwrap();
                digest(bytes).into()
            }
            _ => unreachable!(),
        }),
        "extension" => resolve_property_with(contexts, |vertex: &Vertex| match vertex {
            Vertex::File(path) => path.extension().unwrap().to_str().unwrap().into(),
            _ => unreachable!(),
        }),
        "path" => resolve_property_with(contexts, |vertex: &Vertex| match vertex {
            Vertex::File(path) => path.to_str().unwrap().into(),
            Vertex::Folder(path) => path.to_str().unwrap().into(),
            Vertex::Path(path) => path.to_str().unwrap().into(),
        }),
        "size" => resolve_property_with(contexts, |vertex: &Vertex| match vertex {
            Vertex::File(path) => {
                // if path.is_file() {
                path.metadata().unwrap().file_size().into()
                // }
            }
            _ => unreachable!(),
        }),
        _ => {
            unreachable!("attempted to read unexpected property '{property_name}' on type 'File'")
        }
    }
}

pub(super) fn resolve_folder_property<'a, V: AsVertex<Vertex> + 'a>(
    contexts: ContextIterator<'a, V>,
    property_name: &str,
    _resolve_info: &ResolveInfo,
) -> ContextOutcomeIterator<'a, V, FieldValue> {
    match property_name {
        "path" => resolve_property_with(contexts, |vertex: &Vertex| match vertex {
            Vertex::Folder(path) => path.to_str().unwrap().into(),
            _ => unreachable!(),
        }),
        _ => {
            unreachable!("attempted to read unexpected property '{property_name}' on type 'Folder'")
        }
    }
}

pub(super) fn resolve_path_property<'a, V: AsVertex<Vertex> + 'a>(
    contexts: ContextIterator<'a, V>,
    property_name: &str,
    _resolve_info: &ResolveInfo,
) -> ContextOutcomeIterator<'a, V, FieldValue> {
    match property_name {
        "path" => resolve_property_with(contexts, |vertex: &Vertex| match vertex {
            Vertex::Path(path) => path.to_str().unwrap().into(),
            _ => unreachable!(),
        }),
        _ => {
            unreachable!("attempted to read unexpected property '{property_name}' on type 'Path'")
        }
    }
}
