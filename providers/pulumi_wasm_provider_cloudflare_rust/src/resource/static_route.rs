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
//! const example = new cloudflare.StaticRoute("example", {
//!     accountId: "f037e56e89293a057740de681ac9abbe",
//!     coloNames: ["den01"],
//!     coloRegions: ["APAC"],
//!     description: "New route for new prefix 192.0.2.0/24",
//!     nexthop: "10.0.0.0",
//!     prefix: "192.0.2.0/24",
//!     priority: 100,
//!     weight: 10,
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//! 
//! example = cloudflare.StaticRoute("example",
//!     account_id="f037e56e89293a057740de681ac9abbe",
//!     colo_names=["den01"],
//!     colo_regions=["APAC"],
//!     description="New route for new prefix 192.0.2.0/24",
//!     nexthop="10.0.0.0",
//!     prefix="192.0.2.0/24",
//!     priority=100,
//!     weight=10)
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
//!     var example = new Cloudflare.StaticRoute("example", new()
//!     {
//!         AccountId = "f037e56e89293a057740de681ac9abbe",
//!         ColoNames = new[]
//!         {
//!             "den01",
//!         },
//!         ColoRegions = new[]
//!         {
//!             "APAC",
//!         },
//!         Description = "New route for new prefix 192.0.2.0/24",
//!         Nexthop = "10.0.0.0",
//!         Prefix = "192.0.2.0/24",
//!         Priority = 100,
//!         Weight = 10,
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
//! 		_, err := cloudflare.NewStaticRoute(ctx, "example", &cloudflare.StaticRouteArgs{
//! 			AccountId: pulumi.String("f037e56e89293a057740de681ac9abbe"),
//! 			ColoNames: pulumi.StringArray{
//! 				pulumi.String("den01"),
//! 			},
//! 			ColoRegions: pulumi.StringArray{
//! 				pulumi.String("APAC"),
//! 			},
//! 			Description: pulumi.String("New route for new prefix 192.0.2.0/24"),
//! 			Nexthop:     pulumi.String("10.0.0.0"),
//! 			Prefix:      pulumi.String("192.0.2.0/24"),
//! 			Priority:    pulumi.Int(100),
//! 			Weight:      pulumi.Int(10),
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
//! import com.pulumi.cloudflare.StaticRoute;
//! import com.pulumi.cloudflare.StaticRouteArgs;
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
//!         var example = new StaticRoute("example", StaticRouteArgs.builder()        
//!             .accountId("f037e56e89293a057740de681ac9abbe")
//!             .coloNames("den01")
//!             .coloRegions("APAC")
//!             .description("New route for new prefix 192.0.2.0/24")
//!             .nexthop("10.0.0.0")
//!             .prefix("192.0.2.0/24")
//!             .priority(100)
//!             .weight(10)
//!             .build());
//! 
//!     }
//! }
//! ```
//! ### YAML
//! ```yaml
//! resources:
//!   example:
//!     type: cloudflare:StaticRoute
//!     properties:
//!       accountId: f037e56e89293a057740de681ac9abbe
//!       coloNames:
//!         - den01
//!       coloRegions:
//!         - APAC
//!       description: New route for new prefix 192.0.2.0/24
//!       nexthop: 10.0.0.0
//!       prefix: 192.0.2.0/24
//!       priority: 100
//!       weight: 10
//! ```
//! <!--End PulumiCodeChooser -->
//! 
//! ## Import
//! 
//! ```sh
//! $ pulumi import cloudflare:index/staticRoute:StaticRoute example <account_id>/<static_route_id>
//! ```
//! 

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct StaticRouteArgs {
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

pub struct StaticRouteResult {
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
pub fn create(name: &str, args: StaticRouteArgs) -> StaticRouteResult {

    let result = crate::bindings::pulumi::cloudflare::static_route::invoke(name, &crate::bindings::pulumi::cloudflare::static_route::Args {
        account_id: &args.account_id.get_inner(),
        colo_names: &args.colo_names.get_inner(),
        colo_regions: &args.colo_regions.get_inner(),
        description: &args.description.get_inner(),
        nexthop: &args.nexthop.get_inner(),
        prefix: &args.prefix.get_inner(),
        priority: &args.priority.get_inner(),
        weight: &args.weight.get_inner(),
    });

    StaticRouteResult {
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
