//! Provides a Cloudflare Device Dex Test resource. Device Dex Tests allow for building location-aware device settings policies.
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as cloudflare from "@pulumi/cloudflare";
//! 
//! const example = new cloudflare.ZeroTrustDexTest("example", {
//!     accountId: "f037e56e89293a057740de681ac9abbe",
//!     name: "GET homepage",
//!     description: "Send a HTTP GET request to the home endpoint every half hour.",
//!     interval: "0h30m0s",
//!     enabled: true,
//!     data: {
//!         host: "https://example.com/home",
//!         kind: "http",
//!         method: "GET",
//!     },
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//! 
//! example = cloudflare.ZeroTrustDexTest("example",
//!     account_id="f037e56e89293a057740de681ac9abbe",
//!     name="GET homepage",
//!     description="Send a HTTP GET request to the home endpoint every half hour.",
//!     interval="0h30m0s",
//!     enabled=True,
//!     data={
//!         "host": "https://example.com/home",
//!         "kind": "http",
//!         "method": "GET",
//!     })
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
//!     var example = new Cloudflare.ZeroTrustDexTest("example", new()
//!     {
//!         AccountId = "f037e56e89293a057740de681ac9abbe",
//!         Name = "GET homepage",
//!         Description = "Send a HTTP GET request to the home endpoint every half hour.",
//!         Interval = "0h30m0s",
//!         Enabled = true,
//!         Data = new Cloudflare.Inputs.ZeroTrustDexTestDataArgs
//!         {
//!             Host = "https://example.com/home",
//!             Kind = "http",
//!             Method = "GET",
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
//! 		_, err := cloudflare.NewZeroTrustDexTest(ctx, "example", &cloudflare.ZeroTrustDexTestArgs{
//! 			AccountId:   pulumi.String("f037e56e89293a057740de681ac9abbe"),
//! 			Name:        pulumi.String("GET homepage"),
//! 			Description: pulumi.String("Send a HTTP GET request to the home endpoint every half hour."),
//! 			Interval:    pulumi.String("0h30m0s"),
//! 			Enabled:     pulumi.Bool(true),
//! 			Data: &cloudflare.ZeroTrustDexTestDataArgs{
//! 				Host:   pulumi.String("https://example.com/home"),
//! 				Kind:   pulumi.String("http"),
//! 				Method: pulumi.String("GET"),
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
//! import com.pulumi.cloudflare.ZeroTrustDexTest;
//! import com.pulumi.cloudflare.ZeroTrustDexTestArgs;
//! import com.pulumi.cloudflare.inputs.ZeroTrustDexTestDataArgs;
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
//!         var example = new ZeroTrustDexTest("example", ZeroTrustDexTestArgs.builder()
//!             .accountId("f037e56e89293a057740de681ac9abbe")
//!             .name("GET homepage")
//!             .description("Send a HTTP GET request to the home endpoint every half hour.")
//!             .interval("0h30m0s")
//!             .enabled(true)
//!             .data(ZeroTrustDexTestDataArgs.builder()
//!                 .host("https://example.com/home")
//!                 .kind("http")
//!                 .method("GET")
//!                 .build())
//!             .build());
//! 
//!     }
//! }
//! ```
//! ### YAML
//! ```yaml
//! resources:
//!   example:
//!     type: cloudflare:ZeroTrustDexTest
//!     properties:
//!       accountId: f037e56e89293a057740de681ac9abbe
//!       name: GET homepage
//!       description: Send a HTTP GET request to the home endpoint every half hour.
//!       interval: 0h30m0s
//!       enabled: true
//!       data:
//!         host: https://example.com/home
//!         kind: http
//!         method: GET
//! ```
//! <!--End PulumiCodeChooser -->
//! 
//! ## Import
//! 
//! ```sh
//! $ pulumi import cloudflare:index/zeroTrustDexTest:ZeroTrustDexTest example <account_id>/<device_dex_test_id>
//! ```
//! 

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct ZeroTrustDexTestArgs {
    /// The account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The configuration object which contains the details for the WARP client to conduct the test.
    #[builder(into)]
    pub data: pulumi_wasm_rust::Output<crate::types::ZeroTrustDexTestData>,
    /// Additional details about the test.
    #[builder(into)]
    pub description: pulumi_wasm_rust::Output<String>,
    /// Determines whether or not the test is active.
    #[builder(into)]
    pub enabled: pulumi_wasm_rust::Output<bool>,
    /// How often the test will run.
    #[builder(into)]
    pub interval: pulumi_wasm_rust::Output<String>,
    /// The name of the Device Dex Test. Must be unique.
    #[builder(into)]
    pub name: pulumi_wasm_rust::Output<String>,
}

pub struct ZeroTrustDexTestResult {
    /// The account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// Timestamp of when the Dex Test was created.
    pub created: pulumi_wasm_rust::Output<String>,
    /// The configuration object which contains the details for the WARP client to conduct the test.
    pub data: pulumi_wasm_rust::Output<crate::types::ZeroTrustDexTestData>,
    /// Additional details about the test.
    pub description: pulumi_wasm_rust::Output<String>,
    /// Determines whether or not the test is active.
    pub enabled: pulumi_wasm_rust::Output<bool>,
    /// How often the test will run.
    pub interval: pulumi_wasm_rust::Output<String>,
    /// The name of the Device Dex Test. Must be unique.
    pub name: pulumi_wasm_rust::Output<String>,
    /// Timestamp of when the Dex Test was last updated.
    pub updated: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: ZeroTrustDexTestArgs) -> ZeroTrustDexTestResult {

    let result = crate::bindings::pulumi::cloudflare::zero_trust_dex_test::invoke(name, &crate::bindings::pulumi::cloudflare::zero_trust_dex_test::Args {
        account_id: &args.account_id.get_inner(),
        data: &args.data.get_inner(),
        description: &args.description.get_inner(),
        enabled: &args.enabled.get_inner(),
        interval: &args.interval.get_inner(),
        name: &args.name.get_inner(),
    });

    ZeroTrustDexTestResult {
        account_id: crate::into_domain(result.account_id),
        created: crate::into_domain(result.created),
        data: crate::into_domain(result.data),
        description: crate::into_domain(result.description),
        enabled: crate::into_domain(result.enabled),
        interval: crate::into_domain(result.interval),
        name: crate::into_domain(result.name),
        updated: crate::into_domain(result.updated),
    }
}
