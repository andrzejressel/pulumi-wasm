//! Provides a resource, that manages Cloudflare tunnel routes for Zero
//! Trust. Tunnel routes are used to direct IP traffic through
//! Cloudflare Tunnels.
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as cloudflare from "@pulumi/cloudflare";
//! 
//! // Tunnel route
//! const exampleTunnelRoute = new cloudflare.TunnelRoute("exampleTunnelRoute", {
//!     accountId: "f037e56e89293a057740de681ac9abbe",
//!     tunnelId: "f70ff985-a4ef-4643-bbbc-4a0ed4fc8415",
//!     network: "192.0.2.24/32",
//!     comment: "New tunnel route for documentation",
//!     virtualNetworkId: "bdc39a3c-3104-4c23-8ac0-9f455dda691a",
//! });
//! // Tunnel with tunnel route
//! const tunnel = new cloudflare.Tunnel("tunnel", {
//!     accountId: "f037e56e89293a057740de681ac9abbe",
//!     name: "my_tunnel",
//!     secret: "AQIDBAUGBwgBAgMEBQYHCAECAwQFBgcIAQIDBAUGBwg=",
//! });
//! const exampleIndex_tunnelRouteTunnelRoute = new cloudflare.TunnelRoute("exampleIndex/tunnelRouteTunnelRoute", {
//!     accountId: "f037e56e89293a057740de681ac9abbe",
//!     tunnelId: tunnel.id,
//!     network: "192.0.2.24/32",
//!     comment: "New tunnel route for documentation",
//!     virtualNetworkId: "bdc39a3c-3104-4c23-8ac0-9f455dda691a",
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//! 
//! # Tunnel route
//! example_tunnel_route = cloudflare.TunnelRoute("exampleTunnelRoute",
//!     account_id="f037e56e89293a057740de681ac9abbe",
//!     tunnel_id="f70ff985-a4ef-4643-bbbc-4a0ed4fc8415",
//!     network="192.0.2.24/32",
//!     comment="New tunnel route for documentation",
//!     virtual_network_id="bdc39a3c-3104-4c23-8ac0-9f455dda691a")
//! # Tunnel with tunnel route
//! tunnel = cloudflare.Tunnel("tunnel",
//!     account_id="f037e56e89293a057740de681ac9abbe",
//!     name="my_tunnel",
//!     secret="AQIDBAUGBwgBAgMEBQYHCAECAwQFBgcIAQIDBAUGBwg=")
//! example_index_tunnel_route_tunnel_route = cloudflare.TunnelRoute("exampleIndex/tunnelRouteTunnelRoute",
//!     account_id="f037e56e89293a057740de681ac9abbe",
//!     tunnel_id=tunnel.id,
//!     network="192.0.2.24/32",
//!     comment="New tunnel route for documentation",
//!     virtual_network_id="bdc39a3c-3104-4c23-8ac0-9f455dda691a")
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
//!     // Tunnel route
//!     var exampleTunnelRoute = new Cloudflare.TunnelRoute("exampleTunnelRoute", new()
//!     {
//!         AccountId = "f037e56e89293a057740de681ac9abbe",
//!         TunnelId = "f70ff985-a4ef-4643-bbbc-4a0ed4fc8415",
//!         Network = "192.0.2.24/32",
//!         Comment = "New tunnel route for documentation",
//!         VirtualNetworkId = "bdc39a3c-3104-4c23-8ac0-9f455dda691a",
//!     });
//! 
//!     // Tunnel with tunnel route
//!     var tunnel = new Cloudflare.Tunnel("tunnel", new()
//!     {
//!         AccountId = "f037e56e89293a057740de681ac9abbe",
//!         Name = "my_tunnel",
//!         Secret = "AQIDBAUGBwgBAgMEBQYHCAECAwQFBgcIAQIDBAUGBwg=",
//!     });
//! 
//!     var exampleIndex_tunnelRouteTunnelRoute = new Cloudflare.TunnelRoute("exampleIndex/tunnelRouteTunnelRoute", new()
//!     {
//!         AccountId = "f037e56e89293a057740de681ac9abbe",
//!         TunnelId = tunnel.Id,
//!         Network = "192.0.2.24/32",
//!         Comment = "New tunnel route for documentation",
//!         VirtualNetworkId = "bdc39a3c-3104-4c23-8ac0-9f455dda691a",
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
//! 		// Tunnel route
//! 		_, err := cloudflare.NewTunnelRoute(ctx, "exampleTunnelRoute", &cloudflare.TunnelRouteArgs{
//! 			AccountId:        pulumi.String("f037e56e89293a057740de681ac9abbe"),
//! 			TunnelId:         pulumi.String("f70ff985-a4ef-4643-bbbc-4a0ed4fc8415"),
//! 			Network:          pulumi.String("192.0.2.24/32"),
//! 			Comment:          pulumi.String("New tunnel route for documentation"),
//! 			VirtualNetworkId: pulumi.String("bdc39a3c-3104-4c23-8ac0-9f455dda691a"),
//! 		})
//! 		if err != nil {
//! 			return err
//! 		}
//! 		// Tunnel with tunnel route
//! 		tunnel, err := cloudflare.NewTunnel(ctx, "tunnel", &cloudflare.TunnelArgs{
//! 			AccountId: pulumi.String("f037e56e89293a057740de681ac9abbe"),
//! 			Name:      pulumi.String("my_tunnel"),
//! 			Secret:    pulumi.String("AQIDBAUGBwgBAgMEBQYHCAECAwQFBgcIAQIDBAUGBwg="),
//! 		})
//! 		if err != nil {
//! 			return err
//! 		}
//! 		_, err = cloudflare.NewTunnelRoute(ctx, "exampleIndex/tunnelRouteTunnelRoute", &cloudflare.TunnelRouteArgs{
//! 			AccountId:        pulumi.String("f037e56e89293a057740de681ac9abbe"),
//! 			TunnelId:         tunnel.ID(),
//! 			Network:          pulumi.String("192.0.2.24/32"),
//! 			Comment:          pulumi.String("New tunnel route for documentation"),
//! 			VirtualNetworkId: pulumi.String("bdc39a3c-3104-4c23-8ac0-9f455dda691a"),
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
//! import com.pulumi.cloudflare.TunnelRoute;
//! import com.pulumi.cloudflare.TunnelRouteArgs;
//! import com.pulumi.cloudflare.Tunnel;
//! import com.pulumi.cloudflare.TunnelArgs;
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
//!         // Tunnel route
//!         var exampleTunnelRoute = new TunnelRoute("exampleTunnelRoute", TunnelRouteArgs.builder()        
//!             .accountId("f037e56e89293a057740de681ac9abbe")
//!             .tunnelId("f70ff985-a4ef-4643-bbbc-4a0ed4fc8415")
//!             .network("192.0.2.24/32")
//!             .comment("New tunnel route for documentation")
//!             .virtualNetworkId("bdc39a3c-3104-4c23-8ac0-9f455dda691a")
//!             .build());
//! 
//!         // Tunnel with tunnel route
//!         var tunnel = new Tunnel("tunnel", TunnelArgs.builder()        
//!             .accountId("f037e56e89293a057740de681ac9abbe")
//!             .name("my_tunnel")
//!             .secret("AQIDBAUGBwgBAgMEBQYHCAECAwQFBgcIAQIDBAUGBwg=")
//!             .build());
//! 
//!         var exampleIndex_tunnelRouteTunnelRoute = new TunnelRoute("exampleIndex/tunnelRouteTunnelRoute", TunnelRouteArgs.builder()        
//!             .accountId("f037e56e89293a057740de681ac9abbe")
//!             .tunnelId(tunnel.id())
//!             .network("192.0.2.24/32")
//!             .comment("New tunnel route for documentation")
//!             .virtualNetworkId("bdc39a3c-3104-4c23-8ac0-9f455dda691a")
//!             .build());
//! 
//!     }
//! }
//! ```
//! ### YAML
//! ```yaml
//! resources:
//!   # Tunnel route
//!   exampleTunnelRoute:
//!     type: cloudflare:TunnelRoute
//!     properties:
//!       accountId: f037e56e89293a057740de681ac9abbe
//!       tunnelId: f70ff985-a4ef-4643-bbbc-4a0ed4fc8415
//!       network: 192.0.2.24/32
//!       comment: New tunnel route for documentation
//!       virtualNetworkId: bdc39a3c-3104-4c23-8ac0-9f455dda691a
//!   # Tunnel with tunnel route
//!   tunnel:
//!     type: cloudflare:Tunnel
//!     properties:
//!       accountId: f037e56e89293a057740de681ac9abbe
//!       name: my_tunnel
//!       secret: AQIDBAUGBwgBAgMEBQYHCAECAwQFBgcIAQIDBAUGBwg=
//!   exampleIndex/tunnelRouteTunnelRoute:
//!     type: cloudflare:TunnelRoute
//!     properties:
//!       accountId: f037e56e89293a057740de681ac9abbe
//!       tunnelId: ${tunnel.id}
//!       network: 192.0.2.24/32
//!       comment: New tunnel route for documentation
//!       virtualNetworkId: bdc39a3c-3104-4c23-8ac0-9f455dda691a
//! ```
//! <!--End PulumiCodeChooser -->
//! 
//! ## Import
//! 
//! ```sh
//! $ pulumi import cloudflare:index/tunnelRoute:TunnelRoute example <account_id>/<network_cidr>/<virtual_network_id>
//! ```
//! 

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct TunnelRouteArgs {
    /// The account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// Description of the tunnel route.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub comment: pulumi_wasm_rust::Output<Option<String>>,
    /// The IPv4 or IPv6 network that should use this tunnel route, in CIDR notation.
    #[builder(into)]
    pub network: pulumi_wasm_rust::Output<String>,
    /// The ID of the tunnel that will service the tunnel route.
    #[builder(into)]
    pub tunnel_id: pulumi_wasm_rust::Output<String>,
    /// The ID of the virtual network for which this route is being added; uses the default virtual network of the account if none is provided. **Modifying this attribute will force creation of a new resource.**
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub virtual_network_id: pulumi_wasm_rust::Output<Option<String>>,
}

pub struct TunnelRouteResult {
    /// The account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// Description of the tunnel route.
    pub comment: pulumi_wasm_rust::Output<Option<String>>,
    /// The IPv4 or IPv6 network that should use this tunnel route, in CIDR notation.
    pub network: pulumi_wasm_rust::Output<String>,
    /// The ID of the tunnel that will service the tunnel route.
    pub tunnel_id: pulumi_wasm_rust::Output<String>,
    /// The ID of the virtual network for which this route is being added; uses the default virtual network of the account if none is provided. **Modifying this attribute will force creation of a new resource.**
    pub virtual_network_id: pulumi_wasm_rust::Output<Option<String>>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: TunnelRouteArgs) -> TunnelRouteResult {

    let result = crate::bindings::pulumi::cloudflare::tunnel_route::invoke(name, &crate::bindings::pulumi::cloudflare::tunnel_route::Args {
        account_id: &args.account_id.get_inner(),
        comment: &args.comment.get_inner(),
        network: &args.network.get_inner(),
        tunnel_id: &args.tunnel_id.get_inner(),
        virtual_network_id: &args.virtual_network_id.get_inner(),
    });

    TunnelRouteResult {
        account_id: crate::into_domain(result.account_id),
        comment: crate::into_domain(result.comment),
        network: crate::into_domain(result.network),
        tunnel_id: crate::into_domain(result.tunnel_id),
        virtual_network_id: crate::into_domain(result.virtual_network_id),
    }
}
