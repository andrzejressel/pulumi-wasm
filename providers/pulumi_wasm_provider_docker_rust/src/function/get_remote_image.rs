//! `docker.RemoteImage` provides details about a specific Docker Image which needs to be present on the Docker Host
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ```yaml
//! variables:
//!   latest:
//!     fn::invoke:
//!       Function: docker:getRemoteImage
//!       Arguments:
//!         name: nginx
//!   specific:
//!     fn::invoke:
//!       Function: docker:getRemoteImage
//!       Arguments:
//!         name: nginx:1.17.6
//!   digest:
//!     fn::invoke:
//!       Function: docker:getRemoteImage
//!       Arguments:
//!         name: nginx@sha256:36b74457bccb56fbf8b05f79c85569501b721d4db813b684391d63e02287c0b2
//!   tagAndDigest:
//!     fn::invoke:
//!       Function: docker:getRemoteImage
//!       Arguments:
//!         name: nginx:1.19.1@sha256:36b74457bccb56fbf8b05f79c85569501b721d4db813b684391d63e02287c0b2
//! ```
//! <!--End PulumiCodeChooser -->

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct GetRemoteImageArgs {
    /// The name of the Docker image, including any tags or SHA256 repo digests.
    #[builder(into)]
    pub name: pulumi_wasm_rust::Output<String>,
}

pub struct GetRemoteImageResult {
    /// The provider-assigned unique ID for this managed resource.
    pub id: pulumi_wasm_rust::Output<String>,
    /// The name of the Docker image, including any tags or SHA256 repo digests.
    pub name: pulumi_wasm_rust::Output<String>,
    /// The image sha256 digest in the form of `repo[:tag]@sha256:<hash>`. It may be empty in the edge case where the local image was pulled from a repo, tagged locally, and then referred to in the data source by that local name/tag.
    pub repo_digest: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn invoke(
    args: GetRemoteImageArgs
) -> GetRemoteImageResult {

    let result = crate::bindings::pulumi::docker::get_remote_image::invoke(
        &crate::bindings::pulumi::docker::get_remote_image::Args {
                name: &args.name.get_inner(),
        }
    );

    GetRemoteImageResult {
        id: crate::into_domain(result.id),
        name: crate::into_domain(result.name),
        repo_digest: crate::into_domain(result.repo_digest),
    }
}
