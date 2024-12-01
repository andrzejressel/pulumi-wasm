//! <!-- Bug: Type and Name are switched -->
//! Pulls a Docker image to a given Docker host from a Docker Registry.
//!  This resource will *not* pull new layers of the image automatically unless used in conjunction with docker.RegistryImage data source to update the `pull_triggers` field.
//! 
//! ## Example Usage
//! 
//! ### Basic
//! 
//! Finds and downloads the latest `ubuntu:precise` image but does not check
//! for further updates of the image
//! 
//! <!--Start PulumiCodeChooser -->
//! ```rust
//! use pulumi_wasm_rust::Output;
//! use pulumi_wasm_rust::{add_export, pulumi_main};
//! #[pulumi_main]
//! fn test_main() -> Result<(), Error> {
//!     let ubuntu = remote_image::create(
//!         "ubuntu",
//!         RemoteImageArgs::builder().name("ubuntu:precise").build_struct(),
//!     );
//! }
//! ```
//! <!--End PulumiCodeChooser -->
//! 
//! ### Dynamic updates
//! 
//! To be able to update an image dynamically when the `sha256` sum changes,
//! you need to use it in combination with `docker.RegistryImage` as follows:
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
//! 
//! ### Build
//! 
//! You can also use the resource to build an image.
//! In this case the image "zoo" and "zoo:develop" are built.
//! 
//! <!--Start PulumiCodeChooser -->
//! ```yaml
//! resources:
//!   zoo:
//!     type: docker:RemoteImage
//!     properties:
//!       name: zoo
//!       build:
//!         context: .
//!         tags:
//!           - zoo:develop
//!         buildArg:
//!           foo: zoo
//!         label:
//!           author: zoo
//! ```
//! <!--End PulumiCodeChooser -->
//! 
//! You can use the `triggers` argument to specify when the image should be rebuild. This is for example helpful when you want to rebuild the docker image whenever the source code changes.
//! 

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct RemoteImageArgs {
    /// Configuration to build an image. Please see [docker build command reference](https://docs.docker.com/engine/reference/commandline/build/#options) too.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub build: pulumi_wasm_rust::Output<Option<crate::types::RemoteImageBuild>>,
    /// Always remove intermediate containers
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub force_remove: pulumi_wasm_rust::Output<Option<bool>>,
    /// If true, then the Docker image won't be deleted on destroy operation. If this is false, it will delete the image from the docker local storage on destroy operation.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub keep_locally: pulumi_wasm_rust::Output<Option<bool>>,
    /// type of ulimit, e.g. `nofile`
    #[builder(into)]
    pub name: pulumi_wasm_rust::Output<String>,
    /// Set platform if server is multi-platform capable
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub platform: pulumi_wasm_rust::Output<Option<String>>,
    /// List of values which cause an image pull when changed. This is used to store the image digest from the registry when using the docker*registry*image.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub pull_triggers: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    /// A map of arbitrary strings that, when changed, will force the `docker.RemoteImage` resource to be replaced. This can be used to rebuild an image when contents of source code folders change
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub triggers: pulumi_wasm_rust::Output<Option<std::collections::HashMap<String, String>>>,
}

pub struct RemoteImageResult {
    /// Configuration to build an image. Please see [docker build command reference](https://docs.docker.com/engine/reference/commandline/build/#options) too.
    pub build: pulumi_wasm_rust::Output<Option<crate::types::RemoteImageBuild>>,
    /// Always remove intermediate containers
    pub force_remove: pulumi_wasm_rust::Output<Option<bool>>,
    /// The ID of the image (as seen when executing `docker inspect` on the image). Can be used to reference the image via its ID in other resources.
    pub image_id: pulumi_wasm_rust::Output<String>,
    /// If true, then the Docker image won't be deleted on destroy operation. If this is false, it will delete the image from the docker local storage on destroy operation.
    pub keep_locally: pulumi_wasm_rust::Output<Option<bool>>,
    /// type of ulimit, e.g. `nofile`
    pub name: pulumi_wasm_rust::Output<String>,
    /// Set platform if server is multi-platform capable
    pub platform: pulumi_wasm_rust::Output<Option<String>>,
    /// List of values which cause an image pull when changed. This is used to store the image digest from the registry when using the docker*registry*image.
    pub pull_triggers: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    /// The image sha256 digest in the form of `repo[:tag]@sha256:<hash>`.
    pub repo_digest: pulumi_wasm_rust::Output<String>,
    /// A map of arbitrary strings that, when changed, will force the `docker.RemoteImage` resource to be replaced. This can be used to rebuild an image when contents of source code folders change
    pub triggers: pulumi_wasm_rust::Output<Option<std::collections::HashMap<String, String>>>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: RemoteImageArgs) -> RemoteImageResult {

    let result = crate::bindings::pulumi::docker::remote_image::invoke(name, &crate::bindings::pulumi::docker::remote_image::Args {
        build: &args.build.get_inner(),
        force_remove: &args.force_remove.get_inner(),
        keep_locally: &args.keep_locally.get_inner(),
        name: &args.name.get_inner(),
        platform: &args.platform.get_inner(),
        pull_triggers: &args.pull_triggers.get_inner(),
        triggers: &args.triggers.get_inner(),
    });

    RemoteImageResult {
        build: crate::into_domain(result.build),
        force_remove: crate::into_domain(result.force_remove),
        image_id: crate::into_domain(result.image_id),
        keep_locally: crate::into_domain(result.keep_locally),
        name: crate::into_domain(result.name),
        platform: crate::into_domain(result.platform),
        pull_triggers: crate::into_domain(result.pull_triggers),
        repo_digest: crate::into_domain(result.repo_digest),
        triggers: crate::into_domain(result.triggers),
    }
}
