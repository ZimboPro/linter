use trustfall::{
    provider::{
        field_property, resolve_property_with, AsVertex, ContextIterator, ContextOutcomeIterator,
        ResolveInfo,
    },
    FieldValue,
};

use super::vertex::Vertex;

pub(super) fn resolve_amazon_apigateway_integration_property<'a, V: AsVertex<Vertex> + 'a>(
    contexts: ContextIterator<'a, V>,
    property_name: &str,
    _resolve_info: &ResolveInfo,
) -> ContextOutcomeIterator<'a, V, FieldValue> {
    match property_name {
        "arn" => {
            todo!(
                "implement property 'arn' in fn `resolve_amazon_apigateway_integration_property()`"
            )
        }
        "httpMethod" => resolve_property_with(contexts, field_property!(httpMethod)),
        "passthroughBehavior" => {
            todo!(
                "implement property 'passthroughBehavior' in fn `resolve_amazon_apigateway_integration_property()`"
            )
        }
        "timeoutInMillis" => {
            todo!(
                "implement property 'timeoutInMillis' in fn `resolve_amazon_apigateway_integration_property()`"
            )
        }
        "trigger" => {
            todo!(
                "implement property 'trigger' in fn `resolve_amazon_apigateway_integration_property()`"
            )
        }
        "type" => {
            todo!(
                "implement property 'type' in fn `resolve_amazon_apigateway_integration_property()`"
            )
        }
        "uri" => {
            todo!(
                "implement property 'uri' in fn `resolve_amazon_apigateway_integration_property()`"
            )
        }
        _ => {
            unreachable!(
                "attempted to read unexpected property '{property_name}' on type 'AmazonApigatewayIntegration'"
            )
        }
    }
}

pub(super) fn resolve_info_property<'a, V: AsVertex<Vertex> + 'a>(
    contexts: ContextIterator<'a, V>,
    property_name: &str,
    _resolve_info: &ResolveInfo,
) -> ContextOutcomeIterator<'a, V, FieldValue> {
    match property_name {
        "description" => {
            resolve_property_with(contexts, field_property!(as_info, description))
            // todo!("implement property 'description' in fn `resolve_info_property()`")
        }
        "title" => resolve_property_with(contexts, field_property!(as_info, title)),
        "version" => resolve_property_with(contexts, field_property!(as_info, version)),
        _ => {
            unreachable!("attempted to read unexpected property '{property_name}' on type 'Info'")
        }
    }
}

pub(super) fn resolve_operation_property<'a, V: AsVertex<Vertex> + 'a>(
    contexts: ContextIterator<'a, V>,
    property_name: &str,
    _resolve_info: &ResolveInfo,
) -> ContextOutcomeIterator<'a, V, FieldValue> {
    match property_name {
        "description" => {
            resolve_property_with(contexts, field_property!(as_operation, description))
            // todo!("implement property 'description' in fn `resolve_operation_property()`")
        }
        "summary" => {
            resolve_property_with(contexts, field_property!(as_operation, summary))
            // todo!("implement property 'summary' in fn `resolve_operation_property()`")
        }
        "tags" => resolve_property_with(contexts, field_property!(as_operation, tags)),
        _ => {
            unreachable!(
                "attempted to read unexpected property '{property_name}' on type 'Operation'"
            )
        }
    }
}

pub(super) fn resolve_path_property<'a, V: AsVertex<Vertex> + 'a>(
    contexts: ContextIterator<'a, V>,
    property_name: &str,
    _resolve_info: &ResolveInfo,
) -> ContextOutcomeIterator<'a, V, FieldValue> {
    match property_name {
        "path" => todo!("implement property 'path' in fn `resolve_path_property()`"),
        _ => {
            unreachable!("attempted to read unexpected property '{property_name}' on type 'Path'")
        }
    }
}

pub(super) fn resolve_tag_property<'a, V: AsVertex<Vertex> + 'a>(
    contexts: ContextIterator<'a, V>,
    property_name: &str,
    _resolve_info: &ResolveInfo,
) -> ContextOutcomeIterator<'a, V, FieldValue> {
    match property_name {
        "description" => resolve_property_with(contexts, field_property!(as_tag, description)),
        "name" => resolve_property_with(contexts, field_property!(as_tag, name)),
        _ => {
            unreachable!("attempted to read unexpected property '{property_name}' on type 'Tag'")
        }
    }
}
