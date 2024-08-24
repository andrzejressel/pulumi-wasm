//! Provides a Cloudflare Load Balancer pool resource. This provides a
//! pool of origins that can be used by a Cloudflare Load Balancer.
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as cloudflare from "@pulumi/cloudflare";
//! 
//! const example = new cloudflare.LoadBalancerPool("example", {
//!     accountId: "f037e56e89293a057740de681ac9abbe",
//!     description: "example load balancer pool",
//!     enabled: false,
//!     latitude: 55,
//!     loadSheddings: [{
//!         defaultPercent: 55,
//!         defaultPolicy: "random",
//!         sessionPercent: 12,
//!         sessionPolicy: "hash",
//!     }],
//!     longitude: -12,
//!     minimumOrigins: 1,
//!     name: "example-pool",
//!     notificationEmail: "someone@example.com",
//!     originSteerings: [{
//!         policy: "random",
//!     }],
//!     origins: [
//!         {
//!             address: "192.0.2.1",
//!             enabled: false,
//!             headers: [{
//!                 header: "Host",
//!                 values: ["example-1"],
//!             }],
//!             name: "example-1",
//!         },
//!         {
//!             address: "192.0.2.2",
//!             headers: [{
//!                 header: "Host",
//!                 values: ["example-2"],
//!             }],
//!             name: "example-2",
//!         },
//!     ],
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//! 
//! example = cloudflare.LoadBalancerPool("example",
//!     account_id="f037e56e89293a057740de681ac9abbe",
//!     description="example load balancer pool",
//!     enabled=False,
//!     latitude=55,
//!     load_sheddings=[cloudflare.LoadBalancerPoolLoadSheddingArgs(
//!         default_percent=55,
//!         default_policy="random",
//!         session_percent=12,
//!         session_policy="hash",
//!     )],
//!     longitude=-12,
//!     minimum_origins=1,
//!     name="example-pool",
//!     notification_email="someone@example.com",
//!     origin_steerings=[cloudflare.LoadBalancerPoolOriginSteeringArgs(
//!         policy="random",
//!     )],
//!     origins=[
//!         cloudflare.LoadBalancerPoolOriginArgs(
//!             address="192.0.2.1",
//!             enabled=False,
//!             headers=[cloudflare.LoadBalancerPoolOriginHeaderArgs(
//!                 header="Host",
//!                 values=["example-1"],
//!             )],
//!             name="example-1",
//!         ),
//!         cloudflare.LoadBalancerPoolOriginArgs(
//!             address="192.0.2.2",
//!             headers=[cloudflare.LoadBalancerPoolOriginHeaderArgs(
//!                 header="Host",
//!                 values=["example-2"],
//!             )],
//!             name="example-2",
//!         ),
//!     ])
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
//!     var example = new Cloudflare.LoadBalancerPool("example", new()
//!     {
//!         AccountId = "f037e56e89293a057740de681ac9abbe",
//!         Description = "example load balancer pool",
//!         Enabled = false,
//!         Latitude = 55,
//!         LoadSheddings = new[]
//!         {
//!             new Cloudflare.Inputs.LoadBalancerPoolLoadSheddingArgs
//!             {
//!                 DefaultPercent = 55,
//!                 DefaultPolicy = "random",
//!                 SessionPercent = 12,
//!                 SessionPolicy = "hash",
//!             },
//!         },
//!         Longitude = -12,
//!         MinimumOrigins = 1,
//!         Name = "example-pool",
//!         NotificationEmail = "someone@example.com",
//!         OriginSteerings = new[]
//!         {
//!             new Cloudflare.Inputs.LoadBalancerPoolOriginSteeringArgs
//!             {
//!                 Policy = "random",
//!             },
//!         },
//!         Origins = new[]
//!         {
//!             new Cloudflare.Inputs.LoadBalancerPoolOriginArgs
//!             {
//!                 Address = "192.0.2.1",
//!                 Enabled = false,
//!                 Headers = new[]
//!                 {
//!                     new Cloudflare.Inputs.LoadBalancerPoolOriginHeaderArgs
//!                     {
//!                         Header = "Host",
//!                         Values = new[]
//!                         {
//!                             "example-1",
//!                         },
//!                     },
//!                 },
//!                 Name = "example-1",
//!             },
//!             new Cloudflare.Inputs.LoadBalancerPoolOriginArgs
//!             {
//!                 Address = "192.0.2.2",
//!                 Headers = new[]
//!                 {
//!                     new Cloudflare.Inputs.LoadBalancerPoolOriginHeaderArgs
//!                     {
//!                         Header = "Host",
//!                         Values = new[]
//!                         {
//!                             "example-2",
//!                         },
//!                     },
//!                 },
//!                 Name = "example-2",
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
//! 	"github.com/pulumi/pulumi-cloudflare/sdk/v5/go/cloudflare"
//! 	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
//! )
//! 
//! func main() {
//! 	pulumi.Run(func(ctx *pulumi.Context) error {
//! 		_, err := cloudflare.NewLoadBalancerPool(ctx, "example", &cloudflare.LoadBalancerPoolArgs{
//! 			AccountId:   pulumi.String("f037e56e89293a057740de681ac9abbe"),
//! 			Description: pulumi.String("example load balancer pool"),
//! 			Enabled:     pulumi.Bool(false),
//! 			Latitude:    pulumi.Float64(55),
//! 			LoadSheddings: cloudflare.LoadBalancerPoolLoadSheddingArray{
//! 				&cloudflare.LoadBalancerPoolLoadSheddingArgs{
//! 					DefaultPercent: pulumi.Float64(55),
//! 					DefaultPolicy:  pulumi.String("random"),
//! 					SessionPercent: pulumi.Float64(12),
//! 					SessionPolicy:  pulumi.String("hash"),
//! 				},
//! 			},
//! 			Longitude:         -12,
//! 			MinimumOrigins:    pulumi.Int(1),
//! 			Name:              pulumi.String("example-pool"),
//! 			NotificationEmail: pulumi.String("someone@example.com"),
//! 			OriginSteerings: cloudflare.LoadBalancerPoolOriginSteeringArray{
//! 				&cloudflare.LoadBalancerPoolOriginSteeringArgs{
//! 					Policy: pulumi.String("random"),
//! 				},
//! 			},
//! 			Origins: cloudflare.LoadBalancerPoolOriginArray{
//! 				&cloudflare.LoadBalancerPoolOriginArgs{
//! 					Address: pulumi.String("192.0.2.1"),
//! 					Enabled: pulumi.Bool(false),
//! 					Headers: cloudflare.LoadBalancerPoolOriginHeaderArray{
//! 						&cloudflare.LoadBalancerPoolOriginHeaderArgs{
//! 							Header: pulumi.String("Host"),
//! 							Values: pulumi.StringArray{
//! 								pulumi.String("example-1"),
//! 							},
//! 						},
//! 					},
//! 					Name: pulumi.String("example-1"),
//! 				},
//! 				&cloudflare.LoadBalancerPoolOriginArgs{
//! 					Address: pulumi.String("192.0.2.2"),
//! 					Headers: cloudflare.LoadBalancerPoolOriginHeaderArray{
//! 						&cloudflare.LoadBalancerPoolOriginHeaderArgs{
//! 							Header: pulumi.String("Host"),
//! 							Values: pulumi.StringArray{
//! 								pulumi.String("example-2"),
//! 							},
//! 						},
//! 					},
//! 					Name: pulumi.String("example-2"),
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
//! import com.pulumi.cloudflare.LoadBalancerPool;
//! import com.pulumi.cloudflare.LoadBalancerPoolArgs;
//! import com.pulumi.cloudflare.inputs.LoadBalancerPoolLoadSheddingArgs;
//! import com.pulumi.cloudflare.inputs.LoadBalancerPoolOriginSteeringArgs;
//! import com.pulumi.cloudflare.inputs.LoadBalancerPoolOriginArgs;
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
//!         var example = new LoadBalancerPool("example", LoadBalancerPoolArgs.builder()        
//!             .accountId("f037e56e89293a057740de681ac9abbe")
//!             .description("example load balancer pool")
//!             .enabled(false)
//!             .latitude(55)
//!             .loadSheddings(LoadBalancerPoolLoadSheddingArgs.builder()
//!                 .defaultPercent(55)
//!                 .defaultPolicy("random")
//!                 .sessionPercent(12)
//!                 .sessionPolicy("hash")
//!                 .build())
//!             .longitude("TODO: GenUnaryOpExpression")
//!             .minimumOrigins(1)
//!             .name("example-pool")
//!             .notificationEmail("someone@example.com")
//!             .originSteerings(LoadBalancerPoolOriginSteeringArgs.builder()
//!                 .policy("random")
//!                 .build())
//!             .origins(            
//!                 LoadBalancerPoolOriginArgs.builder()
//!                     .address("192.0.2.1")
//!                     .enabled(false)
//!                     .headers(LoadBalancerPoolOriginHeaderArgs.builder()
//!                         .header("Host")
//!                         .values("example-1")
//!                         .build())
//!                     .name("example-1")
//!                     .build(),
//!                 LoadBalancerPoolOriginArgs.builder()
//!                     .address("192.0.2.2")
//!                     .headers(LoadBalancerPoolOriginHeaderArgs.builder()
//!                         .header("Host")
//!                         .values("example-2")
//!                         .build())
//!                     .name("example-2")
//!                     .build())
//!             .build());
//! 
//!     }
//! }
//! ```
//! <!--End PulumiCodeChooser -->
//! 
//! ## Import
//! 
//! ```sh
//! $ pulumi import cloudflare:index/loadBalancerPool:LoadBalancerPool example <account_id>/<load_balancer_pool_id>
//! ```
//! 

pub struct LoadBalancerPoolArgs {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// A list of regions (specified by region code) from which to run health checks. Empty means every Cloudflare data center (the default), but requires an Enterprise plan. Region codes can be found [here](https://developers.cloudflare.com/load-balancing/reference/region-mapping-api).
    pub check_regions: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    /// Free text description.
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    /// Whether this origin is enabled. Disabled origins will not receive traffic and are excluded from health checks. Defaults to `true`.
    pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
    /// The latitude this pool is physically located at; used for proximity steering.
    pub latitude: pulumi_wasm_rust::Output<Option<f64>>,
    /// Setting for controlling load shedding for this pool.
    pub load_sheddings: pulumi_wasm_rust::Output<Option<Vec<crate::types::LoadBalancerPoolLoadShedding>>>,
    /// The longitude this pool is physically located at; used for proximity steering.
    pub longitude: pulumi_wasm_rust::Output<Option<f64>>,
    /// The minimum number of origins that must be healthy for this pool to serve traffic. If the number of healthy origins falls below this number, the pool will be marked unhealthy and we will failover to the next available pool. Defaults to `1`.
    pub minimum_origins: pulumi_wasm_rust::Output<Option<i32>>,
    /// The ID of the Monitor to use for health checking origins within this pool.
    pub monitor: pulumi_wasm_rust::Output<Option<String>>,
    /// A human-identifiable name for the origin.
    pub name: pulumi_wasm_rust::Output<String>,
    /// The email address to send health status notifications to. This can be an individual mailbox or a mailing list. Multiple emails can be supplied as a comma delimited list.
    pub notification_email: pulumi_wasm_rust::Output<Option<String>>,
    /// Set an origin steering policy to control origin selection within a pool.
    pub origin_steerings: pulumi_wasm_rust::Output<Option<Vec<crate::types::LoadBalancerPoolOriginSteering>>>,
    /// The list of origins within this pool. Traffic directed at this pool is balanced across all currently healthy origins, provided the pool itself is healthy.
    pub origins: pulumi_wasm_rust::Output<Vec<crate::types::LoadBalancerPoolOrigin>>,
}

pub struct LoadBalancerPoolResult {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// A list of regions (specified by region code) from which to run health checks. Empty means every Cloudflare data center (the default), but requires an Enterprise plan. Region codes can be found [here](https://developers.cloudflare.com/load-balancing/reference/region-mapping-api).
    pub check_regions: pulumi_wasm_rust::Output<Vec<String>>,
    /// The RFC3339 timestamp of when the load balancer was created.
    pub created_on: pulumi_wasm_rust::Output<String>,
    /// Free text description.
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    /// Whether this origin is enabled. Disabled origins will not receive traffic and are excluded from health checks. Defaults to `true`.
    pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
    /// The latitude this pool is physically located at; used for proximity steering.
    pub latitude: pulumi_wasm_rust::Output<Option<f64>>,
    /// Setting for controlling load shedding for this pool.
    pub load_sheddings: pulumi_wasm_rust::Output<Option<Vec<crate::types::LoadBalancerPoolLoadShedding>>>,
    /// The longitude this pool is physically located at; used for proximity steering.
    pub longitude: pulumi_wasm_rust::Output<Option<f64>>,
    /// The minimum number of origins that must be healthy for this pool to serve traffic. If the number of healthy origins falls below this number, the pool will be marked unhealthy and we will failover to the next available pool. Defaults to `1`.
    pub minimum_origins: pulumi_wasm_rust::Output<Option<i32>>,
    /// The RFC3339 timestamp of when the load balancer was last modified.
    pub modified_on: pulumi_wasm_rust::Output<String>,
    /// The ID of the Monitor to use for health checking origins within this pool.
    pub monitor: pulumi_wasm_rust::Output<Option<String>>,
    /// A human-identifiable name for the origin.
    pub name: pulumi_wasm_rust::Output<String>,
    /// The email address to send health status notifications to. This can be an individual mailbox or a mailing list. Multiple emails can be supplied as a comma delimited list.
    pub notification_email: pulumi_wasm_rust::Output<Option<String>>,
    /// Set an origin steering policy to control origin selection within a pool.
    pub origin_steerings: pulumi_wasm_rust::Output<Option<Vec<crate::types::LoadBalancerPoolOriginSteering>>>,
    /// The list of origins within this pool. Traffic directed at this pool is balanced across all currently healthy origins, provided the pool itself is healthy.
    pub origins: pulumi_wasm_rust::Output<Vec<crate::types::LoadBalancerPoolOrigin>>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: LoadBalancerPoolArgs) -> LoadBalancerPoolResult {

    let result = crate::bindings::pulumi::cloudflare::load_balancer_pool::invoke(name, &crate::bindings::pulumi::cloudflare::load_balancer_pool::Args {
        account_id: args.account_id.get_inner(),
        check_regions: args.check_regions.get_inner(),
        description: args.description.get_inner(),
        enabled: args.enabled.get_inner(),
        latitude: args.latitude.get_inner(),
        load_sheddings: args.load_sheddings.get_inner(),
        longitude: args.longitude.get_inner(),
        minimum_origins: args.minimum_origins.get_inner(),
        monitor: args.monitor.get_inner(),
        name: args.name.get_inner(),
        notification_email: args.notification_email.get_inner(),
        origin_steerings: args.origin_steerings.get_inner(),
        origins: args.origins.get_inner(),
    });

    LoadBalancerPoolResult {
        account_id: crate::into_domain(result.account_id),
        check_regions: crate::into_domain(result.check_regions),
        created_on: crate::into_domain(result.created_on),
        description: crate::into_domain(result.description),
        enabled: crate::into_domain(result.enabled),
        latitude: crate::into_domain(result.latitude),
        load_sheddings: crate::into_domain(result.load_sheddings),
        longitude: crate::into_domain(result.longitude),
        minimum_origins: crate::into_domain(result.minimum_origins),
        modified_on: crate::into_domain(result.modified_on),
        monitor: crate::into_domain(result.monitor),
        name: crate::into_domain(result.name),
        notification_email: crate::into_domain(result.notification_email),
        origin_steerings: crate::into_domain(result.origin_steerings),
        origins: crate::into_domain(result.origins),
    }
}
