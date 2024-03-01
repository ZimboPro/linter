use trustfall::provider::{ResolveInfo, VertexIterator};

use super::vertex::Vertex;

pub(super) fn info<'a>(
    _resolve_info: &ResolveInfo,
    doc: &openapiv3::OpenAPI,
) -> VertexIterator<'a, Vertex> {
    Box::new(std::iter::once(Vertex::Info(doc.info.clone())))
}

pub(super) fn path<'a>(
    path: &str,
    _resolve_info: &ResolveInfo,
    doc: &openapiv3::OpenAPI,
) -> VertexIterator<'a, Vertex> {
    Box::new(std::iter::once(Vertex::Path((
        path.to_string(),
        doc.paths.paths.get(path).expect("path not found").clone(),
    ))))
}

pub(super) fn paths<'a>(
    _resolve_info: &ResolveInfo,
    doc: &openapiv3::OpenAPI,
) -> VertexIterator<'a, Vertex> {
    Box::new(std::iter::once(Vertex::Paths(
        doc.paths.clone().into_iter().collect(),
    )))
}

pub(super) fn tags<'a>(
    _resolve_info: &ResolveInfo,
    doc: &openapiv3::OpenAPI,
) -> VertexIterator<'a, Vertex> {
    Box::new(std::iter::once(Vertex::Tags(doc.tags.clone())))
}
