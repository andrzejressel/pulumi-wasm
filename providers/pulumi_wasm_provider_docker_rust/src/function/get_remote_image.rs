//! `docker.RemoteImage` provides details about a specific Docker Image which needs to be present on the Docker Host
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as docker from "@pulumi/docker";
//! 
//! const latest = docker.getRemoteImage({
//!     name: "nginx",
//! });
//! const specific = docker.getRemoteImage({
//!     name: "nginx:1.17.6",
//! });
//! const digest = docker.getRemoteImage({
//!     name: "nginx@sha256:36b74457bccb56fbf8b05f79c85569501b721d4db813b684391d63e02287c0b2",
//! });
//! const tagAndDigest = docker.getRemoteImage({
//!     name: "nginx:1.19.1@sha256:36b74457bccb56fbf8b05f79c85569501b721d4db813b684391d63e02287c0b2",
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_docker as docker
//! 
//! latest = docker.get_remote_image(name="nginx")
//! specific = docker.get_remote_image(name="nginx:1.17.6")
//! digest = docker.get_remote_image(name="nginx@sha256:36b74457bccb56fbf8b05f79c85569501b721d4db813b684391d63e02287c0b2")
//! tag_and_digest = docker.get_remote_image(name="nginx:1.19.1@sha256:36b74457bccb56fbf8b05f79c85569501b721d4db813b684391d63e02287c0b2")
//! ```
//! ### C#
//! ```csharp
//! using System.Collections.Generic;
//! using System.Linq;
//! using Pulumi;
//! using Docker = Pulumi.Docker;
//! 
//! return await Deployment.RunAsync(() => 
//! {
//!     var latest = Docker.GetRemoteImage.Invoke(new()
//!     {
//!         Name = "nginx",
//!     });
//! 
//!     var specific = Docker.GetRemoteImage.Invoke(new()
//!     {
//!         Name = "nginx:1.17.6",
//!     });
//! 
//!     var digest = Docker.GetRemoteImage.Invoke(new()
//!     {
//!         Name = "nginx@sha256:36b74457bccb56fbf8b05f79c85569501b721d4db813b684391d63e02287c0b2",
//!     });
//! 
//!     var tagAndDigest = Docker.GetRemoteImage.Invoke(new()
//!     {
//!         Name = "nginx:1.19.1@sha256:36b74457bccb56fbf8b05f79c85569501b721d4db813b684391d63e02287c0b2",
//!     });
//! 
//! });
//! ```
//! ### Go
//! ```go
//! package main
//! 
//! import (
//! 	"github.com/pulumi/pulumi-docker/sdk/v4/go/docker"
//! 	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
//! )
//! 
//! func main() {
//! 	pulumi.Run(func(ctx *pulumi.Context) error {
//! 		_, err := docker.LookupRemoteImage(ctx, &docker.LookupRemoteImageArgs{
//! 			Name: "nginx",
//! 		}, nil)
//! 		if err != nil {
//! 			return err
//! 		}
//! 		_, err = docker.LookupRemoteImage(ctx, &docker.LookupRemoteImageArgs{
//! 			Name: "nginx:1.17.6",
//! 		}, nil)
//! 		if err != nil {
//! 			return err
//! 		}
//! 		_, err = docker.LookupRemoteImage(ctx, &docker.LookupRemoteImageArgs{
//! 			Name: "nginx@sha256:36b74457bccb56fbf8b05f79c85569501b721d4db813b684391d63e02287c0b2",
//! 		}, nil)
//! 		if err != nil {
//! 			return err
//! 		}
//! 		_, err = docker.LookupRemoteImage(ctx, &docker.LookupRemoteImageArgs{
//! 			Name: "nginx:1.19.1@sha256:36b74457bccb56fbf8b05f79c85569501b721d4db813b684391d63e02287c0b2",
//! 		}, nil)
//! 		if err != nil {
//! 			return err
//! 		}
//! 		return nil
//! 	})
//! }
//! ```
//! ### Java
//! ```java
//! package generated_program;
//! 
//! import com.pulumi.Context;
//! import com.pulumi.Pulumi;
//! import com.pulumi.core.Output;
//! import com.pulumi.docker.DockerFunctions;
//! import com.pulumi.docker.inputs.GetRemoteImageArgs;
//! import java.util.List;
//! import java.util.ArrayList;
//! import java.util.Map;
//! import java.io.File;
//! import java.nio.file.Files;
//! import java.nio.file.Paths;
//! 
//! public class App {
//!     public static void main(String[] args) {
//!         Pulumi.run(App::stack);
//!     }
//! 
//!     public static void stack(Context ctx) {
//!         final var latest = DockerFunctions.getRemoteImage(GetRemoteImageArgs.builder()
//!             .name("nginx")
//!             .build());
//! 
//!         final var specific = DockerFunctions.getRemoteImage(GetRemoteImageArgs.builder()
//!             .name("nginx:1.17.6")
//!             .build());
//! 
//!         final var digest = DockerFunctions.getRemoteImage(GetRemoteImageArgs.builder()
//!             .name("nginx@sha256:36b74457bccb56fbf8b05f79c85569501b721d4db813b684391d63e02287c0b2")
//!             .build());
//! 
//!         final var tagAndDigest = DockerFunctions.getRemoteImage(GetRemoteImageArgs.builder()
//!             .name("nginx:1.19.1@sha256:36b74457bccb56fbf8b05f79c85569501b721d4db813b684391d63e02287c0b2")
//!             .build());
//! 
//!     }
//! }
//! ```
//! ### YAML
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
