//! `docker.Network` provides details about a specific Docker Network.
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as docker from "@pulumi/docker";
//! 
//! const main = docker.getNetwork({
//!     name: "main",
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_docker as docker
//! 
//! main = docker.get_network(name="main")
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
//!     var main = Docker.GetNetwork.Invoke(new()
//!     {
//!         Name = "main",
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
//! 		_, err := docker.LookupNetwork(ctx, &docker.LookupNetworkArgs{
//! 			Name: "main",
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
//! import com.pulumi.docker.inputs.GetNetworkArgs;
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
//!         final var main = DockerFunctions.getNetwork(GetNetworkArgs.builder()
//!             .name("main")
//!             .build());
//! 
//!     }
//! }
//! ```
//! ### YAML
//! ```yaml
//! variables:
//!   main:
//!     fn::invoke:
//!       Function: docker:getNetwork
//!       Arguments:
//!         name: main
//! ```
//! <!--End PulumiCodeChooser -->

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct GetNetworkArgs {
    /// The name of the Docker network.
    #[builder(into)]
    pub name: pulumi_wasm_rust::Output<String>,
}

pub struct GetNetworkResult {
    /// The driver of the Docker network. Possible values are `bridge`, `host`, `overlay`, `macvlan`. See [network docs](https://docs.docker.com/network/#network-drivers) for more details.
    pub driver: pulumi_wasm_rust::Output<String>,
    /// The ID of this resource.
    pub id: pulumi_wasm_rust::Output<String>,
    /// If `true`, the network is internal.
    pub internal: pulumi_wasm_rust::Output<bool>,
    /// The IPAM configuration options
    pub ipam_configs: pulumi_wasm_rust::Output<Vec<crate::types::GetNetworkIpamConfig>>,
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
pub fn invoke(args: GetNetworkArgs) -> GetNetworkResult {

    let result = crate::bindings::pulumi::docker::get_network::invoke(&crate::bindings::pulumi::docker::get_network::Args {
        name: &args.name.get_inner(),
    });

    GetNetworkResult {
        driver: crate::into_domain(result.driver),
        id: crate::into_domain(result.id),
        internal: crate::into_domain(result.internal),
        ipam_configs: crate::into_domain(result.ipam_configs),
        name: crate::into_domain(result.name),
        options: crate::into_domain(result.options),
        scope: crate::into_domain(result.scope),
    }
}
