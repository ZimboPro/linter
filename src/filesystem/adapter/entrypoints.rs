use std::path::PathBuf;

use trustfall::provider::{ResolveInfo, VertexIterator};

use super::vertex::Vertex;

pub(super) fn path<'a>(path: &str, _resolve_info: &ResolveInfo) -> VertexIterator<'a, Vertex> {
    let path = PathBuf::from(path);
    if path.exists() {
        return Box::new(std::iter::once(Vertex::Path(path)));
    } else {
        return Box::new(std::iter::empty());
    }
}
