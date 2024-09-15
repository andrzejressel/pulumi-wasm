//! Provides a resource, that manages Cloudflare tunnel virtual networks
//! for Zero Trust. Tunnel virtual networks are used for segregation of
//! Tunnel IP Routes via Virtualized Networks to handle overlapping
//! private IPs in your origins.
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as cloudflare from "@pulumi/cloudflare";
//! 
//! const example = new cloudflare.TunnelVirtualNetwork("example", {
//!     accountId: "f037e56e89293a057740de681ac9abbe",
//!     comment: "New tunnel virtual network for documentation",
//!     name: "vnet-for-documentation",
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//! 
//! example = cloudflare.TunnelVirtualNetwork("example",
//!     account_id="f037e56e89293a057740de681ac9abbe",
//!     comment="New tunnel virtual network for documentation",
//!     name="vnet-for-documentation")
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
//!     var example = new Cloudflare.TunnelVirtualNetwork("example", new()
//!     {
//!         AccountId = "f037e56e89293a057740de681ac9abbe",
//!         Comment = "New tunnel virtual network for documentation",
//!         Name = "vnet-for-documentation",
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
//! 		_, err := cloudflare.NewTunnelVirtualNetwork(ctx, "example", &cloudflare.TunnelVirtualNetworkArgs{
//! 			AccountId: pulumi.String("f037e56e89293a057740de681ac9abbe"),
//! 			Comment:   pulumi.String("New tunnel virtual network for documentation"),
//! 			Name:      pulumi.String("vnet-for-documentation"),
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
//! import com.pulumi.cloudflare.TunnelVirtualNetwork;
//! import com.pulumi.cloudflare.TunnelVirtualNetworkArgs;
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
//!         var example = new TunnelVirtualNetwork("example", TunnelVirtualNetworkArgs.builder()        
//!             .accountId("f037e56e89293a057740de681ac9abbe")
//!             .comment("New tunnel virtual network for documentation")
//!             .name("vnet-for-documentation")
//!             .build());
//! 
//!     }
//! }
//! ```
//! ### YAML
//! ```yaml
//! resources:
//!   example:
//!     type: cloudflare:TunnelVirtualNetwork
//!     properties:
//!       accountId: f037e56e89293a057740de681ac9abbe
//!       comment: New tunnel virtual network for documentation
//!       name: vnet-for-documentation
//! ```
//! <!--End PulumiCodeChooser -->
//! 
//! ## Import
//! 
//! ```sh
//! $ pulumi import cloudflare:index/tunnelVirtualNetwork:TunnelVirtualNetwork example <account_id>/<vnet_id>
//! ```
//! 

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct TunnelVirtualNetworkArgs {
    /// The account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// Description of the tunnel virtual network.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub comment: pulumi_wasm_rust::Output<Option<String>>,
    /// Whether this virtual network is the default one for the account. This means IP Routes belong to this virtual network and Teams Clients in the account route through this virtual network, unless specified otherwise for each case.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub is_default_network: pulumi_wasm_rust::Output<Option<bool>>,
    /// A user-friendly name chosen when the virtual network is created.
    #[builder(into)]
    pub name: pulumi_wasm_rust::Output<String>,
}

pub struct TunnelVirtualNetworkResult {
    /// The account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// Description of the tunnel virtual network.
    pub comment: pulumi_wasm_rust::Output<Option<String>>,
    /// Whether this virtual network is the default one for the account. This means IP Routes belong to this virtual network and Teams Clients in the account route through this virtual network, unless specified otherwise for each case.
    pub is_default_network: pulumi_wasm_rust::Output<Option<bool>>,
    /// A user-friendly name chosen when the virtual network is created.
    pub name: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: TunnelVirtualNetworkArgs) -> TunnelVirtualNetworkResult {

    let result = crate::bindings::pulumi::cloudflare::tunnel_virtual_network::invoke(name, &crate::bindings::pulumi::cloudflare::tunnel_virtual_network::Args {
        account_id: &args.account_id.get_inner(),
        comment: &args.comment.get_inner(),
        is_default_network: &args.is_default_network.get_inner(),
        name: &args.name.get_inner(),
    });

    TunnelVirtualNetworkResult {
        account_id: crate::into_domain(result.account_id),
        comment: crate::into_domain(result.comment),
        is_default_network: crate::into_domain(result.is_default_network),
        name: crate::into_domain(result.name),
    }
}
