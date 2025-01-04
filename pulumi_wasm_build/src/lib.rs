//! `pulumi_wasm_build` generates glue code for given Pulumi provider
//! ## Example
//! Select Pulumi provider you want to use. For demonstration purposes I'll choose [Random](https://www.pulumi.com/registry/packages/random/)
//!
//! Here we will have the provider glue and our code in single library. Due to compilation times
//! it is recommended to separate glue code and custom code.
//! ```bash
//! $ cargo new --lib random && cd random
//! ```
//!
//! First, add `pulumi_wasm_build`, `bon`, `serde`, `anyhow` and `wit-bindgen` to `Cargo.toml`:
//!
//! ```bash
//! $ cargo add --build pulumi_wasm_build
//! $ cargo add bon
//! $ cargo add serde --features derive
//! $ cargo add anyhow
//! $ cargo add wit-bindgen
//! ```
//!
//! To generate glue code, use `pulumi_wasm_build` in `build.rs`
//! ```rust,no_run
//! use std::error::Error;
//! fn main() -> Result<(), Box<dyn Error>> {
//!     pulumi_wasm_build::generate("random", "4.15.0")?;
//!     Ok(())
//! }
//! ```
//!
//! Then add generated code to `lib.rs` and use it in `#[pulumi_main]`
//! ```rust,ignore
//! mod random {
//!   include_provider!("random");
//! }
//!
//! use random::random_string::RandomStringArgs;
//! use random::random_string;
//! use pulumi_wasm_rust::pulumi_main;
//! use anyhow::Result;
//!
//! #[pulumi_main]
//! fn main() -> Result<()> {
//!   let random_string = random_string::create(
//!     "test",
//!     RandomStringArgs::builder().length(32).build_struct()
//!   );
//!   Ok(())
//! }
//! ```

use anyhow::{Context, Result};
use pulumi_wasm_generator::{extract_micro_package, generate_combined};
use std::path::Path;
use std::{env, fs};

/// Generates glue code for given provider, version and namespace. Can be included using [`pulumi_wasm_rust::include_provider!(provider_name)`]
/// Namespaces for provider can be found in Pulumi registry on left hand side with (M) icon:
/// - [AWS](https://www.pulumi.com/registry/packages/aws/)
/// - [Azure](https://www.pulumi.com/registry/packages/azure/)
/// - [GCP](https://www.pulumi.com/registry/packages/gcp/)
pub fn generate_with_filter(
    provider_name: &str,
    provider_version: &str,
    filter: &str,
) -> Result<()> {
    generate_with_optional_filter(provider_name, provider_version, Some(filter))
}

/// Generates glue code for given provider and version. Can be included using [`pulumi_wasm_rust::include_provider!(provider_name)`]
pub fn generate(provider_name: &str, provider_version: &str) -> Result<()> {
    generate_with_optional_filter(provider_name, provider_version, None)
}

/// Generates glue code for given schema json/yaml. Can be included using [`pulumi_wasm_rust::include_provider!(provider_name)`]
pub fn generate_from_schema(schema_file: &Path) -> Result<()> {
    let package = extract_micro_package(schema_file).context("Failed to deserialize package")?;
    let provider_name = package.name;

    let out_dir = env::var_os("OUT_DIR").context("Failed to get OUT_DIR environment variable")?;
    let out_dir = out_dir
        .to_str()
        .context(format!("Failed to convert [{:?}] to string", out_dir))?;
    let location = Path::new(out_dir).join("pulumi").join(provider_name);

    generate_combined(schema_file, &location, None).context("Failed to generate glue files")?;
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed={}", schema_file.display());
    Ok(())
}

fn generate_with_optional_filter(
    provider_name: &str,
    provider_version: &str,
    filter: Option<&str>,
) -> Result<()> {
    let schema_output = std::process::Command::new("pulumi")
        .arg("package")
        .arg("get-schema")
        .arg(format!("{}@{}", provider_name, provider_version))
        .output()
        .context("Failed to execute pulumi command")?;

    let schema = String::from_utf8(schema_output.stdout).expect("Invalid UTF-8 in pulumi output");

    let out_dir = env::var_os("OUT_DIR").context("Failed to get OUT_DIR environment variable")?;
    let out_dir = out_dir
        .to_str()
        .context(format!("Failed to convert [{:?}] to string", out_dir))?;

    let location = Path::new(out_dir).join("pulumi").join(provider_name);

    let temp_dir = tempfile::tempdir().context("Failed to create temporary directory")?;
    let file = temp_dir.path().join("schema.json");
    fs::write(&file, &schema).context("Failed to write schema")?;

    generate_combined(file.as_path(), &location, filter)
        .context("Failed to generate glue files")?;
    println!("cargo::rerun-if-changed=build.rs");

    Ok(())
}
