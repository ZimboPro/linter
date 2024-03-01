use trustfall::provider::check_adapter_invariants;

use super::OpenApiAdapter;

#[test]
fn adapter_satisfies_trustfall_invariants() {
    let adapter = OpenApiAdapter::new();
    let schema = OpenApiAdapter::schema();
    check_adapter_invariants(schema, adapter);
}
