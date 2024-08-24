//! <!-- Bug: Type and Name are switched -->
//! `docker.Network` provides a docker network resource.
//!
//! ## Example Usage
//!
//! <!--Start PulumiCodeChooser -->
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as docker from "@pulumi/docker";
//!
//! const privateNetwork = new docker.Network("privateNetwork", {});
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_docker as docker
//!
//! private_network = docker.Network("privateNetwork")
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
//!     var privateNetwork = new Docker.Network("privateNetwork");
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
//! 		_, err := docker.NewNetwork(ctx, "privateNetwork", nil)
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
//! import com.pulumi.docker.Network;
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
//!         var privateNetwork = new Network("privateNetwork");
//!
//!     }
//! }
//! ```
//! ### YAML
//! ```yaml
//! resources:
//!   privateNetwork:
//!     type: docker:Network
//! ```
//! <!--End PulumiCodeChooser -->
//!
//! ## Import
//!
//! ### Example
//!
//! Assuming you created a `network` as follows
//!
//! #!/bin/bash
//!
//! docker network create foo
//!
//! prints the long ID
//!
//! 87b57a9b91ecab2db2a6dbf38df74c67d7c7108cbe479d6576574ec2cd8c2d73
//!
//! you provide the definition for the resource as follows
//!
//! terraform
//!
//! resource "docker_network" "foo" {
//!
//!   name = "foo"
//!
//! }
//!
//! then the import command is as follows
//!
//! #!/bin/bash
//!
//! ```sh
//! $ pulumi import docker:index/network:Network foo 87b57a9b91ecab2db2a6dbf38df74c67d7c7108cbe479d6576574ec2cd8c2d73
//! ```
//!

pub struct NetworkArgs {
    /// Enable manual container attachment to the network.
    pub attachable: pulumi_wasm_rust::Output<Option<bool>>,
    /// Requests daemon to check for networks with same name.
    pub check_duplicate: pulumi_wasm_rust::Output<Option<bool>>,
    /// The driver of the Docker network. Possible values are `bridge`, `host`, `overlay`, `macvlan`. See [network docs](https://docs.docker.com/network/#network-drivers) for more details.
    pub driver: pulumi_wasm_rust::Output<Option<String>>,
    /// Create swarm routing-mesh network. Defaults to `false`.
    pub ingress: pulumi_wasm_rust::Output<Option<bool>>,
    /// Whether the network is internal.
    pub internal: pulumi_wasm_rust::Output<Option<bool>>,
    /// The IPAM configuration options
    pub ipam_configs: pulumi_wasm_rust::Output<Option<Vec<crate::types::NetworkIpamConfig>>>,
    /// Driver used by the custom IP scheme of the network. Defaults to `default`
    pub ipam_driver: pulumi_wasm_rust::Output<Option<String>>,
    /// Provide explicit options to the IPAM driver. Valid options vary with `ipam_driver` and refer to that driver's documentation for more details.
    pub ipam_options: pulumi_wasm_rust::Output<Option<std::collections::HashMap<String, String>>>,
    /// Enable IPv6 networking. Defaults to `false`.
    pub ipv6: pulumi_wasm_rust::Output<Option<bool>>,
    /// User-defined key/value metadata
    pub labels: pulumi_wasm_rust::Output<Option<Vec<crate::types::NetworkLabel>>>,
    /// The name of the Docker network.
    pub name: pulumi_wasm_rust::Output<Option<String>>,
    /// Only available with bridge networks. See [bridge options docs](https://docs.docker.com/engine/reference/commandline/network_create/#bridge-driver-options) for more details.
    pub options: pulumi_wasm_rust::Output<Option<std::collections::HashMap<String, String>>>,
}

pub struct NetworkResult {
    /// Enable manual container attachment to the network.
    pub attachable: pulumi_wasm_rust::Output<Option<bool>>,
    /// Requests daemon to check for networks with same name.
    pub check_duplicate: pulumi_wasm_rust::Output<Option<bool>>,
    /// The driver of the Docker network. Possible values are `bridge`, `host`, `overlay`, `macvlan`. See [network docs](https://docs.docker.com/network/#network-drivers) for more details.
    pub driver: pulumi_wasm_rust::Output<String>,
    /// Create swarm routing-mesh network. Defaults to `false`.
    pub ingress: pulumi_wasm_rust::Output<Option<bool>>,
    /// Whether the network is internal.
    pub internal: pulumi_wasm_rust::Output<bool>,
    /// The IPAM configuration options
    pub ipam_configs: pulumi_wasm_rust::Output<Vec<crate::types::NetworkIpamConfig>>,
    /// Driver used by the custom IP scheme of the network. Defaults to `default`
    pub ipam_driver: pulumi_wasm_rust::Output<Option<String>>,
    /// Provide explicit options to the IPAM driver. Valid options vary with `ipam_driver` and refer to that driver's documentation for more details.
    pub ipam_options: pulumi_wasm_rust::Output<Option<std::collections::HashMap<String, String>>>,
    /// Enable IPv6 networking. Defaults to `false`.
    pub ipv6: pulumi_wasm_rust::Output<Option<bool>>,
    /// User-defined key/value metadata
    pub labels: pulumi_wasm_rust::Output<Option<Vec<crate::types::NetworkLabel>>>,
    /// The name of the Docker network.
    pub name: pulumi_wasm_rust::Output<String>,
    /// Only available with bridge networks. See [bridge options docs](https://docs.docker.com/engine/reference/commandline/network_create/#bridge-driver-options) for more details.
    pub options: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
    /// Scope of the network. One of `swarm`, `global`, or `local`.
    pub scope: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: NetworkArgs) -> NetworkResult {
    let result = crate::bindings::pulumi::docker::network::invoke(
        name,
        &crate::bindings::pulumi::docker::network::Args {
            attachable: args.attachable.get_inner(),
            check_duplicate: args.check_duplicate.get_inner(),
            driver: args.driver.get_inner(),
            ingress: args.ingress.get_inner(),
            internal: args.internal.get_inner(),
            ipam_configs: args.ipam_configs.get_inner(),
            ipam_driver: args.ipam_driver.get_inner(),
            ipam_options: args.ipam_options.get_inner(),
            ipv6: args.ipv6.get_inner(),
            labels: args.labels.get_inner(),
            name: args.name.get_inner(),
            options: args.options.get_inner(),
        },
    );

    NetworkResult {
        attachable: crate::into_domain(result.attachable),
        check_duplicate: crate::into_domain(result.check_duplicate),
        driver: crate::into_domain(result.driver),
        ingress: crate::into_domain(result.ingress),
        internal: crate::into_domain(result.internal),
        ipam_configs: crate::into_domain(result.ipam_configs),
        ipam_driver: crate::into_domain(result.ipam_driver),
        ipam_options: crate::into_domain(result.ipam_options),
        ipv6: crate::into_domain(result.ipv6),
        labels: crate::into_domain(result.labels),
        name: crate::into_domain(result.name),
        options: crate::into_domain(result.options),
        scope: crate::into_domain(result.scope),
    }
}
