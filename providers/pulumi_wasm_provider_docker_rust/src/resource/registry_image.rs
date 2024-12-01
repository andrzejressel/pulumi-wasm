//! <!-- Bug: Type and Name are switched -->
//! Manages the lifecycle of docker image in a registry. You can upload images to a registry (= `docker push`) and also delete them again
//! 
//! ## Example Usage
//! 
//! Build an image with the `docker.RemoteImage` resource and then push it to a registry:
//! 
//! <!--Start PulumiCodeChooser -->
//! ```ignore
//! use pulumi_wasm_rust::Output;
//! use pulumi_wasm_rust::{add_export, pulumi_main};
//! #[pulumi_main]
//! fn test_main() -> Result<(), Error> {
//!     let helloworld = registry_image::create(
//!         "helloworld",
//!         RegistryImageArgs::builder().keep_remotely(true).build_struct(),
//!     );
//!     let image = remote_image::create(
//!         "image",
//!         RemoteImageArgs::builder()
//!             .build(
//!                 RemoteImageBuild::builder()
//!                     .context("${path.cwd}/absolutePathToContextFolder")
//!                     .build_struct(),
//!             )
//!             .name("registry.com/somename:1.0")
//!             .build_struct(),
//!     );
//! }
//! ```
//! <!--End PulumiCodeChooser -->

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct RegistryImageArgs {
    /// If `true`, the verification of TLS certificates of the server/registry is disabled. Defaults to `false`
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub insecure_skip_verify: pulumi_wasm_rust::Output<Option<bool>>,
    /// If true, then the Docker image won't be deleted on destroy operation. If this is false, it will delete the image from the docker registry on destroy operation. Defaults to `false`
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub keep_remotely: pulumi_wasm_rust::Output<Option<bool>>,
    /// The name of the Docker image.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub name: pulumi_wasm_rust::Output<Option<String>>,
    /// A map of arbitrary strings that, when changed, will force the `docker.RegistryImage` resource to be replaced. This can be used to repush a local image
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub triggers: pulumi_wasm_rust::Output<Option<std::collections::HashMap<String, String>>>,
}

pub struct RegistryImageResult {
    /// If `true`, the verification of TLS certificates of the server/registry is disabled. Defaults to `false`
    pub insecure_skip_verify: pulumi_wasm_rust::Output<Option<bool>>,
    /// If true, then the Docker image won't be deleted on destroy operation. If this is false, it will delete the image from the docker registry on destroy operation. Defaults to `false`
    pub keep_remotely: pulumi_wasm_rust::Output<Option<bool>>,
    /// The name of the Docker image.
    pub name: pulumi_wasm_rust::Output<String>,
    /// The sha256 digest of the image.
    pub sha256_digest: pulumi_wasm_rust::Output<String>,
    /// A map of arbitrary strings that, when changed, will force the `docker.RegistryImage` resource to be replaced. This can be used to repush a local image
    pub triggers: pulumi_wasm_rust::Output<Option<std::collections::HashMap<String, String>>>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: RegistryImageArgs) -> RegistryImageResult {

    let result = crate::bindings::pulumi::docker::registry_image::invoke(name, &crate::bindings::pulumi::docker::registry_image::Args {
        insecure_skip_verify: &args.insecure_skip_verify.get_inner(),
        keep_remotely: &args.keep_remotely.get_inner(),
        name: &args.name.get_inner(),
        triggers: &args.triggers.get_inner(),
    });

    RegistryImageResult {
        insecure_skip_verify: crate::into_domain(result.insecure_skip_verify),
        keep_remotely: crate::into_domain(result.keep_remotely),
        name: crate::into_domain(result.name),
        sha256_digest: crate::into_domain(result.sha256_digest),
        triggers: crate::into_domain(result.triggers),
    }
}
