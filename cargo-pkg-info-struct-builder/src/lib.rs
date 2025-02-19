pub mod build_utils;
pub use build_utils::BuildUtils;
use std::path::PathBuf;

/// Proxies [`BuildUtils::inject_build_metadata`]. See its documentation for details.
///
/// See [`BuildUtils::inject_build_metadata`] for full documentation.
pub fn inject_build_metadata(project_dest_path: PathBuf) {
    BuildUtils::inject_build_metadata(project_dest_path)
}

/// Sets an environment variable for Cargo at build time, ensuring the variable name is valid.
///
/// See [`BuildUtils::set_cargo_env_var`] for full documentation.
pub fn set_cargo_env_var(var_name: &str, value: &str) {
    BuildUtils::set_cargo_env_var(var_name, value)
}

/// Sets an environment variable for Cargo with auto-indented multi-line content.
///
/// See [`BuildUtils::set_multi_line_cargo_env_var`] for full documentation.
pub fn set_multi_line_cargo_env_var(var_name: &str, value: &str) {
    BuildUtils::set_multi_line_cargo_env_var(var_name, value);
}
