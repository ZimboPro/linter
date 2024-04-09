# Roadmap

## Linter

- [ ] Update Linter and config to allow for multiple plugins
    - [ ] Select plugin from path or URL
- [ ] Allow to specify which directory plugin should have access to if applicable
- [ ] Validate if cross plugin lints are valid i.e. if the resulting output structure is correct

## Plugins

- [ ] Create a few example plugins
- [ ] Create template repos for plugins
    - [ ] Rust
    - [ ] Another language that can compile to WASI
- [ ] Extract adapters to standalone crates so others can use them in other projects


## Lints

- [ ] Create lints for each plugin
- [ ] Create lints across plugins where appropriate

## Other

- [ ] Implement benchmarking
- [ ] Unit tests
- [ ] Documentation of usage and code
- [ ] Guides for writing plugins
