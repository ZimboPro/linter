use std::{
    collections::HashSet,
    sync::{Arc, OnceLock},
};

use hn_api::HnClient;
use trustfall::{
    provider::{
        resolve_coercion_using_schema, resolve_property_with, AsVertex, ContextIterator,
        ContextOutcomeIterator, EdgeParameters, ResolveEdgeInfo, ResolveInfo, Typename,
        VertexIterator,
    },
    FieldValue, Schema,
};

use super::vertex::Vertex;

static SCHEMA: OnceLock<Schema> = OnceLock::new();

static CLIENT: OnceLock<HnClient> = OnceLock::new();

pub fn get_client() -> &'static HnClient {
    CLIENT.get_or_init(|| HnClient::init().expect("HnClient instantiated"))
}

#[non_exhaustive]
#[derive(Debug)]
pub struct HackerNewsAdapter {
    /// Set of types that implement the Item interface in the schema
    item_subtypes: HashSet<String>,
}

impl Default for HackerNewsAdapter {
    fn default() -> Self {
        Self::new()
    }
}

impl HackerNewsAdapter {
    pub const SCHEMA_TEXT: &'static str = include_str!("./schema.graphql");

    pub fn schema() -> &'static Schema {
        SCHEMA.get_or_init(|| Schema::parse(Self::SCHEMA_TEXT).expect("not a valid schema"))
    }

    pub fn new() -> Self {
        Self {
            item_subtypes: HackerNewsAdapter::schema()
                .subtypes("Item")
                .expect("Item type exists")
                .map(|x| x.to_owned())
                .collect(),
        }
    }

    fn front_page<'a>(&self) -> VertexIterator<'a, Vertex> {
        self.top(Some(30))
    }

    fn top<'a>(&self, max: Option<usize>) -> VertexIterator<'a, Vertex> {
        let iterator = get_client()
            .get_top_stories()
            .unwrap()
            .into_iter()
            .take(max.unwrap_or(usize::MAX))
            .filter_map(|id| match get_client().get_item(id) {
                Ok(maybe_item) => maybe_item.map(|item| item.into()),
                Err(e) => {
                    eprintln!("Got an error while fetching item: {e}");
                    None
                }
            });
        Box::new(iterator)
    }

    fn latest_stories<'a>(&self, max: Option<usize>) -> VertexIterator<'a, Vertex> {
        let story_ids: Vec<u32> =
            reqwest::blocking::get("https://hacker-news.firebaseio.com/v0/newstories.json")
                .unwrap()
                .json()
                .unwrap();

        let iterator = story_ids
            .into_iter()
            .take(max.unwrap_or(usize::MAX))
            .map(move |id| get_client().get_item(id))
            .filter_map(|res| match res {
                Ok(maybe_item) => maybe_item.map(|item| item.into()),
                Err(e) => {
                    eprintln!("Got an error while fetching item: {e}");
                    None
                }
            });

        Box::new(iterator)
    }

    fn user<'a>(&self, username: &str) -> VertexIterator<'a, Vertex> {
        match get_client().get_user(username) {
            Ok(Some(user)) => {
                let vertex = Vertex::from(user);
                Box::new(std::iter::once(vertex))
            }
            Ok(None) => Box::new(std::iter::empty()),
            Err(e) => {
                eprintln!("Got an error while getting user profile for user {username}: {e}",);
                Box::new(std::iter::empty())
            }
        }
    }
}

macro_rules! item_property_resolver {
    ($attr:ident) => {
        |vertex| -> FieldValue {
            if let Some(s) = vertex.as_story() {
                s.$attr.clone().into()
            } else if let Some(j) = vertex.as_job() {
                j.$attr.clone().into()
            } else if let Some(c) = vertex.as_comment() {
                c.$attr.clone().into()
            } else if let Some(p) = vertex.as_poll() {
                p.$attr.clone().into()
            } else if let Some(p) = vertex.as_poll_option() {
                p.$attr.clone().into()
            } else {
                unreachable!("{:?}", vertex)
            }
        }
    };
}

impl<'a> trustfall::provider::Adapter<'a> for HackerNewsAdapter {
    type Vertex = Vertex;

    fn resolve_starting_vertices(
        &self,
        edge_name: &Arc<str>,
        parameters: &EdgeParameters,
        _resolve_info: &ResolveInfo,
    ) -> VertexIterator<'a, Self::Vertex> {
        match edge_name.as_ref() {
            "FrontPage" => self.front_page(),
            "Latest" => {
                let max: Option<usize> = parameters
                    .get("max")
                    .expect(
                        "failed to find parameter 'max' when resolving 'Latest' starting vertices",
                    )
                    .as_usize();
                self.top(max)
            }
            "Top" => {
                let max: Option<usize> = parameters
                    .get("max")
                    .expect("failed to find parameter 'max' when resolving 'Top' starting vertices")
                    .as_usize();
                self.latest_stories(max)
            }
            "User" => {
                let name: &str = parameters
                    .get("name")
                    .expect(
                        "failed to find parameter 'name' when resolving 'User' starting vertices",
                    )
                    .as_str()
                    .expect(
                        "unexpected null or other incorrect datatype for Trustfall type 'String!'",
                    );
                self.user(name)
            }
            _ => {
                unreachable!(
                    "attempted to resolve starting vertices for unexpected edge name: {edge_name}"
                )
            }
        }
    }

    fn resolve_property<V: AsVertex<Self::Vertex> + 'a>(
        &self,
        contexts: ContextIterator<'a, V>,
        type_name: &Arc<str>,
        property_name: &Arc<str>,
        resolve_info: &ResolveInfo,
    ) -> ContextOutcomeIterator<'a, V, FieldValue> {
        if property_name.as_ref() == "__typename" {
            return resolve_property_with(contexts, |vertex| vertex.typename().into());
        }
        match type_name.as_ref() {
            "Comment" => super::properties::resolve_comment_property(
                contexts,
                property_name.as_ref(),
                resolve_info,
            ),
            // "Item" => super::properties::resolve_item_property(
            //     contexts,
            //     property_name.as_ref(),
            //     resolve_info,
            // ),
            "Job" => super::properties::resolve_job_property(
                contexts,
                property_name.as_ref(),
                resolve_info,
            ),
            "Poll" => super::properties::resolve_poll_property(
                contexts,
                property_name.as_ref(),
                resolve_info,
            ),
            "PollOption" => super::properties::resolve_poll_option_property(
                contexts,
                property_name.as_ref(),
                resolve_info,
            ),
            "Story" => super::properties::resolve_story_property(
                contexts,
                property_name.as_ref(),
                resolve_info,
            ),
            "User" => super::properties::resolve_user_property(
                contexts,
                property_name.as_ref(),
                resolve_info,
            ),
            _ => {
                unreachable!(
                    "attempted to read property '{property_name}' on unexpected type: {type_name}"
                )
            }
        }
    }

    fn resolve_neighbors<V: AsVertex<Self::Vertex> + 'a>(
        &self,
        contexts: ContextIterator<'a, V>,
        type_name: &Arc<str>,
        edge_name: &Arc<str>,
        parameters: &EdgeParameters,
        resolve_info: &ResolveEdgeInfo,
    ) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Self::Vertex>> {
        match type_name.as_ref() {
            "Comment" => super::edges::resolve_comment_edge(
                contexts,
                edge_name.as_ref(),
                parameters,
                resolve_info,
            ),
            "Story" => super::edges::resolve_story_edge(
                contexts,
                edge_name.as_ref(),
                parameters,
                resolve_info,
            ),
            "User" => super::edges::resolve_user_edge(
                contexts,
                edge_name.as_ref(),
                parameters,
                resolve_info,
            ),
            _ => {
                unreachable!(
                    "attempted to resolve edge '{edge_name}' on unexpected type: {type_name}"
                )
            }
        }
    }

    fn resolve_coercion<V: AsVertex<Self::Vertex> + 'a>(
        &self,
        contexts: ContextIterator<'a, V>,
        _type_name: &Arc<str>,
        coerce_to_type: &Arc<str>,
        _resolve_info: &ResolveInfo,
    ) -> ContextOutcomeIterator<'a, V, bool> {
        resolve_coercion_using_schema(contexts, Self::schema(), coerce_to_type.as_ref())
    }
}
