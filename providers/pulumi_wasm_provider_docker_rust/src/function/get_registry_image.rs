//! Reads the image metadata from a Docker Registry. Used in conjunction with the docker.RemoteImage resource to keep an image up to date on the latest available version of the tag.
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ```rust
//! use pulumi_wasm_rust::Output;
//! use pulumi_wasm_rust::{add_export, pulumi_main};
//! #[pulumi_main]
//! fn test_main() -> Result<(), Error> {
//!     let ubuntuRemoteImage = remote_image::create(
//!         "ubuntuRemoteImage",
//!         RemoteImageArgs::builder()
//!             .name("${ubuntuRegistryImage.name}")
//!             .pullTriggers(vec!["${ubuntuRegistryImage.sha256Digest}",])
//!             .build_struct(),
//!     );
//! }
//! ```
//! <!--End PulumiCodeChooser -->

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct GetRegistryImageArgs {
    /// If `true`, the verification of TLS certificates of the server/registry is disabled. Defaults to `false`
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub insecure_skip_verify: pulumi_wasm_rust::Output<Option<bool>>,
    /// The name of the Docker image, including any tags. e.g. `alpine:latest`
    #[builder(into)]
    pub name: pulumi_wasm_rust::Output<String>,
}

pub struct GetRegistryImageResult {
    /// The provider-assigned unique ID for this managed resource.
    pub id: pulumi_wasm_rust::Output<String>,
    /// If `true`, the verification of TLS certificates of the server/registry is disabled. Defaults to `false`
    pub insecure_skip_verify: pulumi_wasm_rust::Output<Option<bool>>,
    /// The name of the Docker image, including any tags. e.g. `alpine:latest`
    pub name: pulumi_wasm_rust::Output<String>,
    /// The content digest of the image, as stored in the registry.
    pub sha256_digest: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn invoke(
    args: GetRegistryImageArgs
) -> GetRegistryImageResult {

    let result = crate::bindings::pulumi::docker::get_registry_image::invoke(
        &crate::bindings::pulumi::docker::get_registry_image::Args {
                insecure_skip_verify: &args.insecure_skip_verify.get_inner(),
                name: &args.name.get_inner(),
        }
    );

    GetRegistryImageResult {
        id: crate::into_domain(result.id),
        insecure_skip_verify: crate::into_domain(result.insecure_skip_verify),
        name: crate::into_domain(result.name),
        sha256_digest: crate::into_domain(result.sha256_digest),
    }
}
