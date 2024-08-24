//! <!-- Bug: Type and Name are switched -->
//! Manages the lifecycle of docker image in a registry. You can upload images to a registry (= `docker push`) and also delete them again
//!
//! ## Example Usage
//!
//! Build an image with the `docker.RemoteImage` resource and then push it to a registry:
//!
//! <!--Start PulumiCodeChooser -->
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as docker from "@pulumi/docker";
//!
//! const helloworld = new docker.RegistryImage("helloworld", {keepRemotely: true});
//! const image = new docker.RemoteImage("image", {
//!     name: "registry.com/somename:1.0",
//!     build: {
//!         context: `${path.cwd}/absolutePathToContextFolder`,
//!     },
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_docker as docker
//!
//! helloworld = docker.RegistryImage("helloworld", keep_remotely=True)
//! image = docker.RemoteImage("image",
//!     name="registry.com/somename:1.0",
//!     build=docker.RemoteImageBuildArgs(
//!         context=f"{path['cwd']}/absolutePathToContextFolder",
//!     ))
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
//!     var helloworld = new Docker.RegistryImage("helloworld", new()
//!     {
//!         KeepRemotely = true,
//!     });
//!
//!     var image = new Docker.RemoteImage("image", new()
//!     {
//!         Name = "registry.com/somename:1.0",
//!         Build = new Docker.Inputs.RemoteImageBuildArgs
//!         {
//!             Context = $"{path.Cwd}/absolutePathToContextFolder",
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
//! 	"fmt"
//!
//! 	"github.com/pulumi/pulumi-docker/sdk/v4/go/docker"
//! 	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
//! )
//!
//! func main() {
//! 	pulumi.Run(func(ctx *pulumi.Context) error {
//! 		_, err := docker.NewRegistryImage(ctx, "helloworld", &docker.RegistryImageArgs{
//! 			KeepRemotely: pulumi.Bool(true),
//! 		})
//! 		if err != nil {
//! 			return err
//! 		}
//! 		_, err = docker.NewRemoteImage(ctx, "image", &docker.RemoteImageArgs{
//! 			Name: pulumi.String("registry.com/somename:1.0"),
//! 			Build: &docker.RemoteImageBuildArgs{
//! 				Context: pulumi.String(fmt.Sprintf("%v/absolutePathToContextFolder", path.Cwd)),
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
//! import com.pulumi.docker.RegistryImage;
//! import com.pulumi.docker.RegistryImageArgs;
//! import com.pulumi.docker.RemoteImage;
//! import com.pulumi.docker.RemoteImageArgs;
//! import com.pulumi.docker.inputs.RemoteImageBuildArgs;
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
//!         var helloworld = new RegistryImage("helloworld", RegistryImageArgs.builder()        
//!             .keepRemotely(true)
//!             .build());
//!
//!         var image = new RemoteImage("image", RemoteImageArgs.builder()        
//!             .name("registry.com/somename:1.0")
//!             .build(RemoteImageBuildArgs.builder()
//!                 .context(String.format("%s/absolutePathToContextFolder", path.cwd()))
//!                 .build())
//!             .build());
//!
//!     }
//! }
//! ```
//! ### YAML
//! ```yaml
//! resources:
//!   helloworld:
//!     type: docker:RegistryImage
//!     properties:
//!       keepRemotely: true
//!   image:
//!     type: docker:RemoteImage
//!     properties:
//!       name: registry.com/somename:1.0
//!       build:
//!         context: ${path.cwd}/absolutePathToContextFolder
//! ```
//! <!--End PulumiCodeChooser -->

pub struct RegistryImageArgs {
    /// If `true`, the verification of TLS certificates of the server/registry is disabled. Defaults to `false`
    pub insecure_skip_verify: pulumi_wasm_rust::Output<Option<bool>>,
    /// If true, then the Docker image won't be deleted on destroy operation. If this is false, it will delete the image from the docker registry on destroy operation. Defaults to `false`
    pub keep_remotely: pulumi_wasm_rust::Output<Option<bool>>,
    /// The name of the Docker image.
    pub name: pulumi_wasm_rust::Output<Option<String>>,
    /// A map of arbitrary strings that, when changed, will force the `docker.RegistryImage` resource to be replaced. This can be used to repush a local image
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
    let result = crate::bindings::pulumi::docker::registry_image::invoke(
        name,
        &crate::bindings::pulumi::docker::registry_image::Args {
            insecure_skip_verify: args.insecure_skip_verify.get_inner(),
            keep_remotely: args.keep_remotely.get_inner(),
            name: args.name.get_inner(),
            triggers: args.triggers.get_inner(),
        },
    );

    RegistryImageResult {
        insecure_skip_verify: crate::into_domain(result.insecure_skip_verify),
        keep_remotely: crate::into_domain(result.keep_remotely),
        name: crate::into_domain(result.name),
        sha256_digest: crate::into_domain(result.sha256_digest),
        triggers: crate::into_domain(result.triggers),
    }
}
