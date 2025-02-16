use cargo_pkg_info_struct_builder::inject_build_metadata;
use std::path::Path;

fn main() {
    inject_build_metadata(Path::new("src").join("cargo_pkg_info.rs").to_path_buf());
}
