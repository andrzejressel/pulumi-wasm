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
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as docker from "@pulumi/docker";
//! 
//! const ubuntu = new docker.RemoteImage("ubuntu", {name: "ubuntu:precise"});
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_docker as docker
//! 
//! ubuntu = docker.RemoteImage("ubuntu", name="ubuntu:precise")
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
//!     var ubuntu = new Docker.RemoteImage("ubuntu", new()
//!     {
//!         Name = "ubuntu:precise",
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
//! 		_, err := docker.NewRemoteImage(ctx, "ubuntu", &docker.RemoteImageArgs{
//! 			Name: pulumi.String("ubuntu:precise"),
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
//!         var ubuntu = new RemoteImage("ubuntu", RemoteImageArgs.builder()        
//!             .name("ubuntu:precise")
//!             .build());
//! 
//!     }
//! }
//! ```
//! ### YAML
//! ```yaml
//! resources:
//!   ubuntu:
//!     type: docker:RemoteImage
//!     properties:
//!       name: ubuntu:precise
//! ```
//! <!--End PulumiCodeChooser -->
//! 
//! ### Dynamic updates
//! 
//! To be able to update an image dynamically when the `sha256` sum changes,
//! you need to use it in combination with `docker.RegistryImage` as follows:
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
//! 
//! ### Build
//! 
//! You can also use the resource to build an image.
//! In this case the image "zoo" and "zoo:develop" are built.
//! 
//! <!--Start PulumiCodeChooser -->
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as docker from "@pulumi/docker";
//! 
//! const zoo = new docker.RemoteImage("zoo", {
//!     name: "zoo",
//!     build: {
//!         context: ".",
//!         tags: ["zoo:develop"],
//!         buildArg: {
//!             foo: "zoo",
//!         },
//!         label: {
//!             author: "zoo",
//!         },
//!     },
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_docker as docker
//! 
//! zoo = docker.RemoteImage("zoo",
//!     name="zoo",
//!     build=docker.RemoteImageBuildArgs(
//!         context=".",
//!         tags=["zoo:develop"],
//!         build_arg={
//!             "foo": "zoo",
//!         },
//!         label={
//!             "author": "zoo",
//!         },
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
//!     var zoo = new Docker.RemoteImage("zoo", new()
//!     {
//!         Name = "zoo",
//!         Build = new Docker.Inputs.RemoteImageBuildArgs
//!         {
//!             Context = ".",
//!             Tags = new[]
//!             {
//!                 "zoo:develop",
//!             },
//!             BuildArg = 
//!             {
//!                 { "foo", "zoo" },
//!             },
//!             Label = 
//!             {
//!                 { "author", "zoo" },
//!             },
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
//! 		_, err := docker.NewRemoteImage(ctx, "zoo", &docker.RemoteImageArgs{
//! 			Name: pulumi.String("zoo"),
//! 			Build: &docker.RemoteImageBuildArgs{
//! 				Context: pulumi.String("."),
//! 				Tags: pulumi.StringArray{
//! 					pulumi.String("zoo:develop"),
//! 				},
//! 				BuildArg: pulumi.StringMap{
//! 					"foo": pulumi.String("zoo"),
//! 				},
//! 				Label: pulumi.StringMap{
//! 					"author": pulumi.String("zoo"),
//! 				},
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
//!         var zoo = new RemoteImage("zoo", RemoteImageArgs.builder()        
//!             .name("zoo")
//!             .build(RemoteImageBuildArgs.builder()
//!                 .context(".")
//!                 .tags("zoo:develop")
//!                 .buildArg(Map.of("foo", "zoo"))
//!                 .label(Map.of("author", "zoo"))
//!                 .build())
//!             .build());
//! 
//!     }
//! }
//! ```
//! ### YAML
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
