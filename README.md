
# cargo-pkg-info-struct-builder

[![made-with-rust][rust-logo]][rust-src-page]
[![crates.io][crates-badge]][crates-page]
[![Documentation][docs-badge]][docs-page]
[![MIT licensed][license-badge]][license-page]


| OS            | Status                                                                               |
|---------------|--------------------------------------------------------------------------------------|
| Ubuntu-latest | [![Ubuntu Tests][ubuntu-latest-badge]][ubuntu-latest-workflow]                       |
| macOS-latest  | [![macOS Tests][macos-latest-badge]][macos-latest-workflow]                          |
| Windows-latest| [![Windows Tests][windows-latest-badge]][windows-latest-workflow]                    |


A build-time crate that generates a struct (`CargoPkgInfo`) to provide easy, structured access to your Rust project’s compile-time Cargo environment variables.

## Available Metadata Methods

Once generated, the `CargoPkgInfo` struct provides access to the following metadata:

| **Method**                              | **Description**                                          |
|-----------------------------------------|----------------------------------------------------------|
| `CargoPkgInfo::pkg_name()`              | Package name -> `Option<&'static str>`                   |
| `CargoPkgInfo::crate_name()`            | Crate name -> `Option<&'static str>`                     |
| `CargoPkgInfo::pkg_version()`           | Full version -> `Option<&'static str>`                   | 
| `CargoPkgInfo::version_major()`         | Major version -> `Option<&'static str>`                  |
| `CargoPkgInfo::version_major_numeric()` | Major version -> `Option<u32>`                           |
| `CargoPkgInfo::version_minor()`         | Minor version -> `Option<&'static str>`                  |
| `CargoPkgInfo::version_minor_numeric()` | Minor version -> `Option<u32>`                           |
| `CargoPkgInfo::version_patch()`         | Patch version -> `Option<&'static str>`                  |
| `CargoPkgInfo::version_patch_numeric()` | Patch version -> `Option<u32>`                           |
| `CargoPkgInfo::version_pre()`           | Pre-release version -> `Option<&'static str>`            |
| `CargoPkgInfo::authors()`               | Authors -> `Option<&'static str>`                        |
| `CargoPkgInfo::description()`           | Description -> `Option<&'static str>`                    |
| `CargoPkgInfo::homepage()`              | Homepage URL -> `Option<&'static str>`                   |
| `CargoPkgInfo::repository()`            | Repository URL -> `Option<&'static str>`                 |
| `CargoPkgInfo::license()`               | License name -> `Option<&'static str>`                   |
| `CargoPkgInfo::license_content()`       | Full license text -> `Option<&'static str>`              |
| `CargoPkgInfo::rust_version()`          | Required Rust version -> `Option<&'static str>`          |
| `CargoPkgInfo::readme_path()`           | Path to README file -> `Option<&'static str>`            |
| `CargoPkgInfo::build_target()`          | Compilation target -> `Option<&'static str>`             |
| `CargoPkgInfo::build_time_utc()`        | Build timestamp UTC -> `Option<u64>`                     |

---

## Overview

At compile time, Cargo sets various environment variables describing your project (version, authors, license, etc.). `cargo-pkg-info-struct-builder` gathers those variables from your project (not this library) and creates a file, at the path of your choosing, containing a `CargoPkgInfo` struct.

This struct provides simple, typed methods to retrieve metadata like:

- Version (major, minor, patch, pre-release)
- Authors, description, homepage, and repository
- License information (including file contents)
- Build target and a compile-time build timestamp

Because this happens during the project’s build process, you get project-wide metadata embedded in the final binary—no additional steps required.

## Quick Start

Add it as a **build dependency**:

```sh
cargo add cargo-pkg-info-struct-builder --build
```

Then, in your **`build.rs`**:

```rust
use cargo_pkg_info_struct_builder::inject_build_metadata;
use std::path::Path;

fn main() {
    // Generate metadata into a Rust file.
    // You can change the path of your choosing, and directories are
    // auto-generated if they do not already exist.
    let dest_path = Path::new("src").join("cargo_pkg_info.rs");
    inject_build_metadata(dest_path.to_path_buf());
}
```

Now, in your main application:

```rust
// Assuming the `dest_path` is used from the above example.
mod cargo_pkg_info;
use cargo_pkg_info::CargoPkgInfo;

fn main() {
    println!(
        "{} v{} (Built on {})",
        CargoPkgInfo::pkg_name(),
        CargoPkgInfo::pkg_version(),
        CargoPkgInfo::build_time_utc().unwrap_or(0),
    );
}
```

**This embeds project metadata into your binary at compile time—runtime environment variables are not used.**

## Why Compile-Time Injection?

Unlike crates that retrieve package metadata **at runtime**, this crate:

- **Embeds metadata at compile time** to avoid file I/O in production.
- **Captures the metadata of your project**, not the dependency itself.
- **Works with any Rust project** (including WASM) without requiring Cargo to be installed at runtime.

## How It Works

When `build.rs` runs:

1. It reads package metadata from Cargo environment variables.
2. It generates a Rust source file (`cargo_pkg_info.rs`) containing a struct with methods that access this metadata.
3. The file is compiled into your project, allowing **runtime access to compiled environment info.**

This is ideal for **logging, debugging, and version tracking** in Rust applications.

## Notes

The generated file **can be committed to version control**, but it will remain **unchanged unless the template itself is modified.** Metadata updates do not change the file itself.

## License
Licensed under **MIT**. See [`LICENSE`][license-page] for details.


[rust-src-page]: https://www.rust-lang.org/
[rust-logo]: https://img.shields.io/badge/Made%20with-Rust-black?&logo=Rust

[crates-page]: https://crates.io/crates/cargo-pkg-info-struct-builder
[crates-badge]: https://img.shields.io/crates/v/cargo-pkg-info-struct-builder.svg

[docs-page]: https://docs.rs/cargo-pkg-info-struct-builder
[docs-badge]: https://docs.rs/cargo-pkg-info-struct-builder/badge.svg

[license-page]: https://github.com/jzombie/rust-cargo-pkg-info-struct-builder/blob/main/LICENSE
[license-badge]: https://img.shields.io/badge/license-MIT-blue.svg

[ubuntu-latest-badge]: https://github.com/jzombie/rust-cargo-pkg-info-struct-builder/actions/workflows/rust-tests.yml/badge.svg?branch=main&job=Run%20Rust%20Tests%20(OS%20=%20ubuntu-latest)
[ubuntu-latest-workflow]: https://github.com/jzombie/rust-cargo-pkg-info-struct-builder/actions/workflows/rust-tests.yml?query=branch%3Amain

[macos-latest-badge]: https://github.com/jzombie/rust-cargo-pkg-info-struct-builder/actions/workflows/rust-tests.yml/badge.svg?branch=main&job=Run%20Rust%20Tests%20(OS%20=%20macos-latest)
[macos-latest-workflow]: https://github.com/jzombie/rust-cargo-pkg-info-struct-builder/actions/workflows/rust-tests.yml?query=branch%3Amain

[windows-latest-badge]: https://github.com/jzombie/rust-cargo-pkg-info-struct-builder/actions/workflows/rust-tests.yml/badge.svg?branch=main&job=Run%20Rust%20Tests%20(OS%20=%20windows-latest)
[windows-latest-workflow]: https://github.com/jzombie/rust-cargo-pkg-info-struct-builder/actions/workflows/rust-tests.yml?query=branch%3Amain
