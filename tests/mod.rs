use std::path::Path;

mod examples_in_readme;
mod files_are_equal;
mod files_are_similar;
mod files_differ;
mod invalid_cli_commands;
mod library;
mod no_cli_arguments_passed;
mod quiet_option;
mod read_error;
mod valid_file_size_suffixes;

fn current_dir(file: &str) -> &Path {
  Path::new(file).parent().unwrap()
}
