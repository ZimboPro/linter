use trustfall::provider::{
    AsVertex, ContextIterator, ContextOutcomeIterator, EdgeParameters, ResolveEdgeInfo,
    VertexIterator,
};

use super::vertex::Vertex;

pub(super) fn resolve_comment_edge<'a, V: AsVertex<Vertex> + 'a>(
    contexts: ContextIterator<'a, V>,
    edge_name: &str,
    _parameters: &EdgeParameters,
    resolve_info: &ResolveEdgeInfo,
) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex>> {
    match edge_name {
        "byUser" => comment::by_user(contexts, resolve_info),
        "parent" => comment::parent(contexts, resolve_info),
        "reply" => comment::reply(contexts, resolve_info),
        _ => {
            unreachable!("attempted to resolve unexpected edge '{edge_name}' on type 'Comment'")
        }
    }
}

mod comment {
    use hn_api::types::Item;
    use trustfall::provider::{
        resolve_neighbors_with, AsVertex, ContextIterator, ContextOutcomeIterator, ResolveEdgeInfo,
        VertexIterator,
    };

    use crate::hacker_news::adapter::adapter_impl::get_client;

    use super::super::vertex::Vertex;

    pub(super) fn by_user<'a, V: AsVertex<Vertex> + 'a>(
        contexts: ContextIterator<'a, V>,
        _resolve_info: &ResolveEdgeInfo,
    ) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex>> {
        resolve_neighbors_with(contexts, move |vertex| {
            let comment = vertex
                .as_comment()
                .expect("conversion failed, vertex was not a Comment");
            let author = comment.by.as_str();
            let neighbours: VertexIterator<'a, Vertex> = match get_client().get_user(author) {
                Ok(None) => Box::new(std::iter::empty()), // no known author
                Ok(Some(user)) => Box::new(std::iter::once(user.into())),
                Err(e) => {
                    eprintln!(
                        "API error while fetching comment {} author \"{}\": {}",
                        comment.id, author, e
                    );
                    Box::new(std::iter::empty())
                }
            };
            neighbours
        })
    }

    pub(super) fn parent<'a, V: AsVertex<Vertex> + 'a>(
        contexts: ContextIterator<'a, V>,
        _resolve_info: &ResolveEdgeInfo,
    ) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex>> {
        resolve_neighbors_with(contexts, move |vertex| {
            let comment = vertex
                .as_comment()
                .expect("conversion failed, vertex was not a Comment");
            let comment_id = comment.id;
            let parent_id = comment.parent;

            let neighbours: VertexIterator<'a, Vertex> = match get_client().get_item(parent_id) {
                Ok(None) => Box::new(std::iter::empty()),
                Ok(Some(item)) => Box::new(std::iter::once(item.into())),
                Err(e) => {
                    eprintln!(
                        "API error while fetching comment {comment_id} parent {parent_id}: {e}",
                    );
                    Box::new(std::iter::empty())
                }
            };
            neighbours
        })
    }

    pub(super) fn reply<'a, V: AsVertex<Vertex> + 'a>(
        contexts: ContextIterator<'a, V>,
        _resolve_info: &ResolveEdgeInfo,
    ) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex>> {
        resolve_neighbors_with(contexts, move |vertex| {
            let comment = vertex
                .as_comment()
                .expect("conversion failed, vertex was not a Comment");
            let comment_id = comment.id;
            let reply_ids = comment.kids.clone().unwrap_or_default();

            let neigbours: VertexIterator<'a, Vertex> =
                Box::new(reply_ids.into_iter().filter_map(move |reply_id| {
                    match get_client().get_item(reply_id) {
                        Ok(None) => None,
                        Ok(Some(item)) => {
                            if let Item::Comment(c) = item {
                                Some(c.into())
                            } else {
                                unreachable!()
                            }
                        }
                        Err(e) => {
                            eprintln!(
                        "API error while fetching comment {comment_id} reply {reply_id}: {e}",
                    );
                            None
                        }
                    }
                }));
            neigbours
        })
    }
}

pub(super) fn resolve_story_edge<'a, V: AsVertex<Vertex> + 'a>(
    contexts: ContextIterator<'a, V>,
    edge_name: &str,
    _parameters: &EdgeParameters,
    resolve_info: &ResolveEdgeInfo,
) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex>> {
    match edge_name {
        "byUser" => story::by_user(contexts, resolve_info),
        "comment" => story::comment(contexts, resolve_info),
        _ => {
            unreachable!("attempted to resolve unexpected edge '{edge_name}' on type 'Story'")
        }
    }
}

mod story {
    use hn_api::types::Item;
    use trustfall::provider::{
        resolve_neighbors_with, AsVertex, ContextIterator, ContextOutcomeIterator, ResolveEdgeInfo,
        VertexIterator,
    };

    use crate::hacker_news::adapter::adapter_impl::get_client;

    use super::super::vertex::Vertex;

    pub(super) fn by_user<'a, V: AsVertex<Vertex> + 'a>(
        contexts: ContextIterator<'a, V>,
        _resolve_info: &ResolveEdgeInfo,
    ) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex>> {
        resolve_neighbors_with(contexts, move |vertex| {
            let story = vertex
                .as_story()
                .expect("conversion failed, vertex was not a Story");
            let author = story.by.as_str();
            match get_client().get_user(author) {
                Ok(None) => Box::new(std::iter::empty()), // no known author
                Ok(Some(user)) => Box::new(std::iter::once(user.into())),
                Err(e) => {
                    eprintln!(
                        "API error while fetching story {} author \"{}\": {}",
                        story.id, author, e
                    );
                    Box::new(std::iter::empty())
                }
            }
        })
    }

    pub(super) fn comment<'a, V: AsVertex<Vertex> + 'a>(
        contexts: ContextIterator<'a, V>,
        _resolve_info: &ResolveEdgeInfo,
    ) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex>> {
        resolve_neighbors_with(contexts, move |vertex| {
            let story = vertex
                .as_story()
                .expect("conversion failed, vertex was not a Story");
            let comments_ids = story.kids.clone().unwrap_or_default();
            let story_id = story.id;

            let neighbours: VertexIterator<'a, Vertex> = Box::new(comments_ids.into_iter().filter_map(move |comment_id| {
                match get_client().get_item(comment_id) {
                    Ok(None) => None,
                                Ok(Some(item)) => {
                                    if let Item::Comment(comment) = item {
                                        Some(comment.into())
                                    } else {
                                        unreachable!()
                                    }
                                }
                                Err(e) => {
                                    eprintln!(
                                        "API error while fetching story {story_id} comment {comment_id}: {e}",
                                    );
                                    None
                                }
                }
            }));
            neighbours
        })
    }
}

pub(super) fn resolve_user_edge<'a, V: AsVertex<Vertex> + 'a>(
    contexts: ContextIterator<'a, V>,
    edge_name: &str,
    _parameters: &EdgeParameters,
    resolve_info: &ResolveEdgeInfo,
) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex>> {
    match edge_name {
        "submitted" => user::submitted(contexts, resolve_info),
        _ => {
            unreachable!("attempted to resolve unexpected edge '{edge_name}' on type 'User'")
        }
    }
}

mod user {
    use trustfall::provider::{
        resolve_neighbors_with, AsVertex, ContextIterator, ContextOutcomeIterator, ResolveEdgeInfo,
        VertexIterator,
    };

    use crate::hacker_news::adapter::adapter_impl::get_client;

    use super::super::vertex::Vertex;

    pub(super) fn submitted<'a, V: AsVertex<Vertex> + 'a>(
        contexts: ContextIterator<'a, V>,
        _resolve_info: &ResolveEdgeInfo,
    ) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex>> {
        resolve_neighbors_with(contexts, move |vertex| {
            let user = vertex
                .as_user()
                .expect("conversion failed, vertex was not a User");
            let submitted_ids = user.submitted.clone();

            let neighbours: VertexIterator<'a, Vertex> =
                Box::new(submitted_ids.into_iter().filter_map(move |submission_id| {
                    match get_client().get_item(submission_id) {
                        Ok(None) => None,
                        Ok(Some(item)) => Some(item.into()),
                        Err(e) => {
                            eprintln!(
                                "API error while fetching submitted item {submission_id}: {e}",
                            );
                            None
                        }
                    }
                }));
            neighbours
        })
    }
}
