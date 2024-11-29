//! Provides a Cloudflare Load Balancer resource. This sits in front of
//! a number of defined pools of origins and provides various options
//! for geographically-aware load balancing. Note that the load balancing
//! feature must be enabled in your Cloudflare account before you can use
//! this resource.
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as cloudflare from "@pulumi/cloudflare";
//! 
//! const exampleLoadBalancerPool = new cloudflare.LoadBalancerPool("example", {
//!     name: "example-lb-pool",
//!     origins: [{
//!         name: "example-1",
//!         address: "192.0.2.1",
//!         enabled: false,
//!     }],
//! });
//! // Define a load balancer which always points to a pool we define below.
//! // In normal usage, would have different pools set for different pops
//! // (cloudflare points-of-presence) and/or for different regions.
//! // Within each pop or region we can define multiple pools in failover order.
//! const example = new cloudflare.LoadBalancer("example", {
//!     zoneId: "0da42c8d2132a9ddaf714f9e7c920711",
//!     name: "example-load-balancer.example.com",
//!     fallbackPoolId: exampleLoadBalancerPool.id,
//!     defaultPoolIds: [exampleLoadBalancerPool.id],
//!     description: "example load balancer using geo-balancing",
//!     proxied: true,
//!     steeringPolicy: "geo",
//!     popPools: [{
//!         pop: "LAX",
//!         poolIds: [exampleLoadBalancerPool.id],
//!     }],
//!     countryPools: [{
//!         country: "US",
//!         poolIds: [exampleLoadBalancerPool.id],
//!     }],
//!     regionPools: [{
//!         region: "WNAM",
//!         poolIds: [exampleLoadBalancerPool.id],
//!     }],
//!     rules: [{
//!         name: "example rule",
//!         condition: "http.request.uri.path contains \"testing\"",
//!         fixedResponse: {
//!             messageBody: "hello",
//!             statusCode: 200,
//!             contentType: "html",
//!             location: "www.example.com",
//!         },
//!     }],
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//! 
//! example_load_balancer_pool = cloudflare.LoadBalancerPool("example",
//!     name="example-lb-pool",
//!     origins=[{
//!         "name": "example-1",
//!         "address": "192.0.2.1",
//!         "enabled": False,
//!     }])
//! # Define a load balancer which always points to a pool we define below.
//! # In normal usage, would have different pools set for different pops
//! # (cloudflare points-of-presence) and/or for different regions.
//! # Within each pop or region we can define multiple pools in failover order.
//! example = cloudflare.LoadBalancer("example",
//!     zone_id="0da42c8d2132a9ddaf714f9e7c920711",
//!     name="example-load-balancer.example.com",
//!     fallback_pool_id=example_load_balancer_pool.id,
//!     default_pool_ids=[example_load_balancer_pool.id],
//!     description="example load balancer using geo-balancing",
//!     proxied=True,
//!     steering_policy="geo",
//!     pop_pools=[{
//!         "pop": "LAX",
//!         "pool_ids": [example_load_balancer_pool.id],
//!     }],
//!     country_pools=[{
//!         "country": "US",
//!         "pool_ids": [example_load_balancer_pool.id],
//!     }],
//!     region_pools=[{
//!         "region": "WNAM",
//!         "pool_ids": [example_load_balancer_pool.id],
//!     }],
//!     rules=[{
//!         "name": "example rule",
//!         "condition": "http.request.uri.path contains \"testing\"",
//!         "fixed_response": {
//!             "message_body": "hello",
//!             "status_code": 200,
//!             "content_type": "html",
//!             "location": "www.example.com",
//!         },
//!     }])
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
//!     var exampleLoadBalancerPool = new Cloudflare.LoadBalancerPool("example", new()
//!     {
//!         Name = "example-lb-pool",
//!         Origins = new[]
//!         {
//!             new Cloudflare.Inputs.LoadBalancerPoolOriginArgs
//!             {
//!                 Name = "example-1",
//!                 Address = "192.0.2.1",
//!                 Enabled = false,
//!             },
//!         },
//!     });
//! 
//!     // Define a load balancer which always points to a pool we define below.
//!     // In normal usage, would have different pools set for different pops
//!     // (cloudflare points-of-presence) and/or for different regions.
//!     // Within each pop or region we can define multiple pools in failover order.
//!     var example = new Cloudflare.LoadBalancer("example", new()
//!     {
//!         ZoneId = "0da42c8d2132a9ddaf714f9e7c920711",
//!         Name = "example-load-balancer.example.com",
//!         FallbackPoolId = exampleLoadBalancerPool.Id,
//!         DefaultPoolIds = new[]
//!         {
//!             exampleLoadBalancerPool.Id,
//!         },
//!         Description = "example load balancer using geo-balancing",
//!         Proxied = true,
//!         SteeringPolicy = "geo",
//!         PopPools = new[]
//!         {
//!             new Cloudflare.Inputs.LoadBalancerPopPoolArgs
//!             {
//!                 Pop = "LAX",
//!                 PoolIds = new[]
//!                 {
//!                     exampleLoadBalancerPool.Id,
//!                 },
//!             },
//!         },
//!         CountryPools = new[]
//!         {
//!             new Cloudflare.Inputs.LoadBalancerCountryPoolArgs
//!             {
//!                 Country = "US",
//!                 PoolIds = new[]
//!                 {
//!                     exampleLoadBalancerPool.Id,
//!                 },
//!             },
//!         },
//!         RegionPools = new[]
//!         {
//!             new Cloudflare.Inputs.LoadBalancerRegionPoolArgs
//!             {
//!                 Region = "WNAM",
//!                 PoolIds = new[]
//!                 {
//!                     exampleLoadBalancerPool.Id,
//!                 },
//!             },
//!         },
//!         Rules = new[]
//!         {
//!             new Cloudflare.Inputs.LoadBalancerRuleArgs
//!             {
//!                 Name = "example rule",
//!                 Condition = "http.request.uri.path contains \"testing\"",
//!                 FixedResponse = new Cloudflare.Inputs.LoadBalancerRuleFixedResponseArgs
//!                 {
//!                     MessageBody = "hello",
//!                     StatusCode = 200,
//!                     ContentType = "html",
//!                     Location = "www.example.com",
//!                 },
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
//! 		exampleLoadBalancerPool, err := cloudflare.NewLoadBalancerPool(ctx, "example", &cloudflare.LoadBalancerPoolArgs{
//! 			Name: pulumi.String("example-lb-pool"),
//! 			Origins: cloudflare.LoadBalancerPoolOriginArray{
//! 				&cloudflare.LoadBalancerPoolOriginArgs{
//! 					Name:    pulumi.String("example-1"),
//! 					Address: pulumi.String("192.0.2.1"),
//! 					Enabled: pulumi.Bool(false),
//! 				},
//! 			},
//! 		})
//! 		if err != nil {
//! 			return err
//! 		}
//! 		// Define a load balancer which always points to a pool we define below.
//! 		// In normal usage, would have different pools set for different pops
//! 		// (cloudflare points-of-presence) and/or for different regions.
//! 		// Within each pop or region we can define multiple pools in failover order.
//! 		_, err = cloudflare.NewLoadBalancer(ctx, "example", &cloudflare.LoadBalancerArgs{
//! 			ZoneId:         pulumi.String("0da42c8d2132a9ddaf714f9e7c920711"),
//! 			Name:           pulumi.String("example-load-balancer.example.com"),
//! 			FallbackPoolId: exampleLoadBalancerPool.ID(),
//! 			DefaultPoolIds: pulumi.StringArray{
//! 				exampleLoadBalancerPool.ID(),
//! 			},
//! 			Description:    pulumi.String("example load balancer using geo-balancing"),
//! 			Proxied:        pulumi.Bool(true),
//! 			SteeringPolicy: pulumi.String("geo"),
//! 			PopPools: cloudflare.LoadBalancerPopPoolArray{
//! 				&cloudflare.LoadBalancerPopPoolArgs{
//! 					Pop: pulumi.String("LAX"),
//! 					PoolIds: pulumi.StringArray{
//! 						exampleLoadBalancerPool.ID(),
//! 					},
//! 				},
//! 			},
//! 			CountryPools: cloudflare.LoadBalancerCountryPoolArray{
//! 				&cloudflare.LoadBalancerCountryPoolArgs{
//! 					Country: pulumi.String("US"),
//! 					PoolIds: pulumi.StringArray{
//! 						exampleLoadBalancerPool.ID(),
//! 					},
//! 				},
//! 			},
//! 			RegionPools: cloudflare.LoadBalancerRegionPoolArray{
//! 				&cloudflare.LoadBalancerRegionPoolArgs{
//! 					Region: pulumi.String("WNAM"),
//! 					PoolIds: pulumi.StringArray{
//! 						exampleLoadBalancerPool.ID(),
//! 					},
//! 				},
//! 			},
//! 			Rules: cloudflare.LoadBalancerRuleArray{
//! 				&cloudflare.LoadBalancerRuleArgs{
//! 					Name:      pulumi.String("example rule"),
//! 					Condition: pulumi.String("http.request.uri.path contains \"testing\""),
//! 					FixedResponse: &cloudflare.LoadBalancerRuleFixedResponseArgs{
//! 						MessageBody: pulumi.String("hello"),
//! 						StatusCode:  pulumi.Int(200),
//! 						ContentType: pulumi.String("html"),
//! 						Location:    pulumi.String("www.example.com"),
//! 					},
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
//! import com.pulumi.cloudflare.inputs.LoadBalancerPoolOriginArgs;
//! import com.pulumi.cloudflare.LoadBalancer;
//! import com.pulumi.cloudflare.LoadBalancerArgs;
//! import com.pulumi.cloudflare.inputs.LoadBalancerPopPoolArgs;
//! import com.pulumi.cloudflare.inputs.LoadBalancerCountryPoolArgs;
//! import com.pulumi.cloudflare.inputs.LoadBalancerRegionPoolArgs;
//! import com.pulumi.cloudflare.inputs.LoadBalancerRuleArgs;
//! import com.pulumi.cloudflare.inputs.LoadBalancerRuleFixedResponseArgs;
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
//!         var exampleLoadBalancerPool = new LoadBalancerPool("exampleLoadBalancerPool", LoadBalancerPoolArgs.builder()
//!             .name("example-lb-pool")
//!             .origins(LoadBalancerPoolOriginArgs.builder()
//!                 .name("example-1")
//!                 .address("192.0.2.1")
//!                 .enabled(false)
//!                 .build())
//!             .build());
//! 
//!         // Define a load balancer which always points to a pool we define below.
//!         // In normal usage, would have different pools set for different pops
//!         // (cloudflare points-of-presence) and/or for different regions.
//!         // Within each pop or region we can define multiple pools in failover order.
//!         var example = new LoadBalancer("example", LoadBalancerArgs.builder()
//!             .zoneId("0da42c8d2132a9ddaf714f9e7c920711")
//!             .name("example-load-balancer.example.com")
//!             .fallbackPoolId(exampleLoadBalancerPool.id())
//!             .defaultPoolIds(exampleLoadBalancerPool.id())
//!             .description("example load balancer using geo-balancing")
//!             .proxied(true)
//!             .steeringPolicy("geo")
//!             .popPools(LoadBalancerPopPoolArgs.builder()
//!                 .pop("LAX")
//!                 .poolIds(exampleLoadBalancerPool.id())
//!                 .build())
//!             .countryPools(LoadBalancerCountryPoolArgs.builder()
//!                 .country("US")
//!                 .poolIds(exampleLoadBalancerPool.id())
//!                 .build())
//!             .regionPools(LoadBalancerRegionPoolArgs.builder()
//!                 .region("WNAM")
//!                 .poolIds(exampleLoadBalancerPool.id())
//!                 .build())
//!             .rules(LoadBalancerRuleArgs.builder()
//!                 .name("example rule")
//!                 .condition("http.request.uri.path contains \"testing\"")
//!                 .fixedResponse(LoadBalancerRuleFixedResponseArgs.builder()
//!                     .messageBody("hello")
//!                     .statusCode(200)
//!                     .contentType("html")
//!                     .location("www.example.com")
//!                     .build())
//!                 .build())
//!             .build());
//! 
//!     }
//! }
//! ```
//! ### YAML
//! ```yaml
//! resources:
//!   # Define a load balancer which always points to a pool we define below.
//!   # In normal usage, would have different pools set for different pops
//!   # (cloudflare points-of-presence) and/or for different regions.
//!   # Within each pop or region we can define multiple pools in failover order.
//!   example:
//!     type: cloudflare:LoadBalancer
//!     properties:
//!       zoneId: 0da42c8d2132a9ddaf714f9e7c920711
//!       name: example-load-balancer.example.com
//!       fallbackPoolId: ${exampleLoadBalancerPool.id}
//!       defaultPoolIds:
//!         - ${exampleLoadBalancerPool.id}
//!       description: example load balancer using geo-balancing
//!       proxied: true
//!       steeringPolicy: geo
//!       popPools:
//!         - pop: LAX
//!           poolIds:
//!             - ${exampleLoadBalancerPool.id}
//!       countryPools:
//!         - country: US
//!           poolIds:
//!             - ${exampleLoadBalancerPool.id}
//!       regionPools:
//!         - region: WNAM
//!           poolIds:
//!             - ${exampleLoadBalancerPool.id}
//!       rules:
//!         - name: example rule
//!           condition: http.request.uri.path contains "testing"
//!           fixedResponse:
//!             messageBody: hello
//!             statusCode: 200
//!             contentType: html
//!             location: www.example.com
//!   exampleLoadBalancerPool:
//!     type: cloudflare:LoadBalancerPool
//!     name: example
//!     properties:
//!       name: example-lb-pool
//!       origins:
//!         - name: example-1
//!           address: 192.0.2.1
//!           enabled: false
//! ```
//! <!--End PulumiCodeChooser -->
//! 
//! ## Import
//! 
//! ```sh
//! $ pulumi import cloudflare:index/loadBalancer:LoadBalancer example <zone_id>/<load_balancer_id>
//! ```
//! 

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct LoadBalancerArgs {
    /// Controls features that modify the routing of requests to pools and origins in response to dynamic conditions, such as during the interval between active health monitoring requests.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub adaptive_routings: pulumi_wasm_rust::Output<Option<Vec<crate::types::LoadBalancerAdaptiveRouting>>>,
    /// A set containing mappings of country codes to a list of pool IDs (ordered by their failover priority) for the given country.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub country_pools: pulumi_wasm_rust::Output<Option<Vec<crate::types::LoadBalancerCountryPool>>>,
    /// A list of pool IDs ordered by their failover priority. Used whenever `pop_pools`/`country_pools`/`region_pools` are not defined.
    #[builder(into)]
    pub default_pool_ids: pulumi_wasm_rust::Output<Vec<String>>,
    /// Free text description.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    /// Enable or disable the load balancer. Defaults to `true`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
    /// The pool ID to use when all other pools are detected as unhealthy.
    #[builder(into)]
    pub fallback_pool_id: pulumi_wasm_rust::Output<String>,
    /// Controls location-based steering for non-proxied requests.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub location_strategies: pulumi_wasm_rust::Output<Option<Vec<crate::types::LoadBalancerLocationStrategy>>>,
    /// The DNS hostname to associate with your load balancer. If this hostname already exists as a DNS record in Cloudflare's DNS, the load balancer will take precedence and the DNS record will not be used.
    #[builder(into)]
    pub name: pulumi_wasm_rust::Output<String>,
    /// A set containing mappings of Cloudflare Point-of-Presence (PoP) identifiers to a list of pool IDs (ordered by their failover priority) for the PoP (datacenter). This feature is only available to enterprise customers.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub pop_pools: pulumi_wasm_rust::Output<Option<Vec<crate::types::LoadBalancerPopPool>>>,
    /// Whether the hostname gets Cloudflare's origin protection. Defaults to `false`. Conflicts with `ttl`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub proxied: pulumi_wasm_rust::Output<Option<bool>>,
    /// Configures pool weights. When `steering_policy="random"`, a random pool is selected with probability proportional to pool weights. When `steering_policy="least_outstanding_requests"`, pool weights are used to scale each pool's outstanding requests. When `steering_policy="least_connections"`, pool weights are used to scale each pool's open connections.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub random_steerings: pulumi_wasm_rust::Output<Option<Vec<crate::types::LoadBalancerRandomSteering>>>,
    /// A set containing mappings of region codes to a list of pool IDs (ordered by their failover priority) for the given region.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub region_pools: pulumi_wasm_rust::Output<Option<Vec<crate::types::LoadBalancerRegionPool>>>,
    /// A list of rules for this load balancer to execute.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub rules: pulumi_wasm_rust::Output<Option<Vec<crate::types::LoadBalancerRule>>>,
    /// Specifies the type of session affinity the load balancer should use unless specified as `none` or `""` (default). With value `cookie`, on the first request to a proxied load balancer, a cookie is generated, encoding information of which origin the request will be forwarded to. Subsequent requests, by the same client to the same load balancer, will be sent to the origin server the cookie encodes, for the duration of the cookie and as long as the origin server remains healthy. If the cookie has expired or the origin server is unhealthy then a new origin server is calculated and used. Value `ip_cookie` behaves the same as `cookie` except the initial origin selection is stable and based on the client's IP address. Available values: `""`, `none`, `cookie`, `ip_cookie`, `header`. Defaults to `none`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub session_affinity: pulumi_wasm_rust::Output<Option<String>>,
    /// Configure attributes for session affinity.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub session_affinity_attributes: pulumi_wasm_rust::Output<Option<Vec<crate::types::LoadBalancerSessionAffinityAttribute>>>,
    /// Time, in seconds, until this load balancer's session affinity cookie expires after being created. This parameter is ignored unless a supported session affinity policy is set. The current default of `82800` (23 hours) will be used unless `session_affinity_ttl` is explicitly set. Once the expiry time has been reached, subsequent requests may get sent to a different origin server. Valid values are between `1800` and `604800`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub session_affinity_ttl: pulumi_wasm_rust::Output<Option<i32>>,
    /// The method the load balancer uses to determine the route to your origin. Value `off` uses `default_pool_ids`. Value `geo` uses `pop_pools`/`country_pools`/`region_pools`. For non-proxied requests, the `country` for `country_pools` is determined by `location_strategy`. Value `random` selects a pool randomly. Value `dynamic_latency` uses round trip time to select the closest pool in `default_pool_ids` (requires pool health checks). Value `proximity` uses the pools' latitude and longitude to select the closest pool using the Cloudflare PoP location for proxied requests or the location determined by `location_strategy` for non-proxied requests. Value `least_outstanding_requests` selects a pool by taking into consideration `random_steering` weights, as well as each pool's number of outstanding requests. Pools with more pending requests are weighted proportionately less relative to others. Value `least_connections` selects a pool by taking into consideration `random_steering` weights, as well as each pool's number of open connections. Pools with more open connections are weighted proportionately less relative to others. Supported for HTTP/1 and HTTP/2 connections. Value `""` maps to `geo` if you use `pop_pools`/`country_pools`/`region_pools` otherwise `off`. Available values: `off`, `geo`, `dynamic_latency`, `random`, `proximity`, `least_outstanding_requests`, `least_connections`, `""` Defaults to `""`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub steering_policy: pulumi_wasm_rust::Output<Option<String>>,
    /// Time to live (TTL) of the DNS entry for the IP address returned by this load balancer. This cannot be set for proxied load balancers. Defaults to `30`. Conflicts with `proxied`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub ttl: pulumi_wasm_rust::Output<Option<i32>>,
    /// The zone ID to add the load balancer to. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct LoadBalancerResult {
    /// Controls features that modify the routing of requests to pools and origins in response to dynamic conditions, such as during the interval between active health monitoring requests.
    pub adaptive_routings: pulumi_wasm_rust::Output<Option<Vec<crate::types::LoadBalancerAdaptiveRouting>>>,
    /// A set containing mappings of country codes to a list of pool IDs (ordered by their failover priority) for the given country.
    pub country_pools: pulumi_wasm_rust::Output<Option<Vec<crate::types::LoadBalancerCountryPool>>>,
    /// The RFC3339 timestamp of when the load balancer was created.
    pub created_on: pulumi_wasm_rust::Output<String>,
    /// A list of pool IDs ordered by their failover priority. Used whenever `pop_pools`/`country_pools`/`region_pools` are not defined.
    pub default_pool_ids: pulumi_wasm_rust::Output<Vec<String>>,
    /// Free text description.
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    /// Enable or disable the load balancer. Defaults to `true`.
    pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
    /// The pool ID to use when all other pools are detected as unhealthy.
    pub fallback_pool_id: pulumi_wasm_rust::Output<String>,
    /// Controls location-based steering for non-proxied requests.
    pub location_strategies: pulumi_wasm_rust::Output<Option<Vec<crate::types::LoadBalancerLocationStrategy>>>,
    /// The RFC3339 timestamp of when the load balancer was last modified.
    pub modified_on: pulumi_wasm_rust::Output<String>,
    /// The DNS hostname to associate with your load balancer. If this hostname already exists as a DNS record in Cloudflare's DNS, the load balancer will take precedence and the DNS record will not be used.
    pub name: pulumi_wasm_rust::Output<String>,
    /// A set containing mappings of Cloudflare Point-of-Presence (PoP) identifiers to a list of pool IDs (ordered by their failover priority) for the PoP (datacenter). This feature is only available to enterprise customers.
    pub pop_pools: pulumi_wasm_rust::Output<Option<Vec<crate::types::LoadBalancerPopPool>>>,
    /// Whether the hostname gets Cloudflare's origin protection. Defaults to `false`. Conflicts with `ttl`.
    pub proxied: pulumi_wasm_rust::Output<Option<bool>>,
    /// Configures pool weights. When `steering_policy="random"`, a random pool is selected with probability proportional to pool weights. When `steering_policy="least_outstanding_requests"`, pool weights are used to scale each pool's outstanding requests. When `steering_policy="least_connections"`, pool weights are used to scale each pool's open connections.
    pub random_steerings: pulumi_wasm_rust::Output<Option<Vec<crate::types::LoadBalancerRandomSteering>>>,
    /// A set containing mappings of region codes to a list of pool IDs (ordered by their failover priority) for the given region.
    pub region_pools: pulumi_wasm_rust::Output<Option<Vec<crate::types::LoadBalancerRegionPool>>>,
    /// A list of rules for this load balancer to execute.
    pub rules: pulumi_wasm_rust::Output<Option<Vec<crate::types::LoadBalancerRule>>>,
    /// Specifies the type of session affinity the load balancer should use unless specified as `none` or `""` (default). With value `cookie`, on the first request to a proxied load balancer, a cookie is generated, encoding information of which origin the request will be forwarded to. Subsequent requests, by the same client to the same load balancer, will be sent to the origin server the cookie encodes, for the duration of the cookie and as long as the origin server remains healthy. If the cookie has expired or the origin server is unhealthy then a new origin server is calculated and used. Value `ip_cookie` behaves the same as `cookie` except the initial origin selection is stable and based on the client's IP address. Available values: `""`, `none`, `cookie`, `ip_cookie`, `header`. Defaults to `none`.
    pub session_affinity: pulumi_wasm_rust::Output<Option<String>>,
    /// Configure attributes for session affinity.
    pub session_affinity_attributes: pulumi_wasm_rust::Output<Option<Vec<crate::types::LoadBalancerSessionAffinityAttribute>>>,
    /// Time, in seconds, until this load balancer's session affinity cookie expires after being created. This parameter is ignored unless a supported session affinity policy is set. The current default of `82800` (23 hours) will be used unless `session_affinity_ttl` is explicitly set. Once the expiry time has been reached, subsequent requests may get sent to a different origin server. Valid values are between `1800` and `604800`.
    pub session_affinity_ttl: pulumi_wasm_rust::Output<Option<i32>>,
    /// The method the load balancer uses to determine the route to your origin. Value `off` uses `default_pool_ids`. Value `geo` uses `pop_pools`/`country_pools`/`region_pools`. For non-proxied requests, the `country` for `country_pools` is determined by `location_strategy`. Value `random` selects a pool randomly. Value `dynamic_latency` uses round trip time to select the closest pool in `default_pool_ids` (requires pool health checks). Value `proximity` uses the pools' latitude and longitude to select the closest pool using the Cloudflare PoP location for proxied requests or the location determined by `location_strategy` for non-proxied requests. Value `least_outstanding_requests` selects a pool by taking into consideration `random_steering` weights, as well as each pool's number of outstanding requests. Pools with more pending requests are weighted proportionately less relative to others. Value `least_connections` selects a pool by taking into consideration `random_steering` weights, as well as each pool's number of open connections. Pools with more open connections are weighted proportionately less relative to others. Supported for HTTP/1 and HTTP/2 connections. Value `""` maps to `geo` if you use `pop_pools`/`country_pools`/`region_pools` otherwise `off`. Available values: `off`, `geo`, `dynamic_latency`, `random`, `proximity`, `least_outstanding_requests`, `least_connections`, `""` Defaults to `""`.
    pub steering_policy: pulumi_wasm_rust::Output<String>,
    /// Time to live (TTL) of the DNS entry for the IP address returned by this load balancer. This cannot be set for proxied load balancers. Defaults to `30`. Conflicts with `proxied`.
    pub ttl: pulumi_wasm_rust::Output<i32>,
    /// The zone ID to add the load balancer to. **Modifying this attribute will force creation of a new resource.**
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: LoadBalancerArgs) -> LoadBalancerResult {

    let result = crate::bindings::pulumi::cloudflare::load_balancer::invoke(name, &crate::bindings::pulumi::cloudflare::load_balancer::Args {
        adaptive_routings: &args.adaptive_routings.get_inner(),
        country_pools: &args.country_pools.get_inner(),
        default_pool_ids: &args.default_pool_ids.get_inner(),
        description: &args.description.get_inner(),
        enabled: &args.enabled.get_inner(),
        fallback_pool_id: &args.fallback_pool_id.get_inner(),
        location_strategies: &args.location_strategies.get_inner(),
        name: &args.name.get_inner(),
        pop_pools: &args.pop_pools.get_inner(),
        proxied: &args.proxied.get_inner(),
        random_steerings: &args.random_steerings.get_inner(),
        region_pools: &args.region_pools.get_inner(),
        rules: &args.rules.get_inner(),
        session_affinity: &args.session_affinity.get_inner(),
        session_affinity_attributes: &args.session_affinity_attributes.get_inner(),
        session_affinity_ttl: &args.session_affinity_ttl.get_inner(),
        steering_policy: &args.steering_policy.get_inner(),
        ttl: &args.ttl.get_inner(),
        zone_id: &args.zone_id.get_inner(),
    });

    LoadBalancerResult {
        adaptive_routings: crate::into_domain(result.adaptive_routings),
        country_pools: crate::into_domain(result.country_pools),
        created_on: crate::into_domain(result.created_on),
        default_pool_ids: crate::into_domain(result.default_pool_ids),
        description: crate::into_domain(result.description),
        enabled: crate::into_domain(result.enabled),
        fallback_pool_id: crate::into_domain(result.fallback_pool_id),
        location_strategies: crate::into_domain(result.location_strategies),
        modified_on: crate::into_domain(result.modified_on),
        name: crate::into_domain(result.name),
        pop_pools: crate::into_domain(result.pop_pools),
        proxied: crate::into_domain(result.proxied),
        random_steerings: crate::into_domain(result.random_steerings),
        region_pools: crate::into_domain(result.region_pools),
        rules: crate::into_domain(result.rules),
        session_affinity: crate::into_domain(result.session_affinity),
        session_affinity_attributes: crate::into_domain(result.session_affinity_attributes),
        session_affinity_ttl: crate::into_domain(result.session_affinity_ttl),
        steering_policy: crate::into_domain(result.steering_policy),
        ttl: crate::into_domain(result.ttl),
        zone_id: crate::into_domain(result.zone_id),
    }
}
