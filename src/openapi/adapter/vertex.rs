use std::rc::Rc;

use openapiv3::{PathItem, ReferenceOr, Schema};

#[non_exhaustive]
#[derive(Debug, Clone, trustfall::provider::TrustfallEnumVertex)]
pub enum Vertex {
    AmazonApigatewayIntegration(()),
    Info(openapiv3::Info),
    Operation(openapiv3::Operation),
    Path((String, ReferenceOr<PathItem>)),
    Paths(Vec<(String, ReferenceOr<PathItem>)>),
    Tags(Vec<openapiv3::Tag>),
    Tag(openapiv3::Tag),
}
