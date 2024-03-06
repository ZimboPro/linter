# Linter

A tool to lint OpenAPI and/or Terraform files and pickup any errors according to the rules specified

## Usage

`linter -a [api yaml file or folder] -t [terraform folder] -c [config/lint file]`

`-a` is optional

## Query

Query uses GraphQL.

Typically, when you make a call to a database for data, you are expecting to receive data.
For the tool, it is more or less the opposite, you want to query to return data **if** it
is an error. For e.g. return any data where the tags are not set

### Available Helper Functions

  - filter - able to filter out the data
    - op - the operation name
    - value - (Optional) array of values to filter against
  - tag - apply a name to the given property field
    - name - (optional) name to apply
  - output - outputs the value
    - name - (Optional) The name to use when outputting
  - optional - optional field in query
  - recurse - recurse through x times on this edge
    - depth - The recursive depth
  - fold - apply a fold
  - transform - run a transformation operation on value
    - op - The operation to run

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

### Transform

 - count