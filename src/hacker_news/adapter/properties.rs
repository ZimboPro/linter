use trustfall::{
    provider::{
        field_property, resolve_property_with, AsVertex, ContextIterator, ContextOutcomeIterator,
        ResolveInfo,
    },
    FieldValue,
};

use super::vertex::{self, Vertex};

pub(super) fn resolve_comment_property<'a, V: AsVertex<Vertex> + 'a>(
    contexts: ContextIterator<'a, V>,
    property_name: &str,
    _resolve_info: &ResolveInfo,
) -> ContextOutcomeIterator<'a, V, FieldValue> {
    match property_name {
        "byUsername" => resolve_property_with(contexts, field_property!(as_comment, by)),
        "childCount" => resolve_property_with(contexts, field_property!(as_comment, kids)),
        "id" => resolve_property_with(contexts, field_property!(as_comment, id)),
        "text" => resolve_property_with(contexts, field_property!(as_comment, text)),
        "unixTime" => resolve_property_with(contexts, field_property!(as_comment, time)),
        "url" => resolve_property_with(contexts, resolve_url),
        _ => {
            unreachable!(
                "attempted to read unexpected property '{property_name}' on type 'Comment'"
            )
        }
    }
}

// pub(super) fn resolve_item_property<'a, V: AsVertex<Vertex> + 'a>(
//     contexts: ContextIterator<'a, V>,
//     property_name: &str,
//     _resolve_info: &ResolveInfo,
// ) -> ContextOutcomeIterator<'a, V, FieldValue> {
//     match property_name {
//         "id" => resolve_property_with(contexts, field_property!(as_item, id)),
//         "unixTime" => resolve_property_with(contexts, field_property!(as_item, time)),

//         "url" => resolve_property_with(contexts, resolve_url),
//         _ => {
//             unreachable!("attempted to read unexpected property '{property_name}' on type 'Item'")
//         }
//     }
// }

pub(super) fn resolve_job_property<'a, V: AsVertex<Vertex> + 'a>(
    contexts: ContextIterator<'a, V>,
    property_name: &str,
    _resolve_info: &ResolveInfo,
) -> ContextOutcomeIterator<'a, V, FieldValue> {
    match property_name {
        "id" => resolve_property_with(contexts, field_property!(as_job, id)),
        "score" => resolve_property_with(contexts, field_property!(as_job, score)),
        "title" => resolve_property_with(contexts, field_property!(as_job, title)),
        "unixTime" => resolve_property_with(contexts, field_property!(as_job, time)),

        "url" => resolve_property_with(contexts, resolve_url),
        _ => {
            unreachable!("attempted to read unexpected property '{property_name}' on type 'Job'")
        }
    }
}

pub(super) fn resolve_poll_property<'a, V: AsVertex<Vertex> + 'a>(
    contexts: ContextIterator<'a, V>,
    property_name: &str,
    _resolve_info: &ResolveInfo,
) -> ContextOutcomeIterator<'a, V, FieldValue> {
    match property_name {
        "id" => resolve_property_with(contexts, field_property!(as_poll, id)),
        "unixTime" => resolve_property_with(contexts, field_property!(as_poll, time)),

        "url" => resolve_property_with(contexts, resolve_url),
        _ => {
            unreachable!("attempted to read unexpected property '{property_name}' on type 'Poll'")
        }
    }
}

pub(super) fn resolve_poll_option_property<'a, V: AsVertex<Vertex> + 'a>(
    contexts: ContextIterator<'a, V>,
    property_name: &str,
    _resolve_info: &ResolveInfo,
) -> ContextOutcomeIterator<'a, V, FieldValue> {
    match property_name {
        "id" => resolve_property_with(contexts, field_property!(as_poll_option, id)),
        "unixTime" => resolve_property_with(contexts, field_property!(as_poll_option, time)),

        "url" => resolve_property_with(contexts, resolve_url),
        _ => {
            unreachable!(
                "attempted to read unexpected property '{property_name}' on type 'PollOption'"
            )
        }
    }
}

pub(super) fn resolve_story_property<'a, V: AsVertex<Vertex> + 'a>(
    contexts: ContextIterator<'a, V>,
    property_name: &str,
    _resolve_info: &ResolveInfo,
) -> ContextOutcomeIterator<'a, V, FieldValue> {
    match property_name {
        "byUsername" => resolve_property_with(contexts, field_property!(as_story, by)),

        "commentsCount" => resolve_property_with(contexts, field_property!(as_story, descendants)),

        "id" => resolve_property_with(contexts, field_property!(as_story, id)),
        "score" => resolve_property_with(contexts, field_property!(as_story, score)),
        "submittedUrl" => resolve_property_with(contexts, field_property!(as_story, url)),

        "text" => resolve_property_with(contexts, field_property!(as_story, text)),
        "title" => resolve_property_with(contexts, field_property!(as_story, title)),
        "unixTime" => resolve_property_with(contexts, field_property!(as_story, time)),

        "url" => resolve_property_with(contexts, resolve_url),
        _ => {
            unreachable!("attempted to read unexpected property '{property_name}' on type 'Story'")
        }
    }
}

pub(super) fn resolve_user_property<'a, V: AsVertex<Vertex> + 'a>(
    contexts: ContextIterator<'a, V>,
    property_name: &str,
    _resolve_info: &ResolveInfo,
) -> ContextOutcomeIterator<'a, V, FieldValue> {
    match property_name {
        "about" => resolve_property_with(contexts, field_property!(as_user, about)),
        "delay" => resolve_property_with(contexts, field_property!(as_user, delay)),
        "id" => resolve_property_with(contexts, field_property!(as_user, id)),
        "karma" => resolve_property_with(contexts, field_property!(as_user, karma)),
        "unixCreatedAt" => resolve_property_with(contexts, field_property!(as_user, created)),

        _ => {
            unreachable!("attempted to read unexpected property '{property_name}' on type 'User'")
        }
    }
}

fn resolve_url(vertex: &Vertex) -> FieldValue {
    let id = match vertex {
        Vertex::Story(x) => x.id,
        Vertex::Job(x) => x.id,
        Vertex::Comment(x) => x.id,
        Vertex::Poll(x) => x.id,
        Vertex::PollOption(x) => x.id,
        Vertex::User(_) => unreachable!("found a User which is not an Item"),
    };

    format!("https://news.ycombinator.com/item?id={id}").into()
}
