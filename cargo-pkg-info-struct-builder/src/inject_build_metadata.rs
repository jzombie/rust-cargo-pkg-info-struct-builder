use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};
use string_auto_indent::auto_indent;
use toml::Value;

/// Injects build metadata, including license content if available.
///
/// This function gathers metadata such as:
/// - The build timestamp in UTC (`BUILD_TIME_UTC`)
/// - The target architecture/OS (`BUILD_TARGET`)
/// - The contents of the license file (`LICENSE_CONTENT`), if specified in `Cargo.toml`
///
/// It writes the metadata into a file specified by `project_dest_path`. If the
/// directory for the file does not exist, it is created. The function also ensures
/// that if `Cargo.toml`, `inject.rs`, or the license file changes, the build will be rerun.
///
/// # Arguments
///
/// * `project_dest_path` - A `PathBuf` representing the relative path to the target file.
///   This file is where the build metadata will be injected.
///
/// # Behavior
/// - Reads the license file path from `Cargo.toml`, if available.
/// - Reads the license file's content and sets it as an environment variable (`LICENSE_CONTENT`).
/// - Ensures that Cargo rebuilds if `Cargo.toml`, `inject.rs`, or the license file changes.
///
/// # Panics
///
/// This function will panic if:
/// - The `CARGO_MANIFEST_DIR` environment variable is not set.
/// - The destination directory cannot be created.
/// - The file cannot be written.
/// - The parent directory of the file cannot be determined.
pub fn inject_build_metadata(project_dest_path: PathBuf) {
    // Retrieve the manifest directory
    let manifest_dir =
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set"));

    let dest_path = manifest_dir.join(&project_dest_path);
    let destination_dir = dest_path.parent().unwrap();

    // Ensure the generated directory exists
    fs::create_dir_all(&destination_dir).expect("Failed to create generated directory");

    // Retrieve the build target
    let build_target = env::var("TARGET").unwrap_or_else(|_| "unknown-target".to_string());
    set_cargo_env_var("BUILD_TARGET", &build_target);

    // Get the build time in UTC
    let build_time_utc = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs()
        .to_string();
    set_cargo_env_var("BUILD_TIME_UTC", &build_time_utc);

    // Read and set the license content if available
    if let Some(license_path) = get_license_file_path(&manifest_dir) {
        if let Ok(license_content) = fs::read_to_string(&license_path) {
            set_cargo_env_var("LICENSE_CONTENT", &license_content);
        }
    }

    // Embed inject.rs as bytes at compile time
    const CONTENTS: &[u8] = include_bytes!("inject_build_metadata.struct.rs");

    // Check if the file already exists with the same content
    if let Ok(existing_contents) = fs::read(&dest_path) {
        if existing_contents == CONTENTS {
            // If the contents are identical, skip rewriting
            println!(
                "No changes to {}; skipping file write.",
                dest_path.display()
            );
            // We do still print rerun-if-changed triggers, so keep going
        } else {
            // Write the embedded bytes to cargo_pkg_info.rs
            fs::write(&dest_path, CONTENTS).expect("Failed to write metadata file");
        }
    } else {
        // If we couldn't read it (likely doesn't exist), just create it
        fs::write(&dest_path, CONTENTS).expect("Failed to write metadata file");
    }

    // Ensure Cargo rebuilds if Cargo.toml or the license file changes
    println!("cargo:rerun-if-changed=Cargo.toml");
    println!("cargo:rerun-if-changed=inject.rs");
    if let Some(license_path) = get_license_file_path(&manifest_dir) {
        println!("cargo:rerun-if-changed={}", license_path.display());
    }
}

/// Escapes all newline sequences, normalizing `\r\n` to `\n`,
/// then replacing `\n` with `\\n`.
///
/// Note: At this time this is intentionally normalized to Unix-style line endings.
fn escape_newlines(s: &str) -> String {
    s.replace("\r\n", "\n").replace('\n', "\\n")
}

/// Reads `Cargo.toml`, parses it, and extracts the value of a specified field.
///
/// This function reads the `[package]` section of `Cargo.toml` and returns the value
/// of the requested key if it exists.
///
/// # Arguments
///
/// * `manifest_dir` - The path to the consuming package's root directory.
/// * `field` - The key in `Cargo.toml` to retrieve (e.g., `"license-file"`, `"version"`, etc.).
///
/// # Returns
///
/// * `Some(String)` - The extracted value if found.
/// * `None` - If the key does not exist.
pub fn get_cargo_field(manifest_dir: &Path, field: &str) -> Option<String> {
    let cargo_toml_path = manifest_dir.join("Cargo.toml");
    let cargo_toml_content = fs::read_to_string(&cargo_toml_path).ok()?;
    let cargo_toml: Value = toml::from_str(&cargo_toml_content).ok()?;

    cargo_toml
        .get("package")?
        .get(field)?
        .as_str()
        .map(|s| s.to_string())
}

/// Retrieves the absolute path of the `license-file` from `Cargo.toml`, if available.

/// Reads `Cargo.toml`, extracts the `license-file` path, and returns it.
///
/// This function looks for the `license-file` key in `Cargo.toml` and extracts
/// its value, which represents the relative path to the license file. If found,
/// the function returns the absolute path to the file.
///
/// # Arguments
///
/// * `manifest_dir` - A reference to the project's root directory.
///
/// # Returns
///
/// * `Some(PathBuf)` - The absolute path to the license file if found.
/// * `None` - If `license-file` is not defined in `Cargo.toml`.
///
/// # Behavior
///
/// - Reads the `Cargo.toml` file and scans for the `license-file` key.
/// - If found, it extracts and constructs the full path relative to `manifest_dir`.
/// - If no `license-file` key is found, it returns `None`.
///
/// # Notes
///
/// - This function does **not** verify whether the license file exists.
/// - It does **not** parse TOML properly; it just scans lines for the `license-file` key.
/// - If `Cargo.toml` uses unconventional formatting, it might not be detected.
pub fn get_license_file_path(manifest_dir: &Path) -> Option<PathBuf> {
    get_cargo_field(manifest_dir, "license-file").map(|rel_path| manifest_dir.join(rel_path))
}

/// Sets an environment variable for Cargo at build time, ensuring the variable name is valid.
///
/// # Arguments
/// * `var_name` - The name of the environment variable (validated).
/// * `value` - The value to assign, with escaped newlines.
///
/// # Panics
/// This function will panic if:
/// - The variable name is empty.
/// - The variable name contains invalid characters.
/// - The variable name starts with a non-alphabetic character.
///
/// ```
/// use cargo_pkg_info_struct_builder::set_cargo_env_var;
/// set_cargo_env_var("BUILD_INFO", "Rust Build System");
/// ```
pub fn set_cargo_env_var(var_name: &str, value: &str) {
    assert!(
        is_valid_env_var_name(var_name),
        "Invalid Cargo environment variable name: '{}'",
        var_name
    );

    assert!(
        !var_name.starts_with("CARGO_"),
        "Environment variable name '{}' is reserved (cannot start with 'CARGO_')",
        var_name
    );

    let formatted_value = escape_newlines(value); // Escape newlines
    println!("cargo:rustc-env={}={}", var_name, formatted_value);
}

/// Sets an environment variable for Cargo with auto-indented multi-line content.
///
/// This function applies `auto_indent` to normalize indentation before
/// setting the environment variable, ensuring consistent formatting.
///
/// # Arguments
/// * `var_name` - The name of the environment variable.
/// * `value` - The multi-line string value to be auto-indented and assigned.
///
/// # Behavior
/// - Ensures proper indentation is maintained when storing multi-line values.
/// - Calls `set_cargo_env_var` after processing the string.
pub fn set_multi_line_cargo_env_var(var_name: &str, value: &str) {
    let auto_indented_value = auto_indent(value);

    set_cargo_env_var(var_name, auto_indented_value.as_str());
}

/// Validates if an environment variable name follows Cargo and POSIX conventions.
///
/// - Must start with an ASCII letter (`A-Z` or `a-z`).
/// - Can contain uppercase letters (`A-Z`), digits (`0-9`), and underscores (`_`).
/// - Cannot contain spaces, special characters, or start with a digit.
///
/// # Returns
/// `true` if the name is valid, `false` otherwise.
fn is_valid_env_var_name(name: &str) -> bool {
    let mut chars = name.chars();

    // First character must be a letter (A-Z or a-z)
    match chars.next() {
        Some(c) if c.is_ascii_alphabetic() => (),
        _ => return false,
    }

    // Remaining characters must be A-Z, 0-9, or _
    chars.all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_')
}
