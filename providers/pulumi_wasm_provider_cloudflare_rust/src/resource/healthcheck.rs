//! Standalone Health Checks provide a way to monitor origin servers
//! without needing a Cloudflare Load Balancer.
//!
//! ## Example Usage
//!
//! <!--Start PulumiCodeChooser -->
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as cloudflare from "@pulumi/cloudflare";
//!
//! // HTTPS Healthcheck
//! const httpHealthCheck = new cloudflare.Healthcheck("httpHealthCheck", {
//!     zoneId: _var.cloudflare_zone_id,
//!     name: "http-health-check",
//!     description: "example http health check",
//!     address: "example.com",
//!     suspended: false,
//!     checkRegions: [
//!         "WEU",
//!         "EEU",
//!     ],
//!     type: "HTTPS",
//!     port: 443,
//!     method: "GET",
//!     path: "/health",
//!     expectedBody: "alive",
//!     expectedCodes: [
//!         "2xx",
//!         "301",
//!     ],
//!     followRedirects: true,
//!     allowInsecure: false,
//!     headers: [{
//!         header: "Host",
//!         values: ["example.com"],
//!     }],
//!     timeout: 10,
//!     retries: 2,
//!     interval: 60,
//!     consecutiveFails: 3,
//!     consecutiveSuccesses: 2,
//! });
//! // TCP Healthcheck
//! const tcpHealthCheck = new cloudflare.Healthcheck("tcpHealthCheck", {
//!     zoneId: _var.cloudflare_zone_id,
//!     name: "tcp-health-check",
//!     description: "example tcp health check",
//!     address: "example.com",
//!     suspended: false,
//!     checkRegions: [
//!         "WEU",
//!         "EEU",
//!     ],
//!     type: "TCP",
//!     port: 22,
//!     method: "connection_established",
//!     timeout: 10,
//!     retries: 2,
//!     interval: 60,
//!     consecutiveFails: 3,
//!     consecutiveSuccesses: 2,
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//!
//! # HTTPS Healthcheck
//! http_health_check = cloudflare.Healthcheck("httpHealthCheck",
//!     zone_id=var["cloudflare_zone_id"],
//!     name="http-health-check",
//!     description="example http health check",
//!     address="example.com",
//!     suspended=False,
//!     check_regions=[
//!         "WEU",
//!         "EEU",
//!     ],
//!     type="HTTPS",
//!     port=443,
//!     method="GET",
//!     path="/health",
//!     expected_body="alive",
//!     expected_codes=[
//!         "2xx",
//!         "301",
//!     ],
//!     follow_redirects=True,
//!     allow_insecure=False,
//!     headers=[cloudflare.HealthcheckHeaderArgs(
//!         header="Host",
//!         values=["example.com"],
//!     )],
//!     timeout=10,
//!     retries=2,
//!     interval=60,
//!     consecutive_fails=3,
//!     consecutive_successes=2)
//! # TCP Healthcheck
//! tcp_health_check = cloudflare.Healthcheck("tcpHealthCheck",
//!     zone_id=var["cloudflare_zone_id"],
//!     name="tcp-health-check",
//!     description="example tcp health check",
//!     address="example.com",
//!     suspended=False,
//!     check_regions=[
//!         "WEU",
//!         "EEU",
//!     ],
//!     type="TCP",
//!     port=22,
//!     method="connection_established",
//!     timeout=10,
//!     retries=2,
//!     interval=60,
//!     consecutive_fails=3,
//!     consecutive_successes=2)
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
//!     // HTTPS Healthcheck
//!     var httpHealthCheck = new Cloudflare.Healthcheck("httpHealthCheck", new()
//!     {
//!         ZoneId = @var.Cloudflare_zone_id,
//!         Name = "http-health-check",
//!         Description = "example http health check",
//!         Address = "example.com",
//!         Suspended = false,
//!         CheckRegions = new[]
//!         {
//!             "WEU",
//!             "EEU",
//!         },
//!         Type = "HTTPS",
//!         Port = 443,
//!         Method = "GET",
//!         Path = "/health",
//!         ExpectedBody = "alive",
//!         ExpectedCodes = new[]
//!         {
//!             "2xx",
//!             "301",
//!         },
//!         FollowRedirects = true,
//!         AllowInsecure = false,
//!         Headers = new[]
//!         {
//!             new Cloudflare.Inputs.HealthcheckHeaderArgs
//!             {
//!                 Header = "Host",
//!                 Values = new[]
//!                 {
//!                     "example.com",
//!                 },
//!             },
//!         },
//!         Timeout = 10,
//!         Retries = 2,
//!         Interval = 60,
//!         ConsecutiveFails = 3,
//!         ConsecutiveSuccesses = 2,
//!     });
//!
//!     // TCP Healthcheck
//!     var tcpHealthCheck = new Cloudflare.Healthcheck("tcpHealthCheck", new()
//!     {
//!         ZoneId = @var.Cloudflare_zone_id,
//!         Name = "tcp-health-check",
//!         Description = "example tcp health check",
//!         Address = "example.com",
//!         Suspended = false,
//!         CheckRegions = new[]
//!         {
//!             "WEU",
//!             "EEU",
//!         },
//!         Type = "TCP",
//!         Port = 22,
//!         Method = "connection_established",
//!         Timeout = 10,
//!         Retries = 2,
//!         Interval = 60,
//!         ConsecutiveFails = 3,
//!         ConsecutiveSuccesses = 2,
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
//! 		// HTTPS Healthcheck
//! 		_, err := cloudflare.NewHealthcheck(ctx, "httpHealthCheck", &cloudflare.HealthcheckArgs{
//! 			ZoneId:      pulumi.Any(_var.Cloudflare_zone_id),
//! 			Name:        pulumi.String("http-health-check"),
//! 			Description: pulumi.String("example http health check"),
//! 			Address:     pulumi.String("example.com"),
//! 			Suspended:   pulumi.Bool(false),
//! 			CheckRegions: pulumi.StringArray{
//! 				pulumi.String("WEU"),
//! 				pulumi.String("EEU"),
//! 			},
//! 			Type:         pulumi.String("HTTPS"),
//! 			Port:         pulumi.Int(443),
//! 			Method:       pulumi.String("GET"),
//! 			Path:         pulumi.String("/health"),
//! 			ExpectedBody: pulumi.String("alive"),
//! 			ExpectedCodes: pulumi.StringArray{
//! 				pulumi.String("2xx"),
//! 				pulumi.String("301"),
//! 			},
//! 			FollowRedirects: pulumi.Bool(true),
//! 			AllowInsecure:   pulumi.Bool(false),
//! 			Headers: cloudflare.HealthcheckHeaderArray{
//! 				&cloudflare.HealthcheckHeaderArgs{
//! 					Header: pulumi.String("Host"),
//! 					Values: pulumi.StringArray{
//! 						pulumi.String("example.com"),
//! 					},
//! 				},
//! 			},
//! 			Timeout:              pulumi.Int(10),
//! 			Retries:              pulumi.Int(2),
//! 			Interval:             pulumi.Int(60),
//! 			ConsecutiveFails:     pulumi.Int(3),
//! 			ConsecutiveSuccesses: pulumi.Int(2),
//! 		})
//! 		if err != nil {
//! 			return err
//! 		}
//! 		// TCP Healthcheck
//! 		_, err = cloudflare.NewHealthcheck(ctx, "tcpHealthCheck", &cloudflare.HealthcheckArgs{
//! 			ZoneId:      pulumi.Any(_var.Cloudflare_zone_id),
//! 			Name:        pulumi.String("tcp-health-check"),
//! 			Description: pulumi.String("example tcp health check"),
//! 			Address:     pulumi.String("example.com"),
//! 			Suspended:   pulumi.Bool(false),
//! 			CheckRegions: pulumi.StringArray{
//! 				pulumi.String("WEU"),
//! 				pulumi.String("EEU"),
//! 			},
//! 			Type:                 pulumi.String("TCP"),
//! 			Port:                 pulumi.Int(22),
//! 			Method:               pulumi.String("connection_established"),
//! 			Timeout:              pulumi.Int(10),
//! 			Retries:              pulumi.Int(2),
//! 			Interval:             pulumi.Int(60),
//! 			ConsecutiveFails:     pulumi.Int(3),
//! 			ConsecutiveSuccesses: pulumi.Int(2),
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
//! import com.pulumi.cloudflare.Healthcheck;
//! import com.pulumi.cloudflare.HealthcheckArgs;
//! import com.pulumi.cloudflare.inputs.HealthcheckHeaderArgs;
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
//!         // HTTPS Healthcheck
//!         var httpHealthCheck = new Healthcheck("httpHealthCheck", HealthcheckArgs.builder()        
//!             .zoneId(var_.cloudflare_zone_id())
//!             .name("http-health-check")
//!             .description("example http health check")
//!             .address("example.com")
//!             .suspended(false)
//!             .checkRegions(            
//!                 "WEU",
//!                 "EEU")
//!             .type("HTTPS")
//!             .port(443)
//!             .method("GET")
//!             .path("/health")
//!             .expectedBody("alive")
//!             .expectedCodes(            
//!                 "2xx",
//!                 "301")
//!             .followRedirects(true)
//!             .allowInsecure(false)
//!             .headers(HealthcheckHeaderArgs.builder()
//!                 .header("Host")
//!                 .values("example.com")
//!                 .build())
//!             .timeout(10)
//!             .retries(2)
//!             .interval(60)
//!             .consecutiveFails(3)
//!             .consecutiveSuccesses(2)
//!             .build());
//!
//!         // TCP Healthcheck
//!         var tcpHealthCheck = new Healthcheck("tcpHealthCheck", HealthcheckArgs.builder()        
//!             .zoneId(var_.cloudflare_zone_id())
//!             .name("tcp-health-check")
//!             .description("example tcp health check")
//!             .address("example.com")
//!             .suspended(false)
//!             .checkRegions(            
//!                 "WEU",
//!                 "EEU")
//!             .type("TCP")
//!             .port(22)
//!             .method("connection_established")
//!             .timeout(10)
//!             .retries(2)
//!             .interval(60)
//!             .consecutiveFails(3)
//!             .consecutiveSuccesses(2)
//!             .build());
//!
//!     }
//! }
//! ```
//! ### YAML
//! ```yaml
//! resources:
//!   # HTTPS Healthcheck
//!   httpHealthCheck:
//!     type: cloudflare:Healthcheck
//!     properties:
//!       zoneId: ${var.cloudflare_zone_id}
//!       name: http-health-check
//!       description: example http health check
//!       address: example.com
//!       suspended: false
//!       checkRegions:
//!         - WEU
//!         - EEU
//!       type: HTTPS
//!       port: 443
//!       method: GET
//!       path: /health
//!       expectedBody: alive
//!       expectedCodes:
//!         - 2xx
//!         - '301'
//!       followRedirects: true
//!       allowInsecure: false
//!       headers:
//!         - header: Host
//!           values:
//!             - example.com
//!       timeout: 10
//!       retries: 2
//!       interval: 60
//!       consecutiveFails: 3
//!       consecutiveSuccesses: 2
//!   # TCP Healthcheck
//!   tcpHealthCheck:
//!     type: cloudflare:Healthcheck
//!     properties:
//!       zoneId: ${var.cloudflare_zone_id}
//!       name: tcp-health-check
//!       description: example tcp health check
//!       address: example.com
//!       suspended: false
//!       checkRegions:
//!         - WEU
//!         - EEU
//!       type: TCP
//!       port: 22
//!       method: connection_established
//!       timeout: 10
//!       retries: 2
//!       interval: 60
//!       consecutiveFails: 3
//!       consecutiveSuccesses: 2
//! ```
//! <!--End PulumiCodeChooser -->
//!
//! ## Import
//!
//! Use the Zone ID and Healthcheck ID to import.
//!
//! ```sh
//! $ pulumi import cloudflare:index/healthcheck:Healthcheck example <zone_id>/<healthcheck_id>
//! ```
//!

pub struct HealthcheckArgs {
    /// The hostname or IP address of the origin server to run health checks on.
    pub address: pulumi_wasm_rust::Output<String>,
    /// Do not validate the certificate when the health check uses HTTPS. Defaults to `false`.
    pub allow_insecure: pulumi_wasm_rust::Output<Option<bool>>,
    /// A list of regions from which to run health checks. If not set, Cloudflare will pick a default region. Available values: `WNAM`, `ENAM`, `WEU`, `EEU`, `NSAM`, `SSAM`, `OC`, `ME`, `NAF`, `SAF`, `IN`, `SEAS`, `NEAS`, `ALL_REGIONS`.
    pub check_regions: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    /// The number of consecutive fails required from a health check before changing the health to unhealthy. Defaults to `1`.
    pub consecutive_fails: pulumi_wasm_rust::Output<Option<i32>>,
    /// The number of consecutive successes required from a health check before changing the health to healthy. Defaults to `1`.
    pub consecutive_successes: pulumi_wasm_rust::Output<Option<i32>>,
    /// A human-readable description of the health check.
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    /// A case-insensitive sub-string to look for in the response body. If this string is not found the origin will be marked as unhealthy.
    pub expected_body: pulumi_wasm_rust::Output<Option<String>>,
    /// The expected HTTP response codes (e.g. '200') or code ranges (e.g. '2xx' for all codes starting with 2) of the health check.
    pub expected_codes: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    /// Follow redirects if the origin returns a 3xx status code. Defaults to `false`.
    pub follow_redirects: pulumi_wasm_rust::Output<Option<bool>>,
    /// The header name.
    pub headers: pulumi_wasm_rust::Output<Option<Vec<crate::types::HealthcheckHeader>>>,
    /// The interval between each health check. Shorter intervals may give quicker notifications if the origin status changes, but will increase the load on the origin as we check from multiple locations. Defaults to `60`.
    pub interval: pulumi_wasm_rust::Output<Option<i32>>,
    /// The HTTP method to use for the health check. Available values: `connection_established`, `GET`, `HEAD`.
    pub method: pulumi_wasm_rust::Output<Option<String>>,
    /// A short name to identify the health check. Only alphanumeric characters, hyphens, and underscores are allowed.
    pub name: pulumi_wasm_rust::Output<String>,
    /// The endpoint path to health check against. Defaults to `/`.
    pub path: pulumi_wasm_rust::Output<Option<String>>,
    /// Port number to connect to for the health check. Defaults to `80`.
    pub port: pulumi_wasm_rust::Output<Option<i32>>,
    /// The number of retries to attempt in case of a timeout before marking the origin as unhealthy. Retries are attempted immediately. Defaults to `2`.
    pub retries: pulumi_wasm_rust::Output<Option<i32>>,
    /// If suspended, no health checks are sent to the origin. Defaults to `false`.
    pub suspended: pulumi_wasm_rust::Output<Option<bool>>,
    /// The timeout (in seconds) before marking the health check as failed. Defaults to `5`.
    pub timeout: pulumi_wasm_rust::Output<Option<i32>>,
    /// The protocol to use for the health check. Available values: `TCP`, `HTTP`, `HTTPS`.
    pub type_: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct HealthcheckResult {
    /// The hostname or IP address of the origin server to run health checks on.
    pub address: pulumi_wasm_rust::Output<String>,
    /// Do not validate the certificate when the health check uses HTTPS. Defaults to `false`.
    pub allow_insecure: pulumi_wasm_rust::Output<Option<bool>>,
    /// A list of regions from which to run health checks. If not set, Cloudflare will pick a default region. Available values: `WNAM`, `ENAM`, `WEU`, `EEU`, `NSAM`, `SSAM`, `OC`, `ME`, `NAF`, `SAF`, `IN`, `SEAS`, `NEAS`, `ALL_REGIONS`.
    pub check_regions: pulumi_wasm_rust::Output<Vec<String>>,
    /// The number of consecutive fails required from a health check before changing the health to unhealthy. Defaults to `1`.
    pub consecutive_fails: pulumi_wasm_rust::Output<Option<i32>>,
    /// The number of consecutive successes required from a health check before changing the health to healthy. Defaults to `1`.
    pub consecutive_successes: pulumi_wasm_rust::Output<Option<i32>>,
    /// Creation time.
    pub created_on: pulumi_wasm_rust::Output<String>,
    /// A human-readable description of the health check.
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    /// A case-insensitive sub-string to look for in the response body. If this string is not found the origin will be marked as unhealthy.
    pub expected_body: pulumi_wasm_rust::Output<Option<String>>,
    /// The expected HTTP response codes (e.g. '200') or code ranges (e.g. '2xx' for all codes starting with 2) of the health check.
    pub expected_codes: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    /// Follow redirects if the origin returns a 3xx status code. Defaults to `false`.
    pub follow_redirects: pulumi_wasm_rust::Output<Option<bool>>,
    /// The header name.
    pub headers: pulumi_wasm_rust::Output<Option<Vec<crate::types::HealthcheckHeader>>>,
    /// The interval between each health check. Shorter intervals may give quicker notifications if the origin status changes, but will increase the load on the origin as we check from multiple locations. Defaults to `60`.
    pub interval: pulumi_wasm_rust::Output<Option<i32>>,
    /// The HTTP method to use for the health check. Available values: `connection_established`, `GET`, `HEAD`.
    pub method: pulumi_wasm_rust::Output<String>,
    /// Last modified time.
    pub modified_on: pulumi_wasm_rust::Output<String>,
    /// A short name to identify the health check. Only alphanumeric characters, hyphens, and underscores are allowed.
    pub name: pulumi_wasm_rust::Output<String>,
    /// The endpoint path to health check against. Defaults to `/`.
    pub path: pulumi_wasm_rust::Output<Option<String>>,
    /// Port number to connect to for the health check. Defaults to `80`.
    pub port: pulumi_wasm_rust::Output<Option<i32>>,
    /// The number of retries to attempt in case of a timeout before marking the origin as unhealthy. Retries are attempted immediately. Defaults to `2`.
    pub retries: pulumi_wasm_rust::Output<Option<i32>>,
    /// If suspended, no health checks are sent to the origin. Defaults to `false`.
    pub suspended: pulumi_wasm_rust::Output<Option<bool>>,
    /// The timeout (in seconds) before marking the health check as failed. Defaults to `5`.
    pub timeout: pulumi_wasm_rust::Output<Option<i32>>,
    /// The protocol to use for the health check. Available values: `TCP`, `HTTP`, `HTTPS`.
    pub type_: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: HealthcheckArgs) -> HealthcheckResult {
    let result = crate::bindings::pulumi::cloudflare::healthcheck::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::healthcheck::Args {
            address: &args.address.get_inner(),
            allow_insecure: &args.allow_insecure.get_inner(),
            check_regions: &args.check_regions.get_inner(),
            consecutive_fails: &args.consecutive_fails.get_inner(),
            consecutive_successes: &args.consecutive_successes.get_inner(),
            description: &args.description.get_inner(),
            expected_body: &args.expected_body.get_inner(),
            expected_codes: &args.expected_codes.get_inner(),
            follow_redirects: &args.follow_redirects.get_inner(),
            headers: &args.headers.get_inner(),
            interval: &args.interval.get_inner(),
            method: &args.method.get_inner(),
            name: &args.name.get_inner(),
            path: &args.path.get_inner(),
            port: &args.port.get_inner(),
            retries: &args.retries.get_inner(),
            suspended: &args.suspended.get_inner(),
            timeout: &args.timeout.get_inner(),
            type_: &args.type_.get_inner(),
            zone_id: &args.zone_id.get_inner(),
        },
    );

    HealthcheckResult {
        address: crate::into_domain(result.address),
        allow_insecure: crate::into_domain(result.allow_insecure),
        check_regions: crate::into_domain(result.check_regions),
        consecutive_fails: crate::into_domain(result.consecutive_fails),
        consecutive_successes: crate::into_domain(result.consecutive_successes),
        created_on: crate::into_domain(result.created_on),
        description: crate::into_domain(result.description),
        expected_body: crate::into_domain(result.expected_body),
        expected_codes: crate::into_domain(result.expected_codes),
        follow_redirects: crate::into_domain(result.follow_redirects),
        headers: crate::into_domain(result.headers),
        interval: crate::into_domain(result.interval),
        method: crate::into_domain(result.method),
        modified_on: crate::into_domain(result.modified_on),
        name: crate::into_domain(result.name),
        path: crate::into_domain(result.path),
        port: crate::into_domain(result.port),
        retries: crate::into_domain(result.retries),
        suspended: crate::into_domain(result.suspended),
        timeout: crate::into_domain(result.timeout),
        type_: crate::into_domain(result.type_),
        zone_id: crate::into_domain(result.zone_id),
    }
}
