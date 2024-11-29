//! Provides a Cloudflare rate limit resource for a given zone. This can
//! be used to limit the traffic you receive zone-wide, or matching more
//! specific types of requests/responses.
//! 
//! > `cloudflare.RateLimit` is in a deprecation phase until January 15th, 2025.
//!   During this time period, this resource is still
//!   fully supported but you are strongly advised to move to the
//!   `cloudflare.Ruleset` resource. Full details can be found in the
//!   developer documentation.
//! 
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as cloudflare from "@pulumi/cloudflare";
//! 
//! const example = new cloudflare.RateLimit("example", {
//!     zoneId: "0da42c8d2132a9ddaf714f9e7c920711",
//!     threshold: 2000,
//!     period: 2,
//!     match: {
//!         request: {
//!             urlPattern: `${cloudflareZone}/*`,
//!             schemes: [
//!                 "HTTP",
//!                 "HTTPS",
//!             ],
//!             methods: [
//!                 "GET",
//!                 "POST",
//!                 "PUT",
//!                 "DELETE",
//!                 "PATCH",
//!                 "HEAD",
//!             ],
//!         },
//!         response: {
//!             statuses: [
//!                 200,
//!                 201,
//!                 202,
//!                 301,
//!                 429,
//!             ],
//!             originTraffic: false,
//!             headers: [
//!                 {
//!                     name: "Host",
//!                     op: "eq",
//!                     value: "localhost",
//!                 },
//!                 {
//!                     name: "X-Example",
//!                     op: "ne",
//!                     value: "my-example",
//!                 },
//!             ],
//!         },
//!     },
//!     action: {
//!         mode: "simulate",
//!         timeout: 43200,
//!         response: {
//!             contentType: "text/plain",
//!             body: "custom response body",
//!         },
//!     },
//!     correlate: {
//!         by: "nat",
//!     },
//!     disabled: false,
//!     description: "example rate limit for a zone",
//!     bypassUrlPatterns: [
//!         "example.com/bypass1",
//!         "example.com/bypass2",
//!     ],
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//! 
//! example = cloudflare.RateLimit("example",
//!     zone_id="0da42c8d2132a9ddaf714f9e7c920711",
//!     threshold=2000,
//!     period=2,
//!     match={
//!         "request": {
//!             "url_pattern": f"{cloudflare_zone}/*",
//!             "schemes": [
//!                 "HTTP",
//!                 "HTTPS",
//!             ],
//!             "methods": [
//!                 "GET",
//!                 "POST",
//!                 "PUT",
//!                 "DELETE",
//!                 "PATCH",
//!                 "HEAD",
//!             ],
//!         },
//!         "response": {
//!             "statuses": [
//!                 200,
//!                 201,
//!                 202,
//!                 301,
//!                 429,
//!             ],
//!             "origin_traffic": False,
//!             "headers": [
//!                 {
//!                     "name": "Host",
//!                     "op": "eq",
//!                     "value": "localhost",
//!                 },
//!                 {
//!                     "name": "X-Example",
//!                     "op": "ne",
//!                     "value": "my-example",
//!                 },
//!             ],
//!         },
//!     },
//!     action={
//!         "mode": "simulate",
//!         "timeout": 43200,
//!         "response": {
//!             "content_type": "text/plain",
//!             "body": "custom response body",
//!         },
//!     },
//!     correlate={
//!         "by": "nat",
//!     },
//!     disabled=False,
//!     description="example rate limit for a zone",
//!     bypass_url_patterns=[
//!         "example.com/bypass1",
//!         "example.com/bypass2",
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
//!     var example = new Cloudflare.RateLimit("example", new()
//!     {
//!         ZoneId = "0da42c8d2132a9ddaf714f9e7c920711",
//!         Threshold = 2000,
//!         Period = 2,
//!         Match = new Cloudflare.Inputs.RateLimitMatchArgs
//!         {
//!             Request = new Cloudflare.Inputs.RateLimitMatchRequestArgs
//!             {
//!                 UrlPattern = $"{cloudflareZone}/*",
//!                 Schemes = new[]
//!                 {
//!                     "HTTP",
//!                     "HTTPS",
//!                 },
//!                 Methods = new[]
//!                 {
//!                     "GET",
//!                     "POST",
//!                     "PUT",
//!                     "DELETE",
//!                     "PATCH",
//!                     "HEAD",
//!                 },
//!             },
//!             Response = new Cloudflare.Inputs.RateLimitMatchResponseArgs
//!             {
//!                 Statuses = new[]
//!                 {
//!                     200,
//!                     201,
//!                     202,
//!                     301,
//!                     429,
//!                 },
//!                 OriginTraffic = false,
//!                 Headers = new[]
//!                 {
//!                     
//!                     {
//!                         { "name", "Host" },
//!                         { "op", "eq" },
//!                         { "value", "localhost" },
//!                     },
//!                     
//!                     {
//!                         { "name", "X-Example" },
//!                         { "op", "ne" },
//!                         { "value", "my-example" },
//!                     },
//!                 },
//!             },
//!         },
//!         Action = new Cloudflare.Inputs.RateLimitActionArgs
//!         {
//!             Mode = "simulate",
//!             Timeout = 43200,
//!             Response = new Cloudflare.Inputs.RateLimitActionResponseArgs
//!             {
//!                 ContentType = "text/plain",
//!                 Body = "custom response body",
//!             },
//!         },
//!         Correlate = new Cloudflare.Inputs.RateLimitCorrelateArgs
//!         {
//!             By = "nat",
//!         },
//!         Disabled = false,
//!         Description = "example rate limit for a zone",
//!         BypassUrlPatterns = new[]
//!         {
//!             "example.com/bypass1",
//!             "example.com/bypass2",
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
//! 	"fmt"
//! 
//! 	"github.com/pulumi/pulumi-cloudflare/sdk/v5/go/cloudflare"
//! 	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
//! )
//! 
//! func main() {
//! 	pulumi.Run(func(ctx *pulumi.Context) error {
//! 		_, err := cloudflare.NewRateLimit(ctx, "example", &cloudflare.RateLimitArgs{
//! 			ZoneId:    pulumi.String("0da42c8d2132a9ddaf714f9e7c920711"),
//! 			Threshold: pulumi.Int(2000),
//! 			Period:    pulumi.Int(2),
//! 			Match: &cloudflare.RateLimitMatchArgs{
//! 				Request: &cloudflare.RateLimitMatchRequestArgs{
//! 					UrlPattern: pulumi.Sprintf("%v/*", cloudflareZone),
//! 					Schemes: pulumi.StringArray{
//! 						pulumi.String("HTTP"),
//! 						pulumi.String("HTTPS"),
//! 					},
//! 					Methods: pulumi.StringArray{
//! 						pulumi.String("GET"),
//! 						pulumi.String("POST"),
//! 						pulumi.String("PUT"),
//! 						pulumi.String("DELETE"),
//! 						pulumi.String("PATCH"),
//! 						pulumi.String("HEAD"),
//! 					},
//! 				},
//! 				Response: &cloudflare.RateLimitMatchResponseArgs{
//! 					Statuses: pulumi.IntArray{
//! 						pulumi.Int(200),
//! 						pulumi.Int(201),
//! 						pulumi.Int(202),
//! 						pulumi.Int(301),
//! 						pulumi.Int(429),
//! 					},
//! 					OriginTraffic: pulumi.Bool(false),
//! 					Headers: pulumi.StringMapArray{
//! 						pulumi.StringMap{
//! 							"name":  pulumi.String("Host"),
//! 							"op":    pulumi.String("eq"),
//! 							"value": pulumi.String("localhost"),
//! 						},
//! 						pulumi.StringMap{
//! 							"name":  pulumi.String("X-Example"),
//! 							"op":    pulumi.String("ne"),
//! 							"value": pulumi.String("my-example"),
//! 						},
//! 					},
//! 				},
//! 			},
//! 			Action: &cloudflare.RateLimitActionArgs{
//! 				Mode:    pulumi.String("simulate"),
//! 				Timeout: pulumi.Int(43200),
//! 				Response: &cloudflare.RateLimitActionResponseArgs{
//! 					ContentType: pulumi.String("text/plain"),
//! 					Body:        pulumi.String("custom response body"),
//! 				},
//! 			},
//! 			Correlate: &cloudflare.RateLimitCorrelateArgs{
//! 				By: pulumi.String("nat"),
//! 			},
//! 			Disabled:    pulumi.Bool(false),
//! 			Description: pulumi.String("example rate limit for a zone"),
//! 			BypassUrlPatterns: pulumi.StringArray{
//! 				pulumi.String("example.com/bypass1"),
//! 				pulumi.String("example.com/bypass2"),
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
//! import com.pulumi.cloudflare.RateLimit;
//! import com.pulumi.cloudflare.RateLimitArgs;
//! import com.pulumi.cloudflare.inputs.RateLimitMatchArgs;
//! import com.pulumi.cloudflare.inputs.RateLimitMatchRequestArgs;
//! import com.pulumi.cloudflare.inputs.RateLimitMatchResponseArgs;
//! import com.pulumi.cloudflare.inputs.RateLimitActionArgs;
//! import com.pulumi.cloudflare.inputs.RateLimitActionResponseArgs;
//! import com.pulumi.cloudflare.inputs.RateLimitCorrelateArgs;
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
//!         var example = new RateLimit("example", RateLimitArgs.builder()
//!             .zoneId("0da42c8d2132a9ddaf714f9e7c920711")
//!             .threshold(2000)
//!             .period(2)
//!             .match(RateLimitMatchArgs.builder()
//!                 .request(RateLimitMatchRequestArgs.builder()
//!                     .urlPattern(String.format("%s/*", cloudflareZone))
//!                     .schemes(                    
//!                         "HTTP",
//!                         "HTTPS")
//!                     .methods(                    
//!                         "GET",
//!                         "POST",
//!                         "PUT",
//!                         "DELETE",
//!                         "PATCH",
//!                         "HEAD")
//!                     .build())
//!                 .response(RateLimitMatchResponseArgs.builder()
//!                     .statuses(                    
//!                         200,
//!                         201,
//!                         202,
//!                         301,
//!                         429)
//!                     .originTraffic(false)
//!                     .headers(                    
//!                         Map.ofEntries(
//!                             Map.entry("name", "Host"),
//!                             Map.entry("op", "eq"),
//!                             Map.entry("value", "localhost")
//!                         ),
//!                         Map.ofEntries(
//!                             Map.entry("name", "X-Example"),
//!                             Map.entry("op", "ne"),
//!                             Map.entry("value", "my-example")
//!                         ))
//!                     .build())
//!                 .build())
//!             .action(RateLimitActionArgs.builder()
//!                 .mode("simulate")
//!                 .timeout(43200)
//!                 .response(RateLimitActionResponseArgs.builder()
//!                     .contentType("text/plain")
//!                     .body("custom response body")
//!                     .build())
//!                 .build())
//!             .correlate(RateLimitCorrelateArgs.builder()
//!                 .by("nat")
//!                 .build())
//!             .disabled(false)
//!             .description("example rate limit for a zone")
//!             .bypassUrlPatterns(            
//!                 "example.com/bypass1",
//!                 "example.com/bypass2")
//!             .build());
//! 
//!     }
//! }
//! ```
//! ### YAML
//! ```yaml
//! resources:
//!   example:
//!     type: cloudflare:RateLimit
//!     properties:
//!       zoneId: 0da42c8d2132a9ddaf714f9e7c920711
//!       threshold: 2000
//!       period: 2
//!       match:
//!         request:
//!           urlPattern: ${cloudflareZone}/*
//!           schemes:
//!             - HTTP
//!             - HTTPS
//!           methods:
//!             - GET
//!             - POST
//!             - PUT
//!             - DELETE
//!             - PATCH
//!             - HEAD
//!         response:
//!           statuses:
//!             - 200
//!             - 201
//!             - 202
//!             - 301
//!             - 429
//!           originTraffic: false
//!           headers:
//!             - name: Host
//!               op: eq
//!               value: localhost
//!             - name: X-Example
//!               op: ne
//!               value: my-example
//!       action:
//!         mode: simulate
//!         timeout: 43200
//!         response:
//!           contentType: text/plain
//!           body: custom response body
//!       correlate:
//!         by: nat
//!       disabled: false
//!       description: example rate limit for a zone
//!       bypassUrlPatterns:
//!         - example.com/bypass1
//!         - example.com/bypass2
//! ```
//! <!--End PulumiCodeChooser -->
//! 
//! ## Import
//! 
//! ```sh
//! $ pulumi import cloudflare:index/rateLimit:RateLimit example <zone_id>/<rate_limit_id>
//! ```
//! 

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct RateLimitArgs {
    /// The action to be performed when the threshold of matched traffic within the period defined is exceeded.
    #[builder(into)]
    pub action: pulumi_wasm_rust::Output<crate::types::RateLimitAction>,
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub bypass_url_patterns: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    /// Determines how rate limiting is applied. By default if not specified, rate limiting applies to the clients IP address.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub correlate: pulumi_wasm_rust::Output<Option<crate::types::RateLimitCorrelate>>,
    /// A note that you can use to describe the reason for a rate limit. This value is sanitized and all tags are removed.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    /// Whether this ratelimit is currently disabled. Defaults to `false`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub disabled: pulumi_wasm_rust::Output<Option<bool>>,
    /// Determines which traffic the rate limit counts towards the threshold. By default matches all traffic in the zone.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub match_: pulumi_wasm_rust::Output<Option<crate::types::RateLimitMatch>>,
    /// The time in seconds to count matching traffic. If the count exceeds threshold within this period the action will be performed.
    #[builder(into)]
    pub period: pulumi_wasm_rust::Output<i32>,
    /// The threshold that triggers the rate limit mitigations, combine with period.
    #[builder(into)]
    pub threshold: pulumi_wasm_rust::Output<i32>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct RateLimitResult {
    /// The action to be performed when the threshold of matched traffic within the period defined is exceeded.
    pub action: pulumi_wasm_rust::Output<crate::types::RateLimitAction>,
    pub bypass_url_patterns: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    /// Determines how rate limiting is applied. By default if not specified, rate limiting applies to the clients IP address.
    pub correlate: pulumi_wasm_rust::Output<Option<crate::types::RateLimitCorrelate>>,
    /// A note that you can use to describe the reason for a rate limit. This value is sanitized and all tags are removed.
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    /// Whether this ratelimit is currently disabled. Defaults to `false`.
    pub disabled: pulumi_wasm_rust::Output<Option<bool>>,
    /// Determines which traffic the rate limit counts towards the threshold. By default matches all traffic in the zone.
    pub match_: pulumi_wasm_rust::Output<crate::types::RateLimitMatch>,
    /// The time in seconds to count matching traffic. If the count exceeds threshold within this period the action will be performed.
    pub period: pulumi_wasm_rust::Output<i32>,
    /// The threshold that triggers the rate limit mitigations, combine with period.
    pub threshold: pulumi_wasm_rust::Output<i32>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: RateLimitArgs) -> RateLimitResult {

    let result = crate::bindings::pulumi::cloudflare::rate_limit::invoke(name, &crate::bindings::pulumi::cloudflare::rate_limit::Args {
        action: &args.action.get_inner(),
        bypass_url_patterns: &args.bypass_url_patterns.get_inner(),
        correlate: &args.correlate.get_inner(),
        description: &args.description.get_inner(),
        disabled: &args.disabled.get_inner(),
        match_: &args.match_.get_inner(),
        period: &args.period.get_inner(),
        threshold: &args.threshold.get_inner(),
        zone_id: &args.zone_id.get_inner(),
    });

    RateLimitResult {
        action: crate::into_domain(result.action),
        bypass_url_patterns: crate::into_domain(result.bypass_url_patterns),
        correlate: crate::into_domain(result.correlate),
        description: crate::into_domain(result.description),
        disabled: crate::into_domain(result.disabled),
        match_: crate::into_domain(result.match_),
        period: crate::into_domain(result.period),
        threshold: crate::into_domain(result.threshold),
        zone_id: crate::into_domain(result.zone_id),
    }
}
