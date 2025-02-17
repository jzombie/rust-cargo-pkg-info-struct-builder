use cargo_pkg_info_struct_builder::{inject_build_metadata, set_cargo_env_var};
use std::path::Path;
use string_replace_all::string_replace_all;

fn main() {
    set_cargo_env_var("TEST_CUSTOM_ENV_VAR", "TEST_RESULT");
    set_cargo_env_var(
        "TEST_MULTI_LINE_CUSTOM_ENV_VAR",
        &string_replace_all(
            r#"
        Some multi-line environment variable
        Testing 1 2 3
        "#,
            "  ",
            " ",
        ),
    );

    inject_build_metadata(Path::new("src").join("cargo_pkg_info.rs").to_path_buf());
}
