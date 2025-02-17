use cargo_pkg_info_struct_builder::{
    inject_build_metadata::{inject_build_metadata, set_multi_line_cargo_env_var},
    set_cargo_env_var,
};
use std::path::Path;

fn main() {
    set_cargo_env_var("TEST_CUSTOM_ENV_VAR", "TEST_RESULT");
    set_multi_line_cargo_env_var(
        "TEST_MULTI_LINE_CUSTOM_ENV_VAR",
        r#"
        Some multi-line environment variable
        
        Level 1
            Level 2
                Level 3
        "#,
    );

    inject_build_metadata(Path::new("src").join("cargo_pkg_info.rs").to_path_buf());
}
