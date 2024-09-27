//! Use this datasource to lookup a tunnel virtual network in an account.
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as cloudflare from "@pulumi/cloudflare";
//! 
//! const example = cloudflare.getTunnelVirtualNetwork({
//!     accountId: "f037e56e89293a057740de681ac9abbe",
//!     name: "example",
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//! 
//! example = cloudflare.get_tunnel_virtual_network(account_id="f037e56e89293a057740de681ac9abbe",
//!     name="example")
//! ```
//! ### C#
//! ```csharp
//! using System.Collections.Generic;
//! using System.Linq;
//! using Pulumi;
//! using Cloudflare = Pulumi.Cloudflare;
//! 
//! return await Deployment.RunAsync(() => 
//! {
//!     var example = Cloudflare.GetTunnelVirtualNetwork.Invoke(new()
//!     {
//!         AccountId = "f037e56e89293a057740de681ac9abbe",
//!         Name = "example",
//!     });
//! 
//! });
//! ```
//! ### Go
//! ```go
//! package main
//! 
//! import (
//! 	"github.com/pulumi/pulumi-cloudflare/sdk/v5/go/cloudflare"
//! 	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
//! )
//! 
//! func main() {
//! 	pulumi.Run(func(ctx *pulumi.Context) error {
//! 		_, err := cloudflare.LookupTunnelVirtualNetwork(ctx, &cloudflare.LookupTunnelVirtualNetworkArgs{
//! 			AccountId: "f037e56e89293a057740de681ac9abbe",
//! 			Name:      "example",
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
//! import com.pulumi.cloudflare.CloudflareFunctions;
//! import com.pulumi.cloudflare.inputs.GetTunnelVirtualNetworkArgs;
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
//!         final var example = CloudflareFunctions.getTunnelVirtualNetwork(GetTunnelVirtualNetworkArgs.builder()
//!             .accountId("f037e56e89293a057740de681ac9abbe")
//!             .name("example")
//!             .build());
//! 
//!     }
//! }
//! ```
//! ### YAML
//! ```yaml
//! variables:
//!   example:
//!     fn::invoke:
//!       Function: cloudflare:getTunnelVirtualNetwork
//!       Arguments:
//!         accountId: f037e56e89293a057740de681ac9abbe
//!         name: example
//! ```
//! <!--End PulumiCodeChooser -->

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct GetTunnelVirtualNetworkArgs {
    /// The account identifier to target for the resource.
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The Virtual Network Name.
    #[builder(into)]
    pub name: pulumi_wasm_rust::Output<String>,
}

pub struct GetTunnelVirtualNetworkResult {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The Virtual Network Comment.
    pub comment: pulumi_wasm_rust::Output<String>,
    /// The provider-assigned unique ID for this managed resource.
    pub id: pulumi_wasm_rust::Output<String>,
    /// If true, only include deleted virtual networks. If false, exclude deleted virtual networks. If empty, all virtual networks will be included.
    pub is_default: pulumi_wasm_rust::Output<bool>,
    /// The Virtual Network Name.
    pub name: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn invoke(args: GetTunnelVirtualNetworkArgs) -> GetTunnelVirtualNetworkResult {

    let result = crate::bindings::pulumi::cloudflare::get_tunnel_virtual_network::invoke(&crate::bindings::pulumi::cloudflare::get_tunnel_virtual_network::Args {
        account_id: &args.account_id.get_inner(),
        name: &args.name.get_inner(),
    });

    GetTunnelVirtualNetworkResult {
        account_id: crate::into_domain(result.account_id),
        comment: crate::into_domain(result.comment),
        id: crate::into_domain(result.id),
        is_default: crate::into_domain(result.is_default),
        name: crate::into_domain(result.name),
    }
}
