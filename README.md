# Linter

A tool to lint the files and pickup any errors

## Usage

`linter query <query-file> [<max_results>]`

## Query

Query uses GraphQL

### Filters

- is_null
- is_not_null
- =
- !=
- <
- <=
- >
- >=
- contains
- not_contains
- one_of
- not_one_of
- has_prefix
- not_has_prefix
- has_suffix
- not_has_suffix
- has_substring
- not_has_substring
- regex
- not_regex

## TODOs

- [ ] Add response to OpenAPI schema
- [ ] Determine what to query in Terraform
  - [ ] Terraform
    - [ ] required version
      - [ ] Map to semver struct?
    - [ ] backend
  - [ ] resource
    - [ ] type
    - [ ] name
    - [ ] properties
  - [ ] variable
    - [ ] name
    - [ ] type
    - [ ] default
    - [ ] description
  - [ ] locals
    - [ ] name
    - [ ] value
  - [ ] module
    - [ ] name
    - [ ] source
    - [ ] properties
    - [ ] lambda permissions
      - [ ] endpoints
  - [ ] provider
    - [ ] provider name
    - [ ] properties

## Terraform

Able