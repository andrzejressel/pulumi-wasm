//! <!-- Bug: Type and Name are switched -->
//! Creates and destroys a volume in Docker. This can be used alongside docker.Container to prepare volumes that can be shared across containers.
//! 
//! ## Example Usage
//! 
//! ```ignore
//! use pulumi_wasm_rust::Output;
//! use pulumi_wasm_rust::{add_export, pulumi_main};
//! #[pulumi_main]
//! fn test_main() -> Result<(), Error> {
//!     let sharedVolume = volume::create(
//!         "sharedVolume",
//!         VolumeArgs::builder().build_struct(),
//!     );
//! }
//! ```
//! 
//! ## Import
//! 
//! ### Example
//! 
//! Assuming you created a `volume` as follows
//! 
//! #!/bin/bash
//! 
//! docker volume create
//! 
//! prints the long ID
//! 
//! 524b0457aa2a87dd2b75c74c3e4e53f406974249e63ab3ed9bf21e5644f9dc7d
//! 
//! you provide the definition for the resource as follows
//! 
//! terraform
//! 
//! resource "docker_volume" "foo" {
//! 
//!   name = "524b0457aa2a87dd2b75c74c3e4e53f406974249e63ab3ed9bf21e5644f9dc7d"
//! 
//! }
//! 
//! then the import command is as follows
//! 
//! #!/bin/bash
//! 
//! ```sh
//! $ pulumi import docker:index/volume:Volume foo 524b0457aa2a87dd2b75c74c3e4e53f406974249e63ab3ed9bf21e5644f9dc7d
//! ```
//! 

#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct VolumeArgs {
    /// Driver type for the volume. Defaults to `local`.
    #[builder(into, default)]
    pub driver: pulumi_wasm_rust::Output<Option<String>>,
    /// Options specific to the driver.
    #[builder(into, default)]
    pub driver_opts: pulumi_wasm_rust::Output<Option<std::collections::HashMap<String, String>>>,
    /// User-defined key/value metadata
    #[builder(into, default)]
    pub labels: pulumi_wasm_rust::Output<Option<Vec<crate::types::VolumeLabel>>>,
    /// The name of the Docker volume (will be generated if not provided).
    #[builder(into, default)]
    pub name: pulumi_wasm_rust::Output<Option<String>>,
}

pub struct VolumeResult {
    /// Driver type for the volume. Defaults to `local`.
    pub driver: pulumi_wasm_rust::Output<String>,
    /// Options specific to the driver.
    pub driver_opts: pulumi_wasm_rust::Output<Option<std::collections::HashMap<String, String>>>,
    /// User-defined key/value metadata
    pub labels: pulumi_wasm_rust::Output<Option<Vec<crate::types::VolumeLabel>>>,
    /// The mountpoint of the volume.
    pub mountpoint: pulumi_wasm_rust::Output<String>,
    /// The name of the Docker volume (will be generated if not provided).
    pub name: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: VolumeArgs) -> VolumeResult {

    let result = crate::bindings::pulumi::docker::volume::invoke(name, &crate::bindings::pulumi::docker::volume::Args {
        driver: &args.driver.get_inner(),
        driver_opts: &args.driver_opts.get_inner(),
        labels: &args.labels.get_inner(),
        name: &args.name.get_inner(),
    });

    VolumeResult {
        driver: crate::into_domain(result.driver),
        driver_opts: crate::into_domain(result.driver_opts),
        labels: crate::into_domain(result.labels),
        mountpoint: crate::into_domain(result.mountpoint),
        name: crate::into_domain(result.name),
    }
}
