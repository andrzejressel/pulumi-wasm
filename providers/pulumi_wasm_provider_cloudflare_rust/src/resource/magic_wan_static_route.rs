//! Provides a resource, that manages Cloudflare static routes for Magic
//! Transit or Magic WAN. Static routes are used to route traffic
//! through GRE tunnels.
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as cloudflare from "@pulumi/cloudflare";
//! 
//! const example = new cloudflare.MagicWanStaticRoute("example", {
//!     accountId: "f037e56e89293a057740de681ac9abbe",
//!     description: "New route for new prefix 192.0.2.0/24",
//!     prefix: "192.0.2.0/24",
//!     nexthop: "10.0.0.0",
//!     priority: 100,
//!     weight: 10,
//!     coloNames: ["den01"],
//!     coloRegions: ["APAC"],
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//! 
//! example = cloudflare.MagicWanStaticRoute("example",
//!     account_id="f037e56e89293a057740de681ac9abbe",
//!     description="New route for new prefix 192.0.2.0/24",
//!     prefix="192.0.2.0/24",
//!     nexthop="10.0.0.0",
//!     priority=100,
//!     weight=10,
//!     colo_names=["den01"],
//!     colo_regions=["APAC"])
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
//!     var example = new Cloudflare.MagicWanStaticRoute("example", new()
//!     {
//!         AccountId = "f037e56e89293a057740de681ac9abbe",
//!         Description = "New route for new prefix 192.0.2.0/24",
//!         Prefix = "192.0.2.0/24",
//!         Nexthop = "10.0.0.0",
//!         Priority = 100,
//!         Weight = 10,
//!         ColoNames = new[]
//!         {
//!             "den01",
//!         },
//!         ColoRegions = new[]
//!         {
//!             "APAC",
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
//! 	"github.com/pulumi/pulumi-cloudflare/sdk/v5/go/cloudflare"
//! 	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
//! )
//! 
//! func main() {
//! 	pulumi.Run(func(ctx *pulumi.Context) error {
//! 		_, err := cloudflare.NewMagicWanStaticRoute(ctx, "example", &cloudflare.MagicWanStaticRouteArgs{
//! 			AccountId:   pulumi.String("f037e56e89293a057740de681ac9abbe"),
//! 			Description: pulumi.String("New route for new prefix 192.0.2.0/24"),
//! 			Prefix:      pulumi.String("192.0.2.0/24"),
//! 			Nexthop:     pulumi.String("10.0.0.0"),
//! 			Priority:    pulumi.Int(100),
//! 			Weight:      pulumi.Int(10),
//! 			ColoNames: pulumi.StringArray{
//! 				pulumi.String("den01"),
//! 			},
//! 			ColoRegions: pulumi.StringArray{
//! 				pulumi.String("APAC"),
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
//! import com.pulumi.cloudflare.MagicWanStaticRoute;
//! import com.pulumi.cloudflare.MagicWanStaticRouteArgs;
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
//!         var example = new MagicWanStaticRoute("example", MagicWanStaticRouteArgs.builder()
//!             .accountId("f037e56e89293a057740de681ac9abbe")
//!             .description("New route for new prefix 192.0.2.0/24")
//!             .prefix("192.0.2.0/24")
//!             .nexthop("10.0.0.0")
//!             .priority(100)
//!             .weight(10)
//!             .coloNames("den01")
//!             .coloRegions("APAC")
//!             .build());
//! 
//!     }
//! }
//! ```
//! ### YAML
//! ```yaml
//! resources:
//!   example:
//!     type: cloudflare:MagicWanStaticRoute
//!     properties:
//!       accountId: f037e56e89293a057740de681ac9abbe
//!       description: New route for new prefix 192.0.2.0/24
//!       prefix: 192.0.2.0/24
//!       nexthop: 10.0.0.0
//!       priority: 100
//!       weight: 10
//!       coloNames:
//!         - den01
//!       coloRegions:
//!         - APAC
//! ```
//! <!--End PulumiCodeChooser -->
//! 
//! ## Import
//! 
//! ```sh
//! $ pulumi import cloudflare:index/magicWanStaticRoute:MagicWanStaticRoute example <account_id>/<static_route_id>
//! ```
//! 

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct MagicWanStaticRouteArgs {
    /// The account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub account_id: pulumi_wasm_rust::Output<Option<String>>,
    /// List of Cloudflare colocation regions for this static route.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub colo_names: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    /// List of Cloudflare colocation names for this static route.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub colo_regions: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    /// Description of the static route.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    /// The nexthop IP address where traffic will be routed to.
    #[builder(into)]
    pub nexthop: pulumi_wasm_rust::Output<String>,
    /// Your network prefix using CIDR notation.
    #[builder(into)]
    pub prefix: pulumi_wasm_rust::Output<String>,
    /// The priority for the static route.
    #[builder(into)]
    pub priority: pulumi_wasm_rust::Output<i32>,
    /// The optional weight for ECMP routes. **Modifying this attribute will force creation of a new resource.**
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub weight: pulumi_wasm_rust::Output<Option<i32>>,
}

pub struct MagicWanStaticRouteResult {
    /// The account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub account_id: pulumi_wasm_rust::Output<Option<String>>,
    /// List of Cloudflare colocation regions for this static route.
    pub colo_names: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    /// List of Cloudflare colocation names for this static route.
    pub colo_regions: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    /// Description of the static route.
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    /// The nexthop IP address where traffic will be routed to.
    pub nexthop: pulumi_wasm_rust::Output<String>,
    /// Your network prefix using CIDR notation.
    pub prefix: pulumi_wasm_rust::Output<String>,
    /// The priority for the static route.
    pub priority: pulumi_wasm_rust::Output<i32>,
    /// The optional weight for ECMP routes. **Modifying this attribute will force creation of a new resource.**
    pub weight: pulumi_wasm_rust::Output<Option<i32>>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: MagicWanStaticRouteArgs) -> MagicWanStaticRouteResult {

    let result = crate::bindings::pulumi::cloudflare::magic_wan_static_route::invoke(name, &crate::bindings::pulumi::cloudflare::magic_wan_static_route::Args {
        account_id: &args.account_id.get_inner(),
        colo_names: &args.colo_names.get_inner(),
        colo_regions: &args.colo_regions.get_inner(),
        description: &args.description.get_inner(),
        nexthop: &args.nexthop.get_inner(),
        prefix: &args.prefix.get_inner(),
        priority: &args.priority.get_inner(),
        weight: &args.weight.get_inner(),
    });

    MagicWanStaticRouteResult {
        account_id: crate::into_domain(result.account_id),
        colo_names: crate::into_domain(result.colo_names),
        colo_regions: crate::into_domain(result.colo_regions),
        description: crate::into_domain(result.description),
        nexthop: crate::into_domain(result.nexthop),
        prefix: crate::into_domain(result.prefix),
        priority: crate::into_domain(result.priority),
        weight: crate::into_domain(result.weight),
    }
}