//! <!-- Bug: Type and Name are switched -->
//! Creates and destroys a volume in Docker. This can be used alongside docker.Container to prepare volumes that can be shared across containers.
//!
//! ## Example Usage
//!
//! <!--Start PulumiCodeChooser -->
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as docker from "@pulumi/docker";
//!
//! const sharedVolume = new docker.Volume("sharedVolume", {});
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_docker as docker
//!
//! shared_volume = docker.Volume("sharedVolume")
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
//!     var sharedVolume = new Docker.Volume("sharedVolume");
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
//! 		_, err := docker.NewVolume(ctx, "sharedVolume", nil)
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
//! import com.pulumi.docker.Volume;
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
//!         var sharedVolume = new Volume("sharedVolume");
//!
//!     }
//! }
//! ```
//! ### YAML
//! ```yaml
//! resources:
//!   sharedVolume:
//!     type: docker:Volume
//! ```
//! <!--End PulumiCodeChooser -->
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

pub struct VolumeArgs {
    /// Driver type for the volume. Defaults to `local`.
    pub driver: pulumi_wasm_rust::Output<Option<String>>,
    /// Options specific to the driver.
    pub driver_opts: pulumi_wasm_rust::Output<Option<std::collections::HashMap<String, String>>>,
    /// User-defined key/value metadata
    pub labels: pulumi_wasm_rust::Output<Option<Vec<crate::types::VolumeLabel>>>,
    /// The name of the Docker volume (will be generated if not provided).
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
    let result = crate::bindings::pulumi::docker::volume::invoke(
        name,
        &crate::bindings::pulumi::docker::volume::Args {
            driver: args.driver.get_inner(),
            driver_opts: args.driver_opts.get_inner(),
            labels: args.labels.get_inner(),
            name: args.name.get_inner(),
        },
    );

    VolumeResult {
        driver: crate::into_domain(result.driver),
        driver_opts: crate::into_domain(result.driver_opts),
        labels: crate::into_domain(result.labels),
        mountpoint: crate::into_domain(result.mountpoint),
        name: crate::into_domain(result.name),
    }
}
