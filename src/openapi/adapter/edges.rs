use trustfall::provider::{
    AsVertex, ContextIterator, ContextOutcomeIterator, EdgeParameters, ResolveEdgeInfo,
    VertexIterator,
};

use super::vertex::Vertex;

pub(super) fn resolve_operation_edge<'a, V: AsVertex<Vertex> + 'a>(
    contexts: ContextIterator<'a, V>,
    edge_name: &str,
    parameters: &EdgeParameters,
    resolve_info: &ResolveEdgeInfo,
) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex>> {
    match edge_name {
        "xAmazonApigatewayIntegration" => {
            operation::x_amazon_apigateway_integration(contexts, resolve_info)
        }
        _ => {
            unreachable!("attempted to resolve unexpected edge '{edge_name}' on type 'Operation'")
        }
    }
}

mod operation {
    use trustfall::provider::{
        resolve_neighbors_with, AsVertex, ContextIterator, ContextOutcomeIterator, ResolveEdgeInfo,
        VertexIterator,
    };

    use super::super::vertex::Vertex;

    pub(super) fn x_amazon_apigateway_integration<'a, V: AsVertex<Vertex> + 'a>(
        contexts: ContextIterator<'a, V>,
        _resolve_info: &ResolveEdgeInfo,
    ) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex>> {
        resolve_neighbors_with(contexts, move |vertex| {
            let vertex = vertex
                .as_operation()
                .expect("conversion failed, vertex was not a Operation");
            todo!("get neighbors along edge 'xAmazonApigatewayIntegration' for type 'Operation'")
        })
    }
}

pub(super) fn resolve_path_edge<'a, V: AsVertex<Vertex> + 'a>(
    contexts: ContextIterator<'a, V>,
    edge_name: &str,
    parameters: &EdgeParameters,
    resolve_info: &ResolveEdgeInfo,
) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex>> {
    match edge_name {
        "delete" => path::delete(contexts, resolve_info),
        "get" => path::get(contexts, resolve_info),
        "options" => path::options(contexts, resolve_info),
        "patch" => path::patch(contexts, resolve_info),
        "post" => path::post(contexts, resolve_info),
        "put" => path::put(contexts, resolve_info),
        _ => {
            unreachable!("attempted to resolve unexpected edge '{edge_name}' on type 'Path'")
        }
    }
}

mod path {
    use trustfall::provider::{
        resolve_neighbors_with, AsVertex, ContextIterator, ContextOutcomeIterator, ResolveEdgeInfo,
        VertexIterator,
    };

    use super::super::vertex::Vertex;

    pub(super) fn delete<'a, V: AsVertex<Vertex> + 'a>(
        contexts: ContextIterator<'a, V>,
        _resolve_info: &ResolveEdgeInfo,
    ) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex>> {
        resolve_neighbors_with(contexts, move |vertex| {
            let vertex = vertex
                .as_path()
                .expect("conversion failed, vertex was not a Path");
            todo!("get neighbors along edge 'delete' for type 'Path'")
        })
    }

    pub(super) fn get<'a, V: AsVertex<Vertex> + 'a>(
        contexts: ContextIterator<'a, V>,
        _resolve_info: &ResolveEdgeInfo,
    ) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex>> {
        resolve_neighbors_with(contexts, move |vertex| {
            let vertex = vertex
                .as_path()
                .expect("conversion failed, vertex was not a Path");
            todo!("get neighbors along edge 'get' for type 'Path'")
        })
    }

    pub(super) fn options<'a, V: AsVertex<Vertex> + 'a>(
        contexts: ContextIterator<'a, V>,
        _resolve_info: &ResolveEdgeInfo,
    ) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex>> {
        resolve_neighbors_with(contexts, move |vertex| {
            let vertex = vertex
                .as_path()
                .expect("conversion failed, vertex was not a Path");
            todo!("get neighbors along edge 'options' for type 'Path'")
        })
    }

    pub(super) fn patch<'a, V: AsVertex<Vertex> + 'a>(
        contexts: ContextIterator<'a, V>,
        _resolve_info: &ResolveEdgeInfo,
    ) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex>> {
        resolve_neighbors_with(contexts, move |vertex| {
            let vertex = vertex
                .as_path()
                .expect("conversion failed, vertex was not a Path");
            todo!("get neighbors along edge 'patch' for type 'Path'")
        })
    }

    pub(super) fn post<'a, V: AsVertex<Vertex> + 'a>(
        contexts: ContextIterator<'a, V>,
        _resolve_info: &ResolveEdgeInfo,
    ) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex>> {
        resolve_neighbors_with(contexts, move |vertex| {
            let vertex = vertex
                .as_path()
                .expect("conversion failed, vertex was not a Path");
            todo!("get neighbors along edge 'post' for type 'Path'")
        })
    }

    pub(super) fn put<'a, V: AsVertex<Vertex> + 'a>(
        contexts: ContextIterator<'a, V>,
        _resolve_info: &ResolveEdgeInfo,
    ) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex>> {
        resolve_neighbors_with(contexts, move |vertex| {
            let vertex = vertex
                .as_path()
                .expect("conversion failed, vertex was not a Path");
            todo!("get neighbors along edge 'put' for type 'Path'")
        })
    }
}
