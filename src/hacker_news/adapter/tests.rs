use trustfall::provider::check_adapter_invariants;

use super::HackerNewsAdapter;

#[test]
fn adapter_satisfies_trustfall_invariants() {
    let adapter = HackerNewsAdapter::new();
    let schema = HackerNewsAdapter::schema();
    check_adapter_invariants(schema, adapter);
}
