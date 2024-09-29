//! Reads the image metadata from a Docker Registry. Used in conjunction with the docker.RemoteImage resource to keep an image up to date on the latest available version of the tag.
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as docker from "@pulumi/docker";
//! 
//! const ubuntuRegistryImage = docker.getRegistryImage({
//!     name: "ubuntu:precise",
//! });
//! const ubuntuRemoteImage = new docker.RemoteImage("ubuntuRemoteImage", {
//!     name: ubuntuRegistryImage.then(ubuntuRegistryImage => ubuntuRegistryImage.name),
//!     pullTriggers: [ubuntuRegistryImage.then(ubuntuRegistryImage => ubuntuRegistryImage.sha256Digest)],
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_docker as docker
//! 
//! ubuntu_registry_image = docker.get_registry_image(name="ubuntu:precise")
//! ubuntu_remote_image = docker.RemoteImage("ubuntuRemoteImage",
//!     name=ubuntu_registry_image.name,
//!     pull_triggers=[ubuntu_registry_image.sha256_digest])
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
//!     var ubuntuRegistryImage = Docker.GetRegistryImage.Invoke(new()
//!     {
//!         Name = "ubuntu:precise",
//!     });
//! 
//!     var ubuntuRemoteImage = new Docker.RemoteImage("ubuntuRemoteImage", new()
//!     {
//!         Name = ubuntuRegistryImage.Apply(getRegistryImageResult => getRegistryImageResult.Name),
//!         PullTriggers = new[]
//!         {
//!             ubuntuRegistryImage.Apply(getRegistryImageResult => getRegistryImageResult.Sha256Digest),
//!         },
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
//! 		ubuntuRegistryImage, err := docker.LookupRegistryImage(ctx, &docker.LookupRegistryImageArgs{
//! 			Name: "ubuntu:precise",
//! 		}, nil)
//! 		if err != nil {
//! 			return err
//! 		}
//! 		_, err = docker.NewRemoteImage(ctx, "ubuntuRemoteImage", &docker.RemoteImageArgs{
//! 			Name: pulumi.String(ubuntuRegistryImage.Name),
//! 			PullTriggers: pulumi.StringArray{
//! 				pulumi.String(ubuntuRegistryImage.Sha256Digest),
//! 			},
//! 		})
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
//! import com.pulumi.docker.inputs.GetRegistryImageArgs;
//! import com.pulumi.docker.RemoteImage;
//! import com.pulumi.docker.RemoteImageArgs;
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
//!         final var ubuntuRegistryImage = DockerFunctions.getRegistryImage(GetRegistryImageArgs.builder()
//!             .name("ubuntu:precise")
//!             .build());
//! 
//!         var ubuntuRemoteImage = new RemoteImage("ubuntuRemoteImage", RemoteImageArgs.builder()        
//!             .name(ubuntuRegistryImage.applyValue(getRegistryImageResult -> getRegistryImageResult.name()))
//!             .pullTriggers(ubuntuRegistryImage.applyValue(getRegistryImageResult -> getRegistryImageResult.sha256Digest()))
//!             .build());
//! 
//!     }
//! }
//! ```
//! ### YAML
//! ```yaml
//! resources:
//!   ubuntuRemoteImage:
//!     type: docker:RemoteImage
//!     properties:
//!       name: ${ubuntuRegistryImage.name}
//!       pullTriggers:
//!         - ${ubuntuRegistryImage.sha256Digest}
//! variables:
//!   ubuntuRegistryImage:
//!     fn::invoke:
//!       Function: docker:getRegistryImage
//!       Arguments:
//!         name: ubuntu:precise
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
