use trustfall::provider::{ResolveInfo, VertexIterator};

use super::vertex::Vertex;

pub(super) fn front_page<'a>(_resolve_info: &ResolveInfo) -> VertexIterator<'a, Vertex> {
    todo!("implement resolving starting vertices for entrypoint edge 'FrontPage'")
}

pub(super) fn latest<'a>(
    _max: Option<i64>,
    _resolve_info: &ResolveInfo,
) -> VertexIterator<'a, Vertex> {
    todo!("implement resolving starting vertices for entrypoint edge 'Latest'")
}

pub(super) fn top<'a>(
    _max: Option<i64>,
    _resolve_info: &ResolveInfo,
) -> VertexIterator<'a, Vertex> {
    todo!("implement resolving starting vertices for entrypoint edge 'Top'")
}

pub(super) fn user<'a>(_name: &str, _resolve_info: &ResolveInfo) -> VertexIterator<'a, Vertex> {
    todo!("implement resolving starting vertices for entrypoint edge 'User'")
}
