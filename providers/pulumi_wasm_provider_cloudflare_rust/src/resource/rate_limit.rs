//! Provides a Cloudflare rate limit resource for a given zone. This can
//! be used to limit the traffic you receive zone-wide, or matching more
//! specific types of requests/responses.
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
//!     action: {
//!         mode: "simulate",
//!         response: {
//!             body: "custom response body",
//!             contentType: "text/plain",
//!         },
//!         timeout: 43200,
//!     },
//!     bypassUrlPatterns: [
//!         "example.com/bypass1",
//!         "example.com/bypass2",
//!     ],
//!     correlate: {
//!         by: "nat",
//!     },
//!     description: "example rate limit for a zone",
//!     disabled: false,
//!     match: {
//!         request: {
//!             methods: [
//!                 "GET",
//!                 "POST",
//!                 "PUT",
//!                 "DELETE",
//!                 "PATCH",
//!                 "HEAD",
//!             ],
//!             schemes: [
//!                 "HTTP",
//!                 "HTTPS",
//!             ],
//!             urlPattern: `${_var.cloudflare_zone}/*`,
//!         },
//!         response: {
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
//!             originTraffic: false,
//!             statuses: [
//!                 200,
//!                 201,
//!                 202,
//!                 301,
//!                 429,
//!             ],
//!         },
//!     },
//!     period: 2,
//!     threshold: 2000,
//!     zoneId: "0da42c8d2132a9ddaf714f9e7c920711",
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//! 
//! example = cloudflare.RateLimit("example",
//!     action=cloudflare.RateLimitActionArgs(
//!         mode="simulate",
//!         response=cloudflare.RateLimitActionResponseArgs(
//!             body="custom response body",
//!             content_type="text/plain",
//!         ),
//!         timeout=43200,
//!     ),
//!     bypass_url_patterns=[
//!         "example.com/bypass1",
//!         "example.com/bypass2",
//!     ],
//!     correlate=cloudflare.RateLimitCorrelateArgs(
//!         by="nat",
//!     ),
//!     description="example rate limit for a zone",
//!     disabled=False,
//!     match=cloudflare.RateLimitMatchArgs(
//!         request=cloudflare.RateLimitMatchRequestArgs(
//!             methods=[
//!                 "GET",
//!                 "POST",
//!                 "PUT",
//!                 "DELETE",
//!                 "PATCH",
//!                 "HEAD",
//!             ],
//!             schemes=[
//!                 "HTTP",
//!                 "HTTPS",
//!             ],
//!             url_pattern=f"{var['cloudflare_zone']}/*",
//!         ),
//!         response=cloudflare.RateLimitMatchResponseArgs(
//!             headers=[
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
//!             origin_traffic=False,
//!             statuses=[
//!                 200,
//!                 201,
//!                 202,
//!                 301,
//!                 429,
//!             ],
//!         ),
//!     ),
//!     period=2,
//!     threshold=2000,
//!     zone_id="0da42c8d2132a9ddaf714f9e7c920711")
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
//!         Action = new Cloudflare.Inputs.RateLimitActionArgs
//!         {
//!             Mode = "simulate",
//!             Response = new Cloudflare.Inputs.RateLimitActionResponseArgs
//!             {
//!                 Body = "custom response body",
//!                 ContentType = "text/plain",
//!             },
//!             Timeout = 43200,
//!         },
//!         BypassUrlPatterns = new[]
//!         {
//!             "example.com/bypass1",
//!             "example.com/bypass2",
//!         },
//!         Correlate = new Cloudflare.Inputs.RateLimitCorrelateArgs
//!         {
//!             By = "nat",
//!         },
//!         Description = "example rate limit for a zone",
//!         Disabled = false,
//!         Match = new Cloudflare.Inputs.RateLimitMatchArgs
//!         {
//!             Request = new Cloudflare.Inputs.RateLimitMatchRequestArgs
//!             {
//!                 Methods = new[]
//!                 {
//!                     "GET",
//!                     "POST",
//!                     "PUT",
//!                     "DELETE",
//!                     "PATCH",
//!                     "HEAD",
//!                 },
//!                 Schemes = new[]
//!                 {
//!                     "HTTP",
//!                     "HTTPS",
//!                 },
//!                 UrlPattern = $"{@var.Cloudflare_zone}/*",
//!             },
//!             Response = new Cloudflare.Inputs.RateLimitMatchResponseArgs
//!             {
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
//!                 OriginTraffic = false,
//!                 Statuses = new[]
//!                 {
//!                     200,
//!                     201,
//!                     202,
//!                     301,
//!                     429,
//!                 },
//!             },
//!         },
//!         Period = 2,
//!         Threshold = 2000,
//!         ZoneId = "0da42c8d2132a9ddaf714f9e7c920711",
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
//! 			Action: &cloudflare.RateLimitActionArgs{
//! 				Mode: pulumi.String("simulate"),
//! 				Response: &cloudflare.RateLimitActionResponseArgs{
//! 					Body:        pulumi.String("custom response body"),
//! 					ContentType: pulumi.String("text/plain"),
//! 				},
//! 				Timeout: pulumi.Int(43200),
//! 			},
//! 			BypassUrlPatterns: pulumi.StringArray{
//! 				pulumi.String("example.com/bypass1"),
//! 				pulumi.String("example.com/bypass2"),
//! 			},
//! 			Correlate: &cloudflare.RateLimitCorrelateArgs{
//! 				By: pulumi.String("nat"),
//! 			},
//! 			Description: pulumi.String("example rate limit for a zone"),
//! 			Disabled:    pulumi.Bool(false),
//! 			Match: &cloudflare.RateLimitMatchArgs{
//! 				Request: &cloudflare.RateLimitMatchRequestArgs{
//! 					Methods: pulumi.StringArray{
//! 						pulumi.String("GET"),
//! 						pulumi.String("POST"),
//! 						pulumi.String("PUT"),
//! 						pulumi.String("DELETE"),
//! 						pulumi.String("PATCH"),
//! 						pulumi.String("HEAD"),
//! 					},
//! 					Schemes: pulumi.StringArray{
//! 						pulumi.String("HTTP"),
//! 						pulumi.String("HTTPS"),
//! 					},
//! 					UrlPattern: pulumi.String(fmt.Sprintf("%v/*", _var.Cloudflare_zone)),
//! 				},
//! 				Response: &cloudflare.RateLimitMatchResponseArgs{
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
//! 					OriginTraffic: pulumi.Bool(false),
//! 					Statuses: pulumi.IntArray{
//! 						pulumi.Int(200),
//! 						pulumi.Int(201),
//! 						pulumi.Int(202),
//! 						pulumi.Int(301),
//! 						pulumi.Int(429),
//! 					},
//! 				},
//! 			},
//! 			Period:    pulumi.Int(2),
//! 			Threshold: pulumi.Int(2000),
//! 			ZoneId:    pulumi.String("0da42c8d2132a9ddaf714f9e7c920711"),
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
//! import com.pulumi.cloudflare.inputs.RateLimitActionArgs;
//! import com.pulumi.cloudflare.inputs.RateLimitActionResponseArgs;
//! import com.pulumi.cloudflare.inputs.RateLimitCorrelateArgs;
//! import com.pulumi.cloudflare.inputs.RateLimitMatchArgs;
//! import com.pulumi.cloudflare.inputs.RateLimitMatchRequestArgs;
//! import com.pulumi.cloudflare.inputs.RateLimitMatchResponseArgs;
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
//!             .action(RateLimitActionArgs.builder()
//!                 .mode("simulate")
//!                 .response(RateLimitActionResponseArgs.builder()
//!                     .body("custom response body")
//!                     .contentType("text/plain")
//!                     .build())
//!                 .timeout(43200)
//!                 .build())
//!             .bypassUrlPatterns(            
//!                 "example.com/bypass1",
//!                 "example.com/bypass2")
//!             .correlate(RateLimitCorrelateArgs.builder()
//!                 .by("nat")
//!                 .build())
//!             .description("example rate limit for a zone")
//!             .disabled(false)
//!             .match(RateLimitMatchArgs.builder()
//!                 .request(RateLimitMatchRequestArgs.builder()
//!                     .methods(                    
//!                         "GET",
//!                         "POST",
//!                         "PUT",
//!                         "DELETE",
//!                         "PATCH",
//!                         "HEAD")
//!                     .schemes(                    
//!                         "HTTP",
//!                         "HTTPS")
//!                     .urlPattern(String.format("%s/*", var_.cloudflare_zone()))
//!                     .build())
//!                 .response(RateLimitMatchResponseArgs.builder()
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
//!                     .originTraffic(false)
//!                     .statuses(                    
//!                         200,
//!                         201,
//!                         202,
//!                         301,
//!                         429)
//!                     .build())
//!                 .build())
//!             .period(2)
//!             .threshold(2000)
//!             .zoneId("0da42c8d2132a9ddaf714f9e7c920711")
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
//!       action:
//!         mode: simulate
//!         response:
//!           body: custom response body
//!           contentType: text/plain
//!         timeout: 43200
//!       bypassUrlPatterns:
//!         - example.com/bypass1
//!         - example.com/bypass2
//!       correlate:
//!         by: nat
//!       description: example rate limit for a zone
//!       disabled: false
//!       match:
//!         request:
//!           methods:
//!             - GET
//!             - POST
//!             - PUT
//!             - DELETE
//!             - PATCH
//!             - HEAD
//!           schemes:
//!             - HTTP
//!             - HTTPS
//!           urlPattern: ${var.cloudflare_zone}/*
//!         response:
//!           headers:
//!             - name: Host
//!               op: eq
//!               value: localhost
//!             - name: X-Example
//!               op: ne
//!               value: my-example
//!           originTraffic: false
//!           statuses:
//!             - 200
//!             - 201
//!             - 202
//!             - 301
//!             - 429
//!       period: 2
//!       threshold: 2000
//!       zoneId: 0da42c8d2132a9ddaf714f9e7c920711
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
