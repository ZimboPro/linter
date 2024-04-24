mod errors;
mod lint;
mod utils;

pub use lint::{Lint, LintResult};

pub use errors::PluginErrors;
pub use trustfall;
pub use utils::{
    convert_to_args, find_files, find_files_ignore_dir, from_field_value, from_json_value,
    open_file,
};
