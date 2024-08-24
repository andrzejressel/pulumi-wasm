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
//! const example = new cloudflare.DeviceDexTest("example", {
//!     accountId: "f037e56e89293a057740de681ac9abbe",
//!     data: {
//!         host: "https://example.com/home",
//!         kind: "http",
//!         method: "GET",
//!     },
//!     description: "Send a HTTP GET request to the home endpoint every half hour.",
//!     enabled: true,
//!     interval: "0h30m0s",
//!     name: "GET homepage",
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//!
//! example = cloudflare.DeviceDexTest("example",
//!     account_id="f037e56e89293a057740de681ac9abbe",
//!     data=cloudflare.DeviceDexTestDataArgs(
//!         host="https://example.com/home",
//!         kind="http",
//!         method="GET",
//!     ),
//!     description="Send a HTTP GET request to the home endpoint every half hour.",
//!     enabled=True,
//!     interval="0h30m0s",
//!     name="GET homepage")
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
//!     var example = new Cloudflare.DeviceDexTest("example", new()
//!     {
//!         AccountId = "f037e56e89293a057740de681ac9abbe",
//!         Data = new Cloudflare.Inputs.DeviceDexTestDataArgs
//!         {
//!             Host = "https://example.com/home",
//!             Kind = "http",
//!             Method = "GET",
//!         },
//!         Description = "Send a HTTP GET request to the home endpoint every half hour.",
//!         Enabled = true,
//!         Interval = "0h30m0s",
//!         Name = "GET homepage",
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
//! 		_, err := cloudflare.NewDeviceDexTest(ctx, "example", &cloudflare.DeviceDexTestArgs{
//! 			AccountId: pulumi.String("f037e56e89293a057740de681ac9abbe"),
//! 			Data: &cloudflare.DeviceDexTestDataArgs{
//! 				Host:   pulumi.String("https://example.com/home"),
//! 				Kind:   pulumi.String("http"),
//! 				Method: pulumi.String("GET"),
//! 			},
//! 			Description: pulumi.String("Send a HTTP GET request to the home endpoint every half hour."),
//! 			Enabled:     pulumi.Bool(true),
//! 			Interval:    pulumi.String("0h30m0s"),
//! 			Name:        pulumi.String("GET homepage"),
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
//! import com.pulumi.cloudflare.DeviceDexTest;
//! import com.pulumi.cloudflare.DeviceDexTestArgs;
//! import com.pulumi.cloudflare.inputs.DeviceDexTestDataArgs;
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
//!         var example = new DeviceDexTest("example", DeviceDexTestArgs.builder()        
//!             .accountId("f037e56e89293a057740de681ac9abbe")
//!             .data(DeviceDexTestDataArgs.builder()
//!                 .host("https://example.com/home")
//!                 .kind("http")
//!                 .method("GET")
//!                 .build())
//!             .description("Send a HTTP GET request to the home endpoint every half hour.")
//!             .enabled(true)
//!             .interval("0h30m0s")
//!             .name("GET homepage")
//!             .build());
//!
//!     }
//! }
//! ```
//! ### YAML
//! ```yaml
//! resources:
//!   example:
//!     type: cloudflare:DeviceDexTest
//!     properties:
//!       accountId: f037e56e89293a057740de681ac9abbe
//!       data:
//!         host: https://example.com/home
//!         kind: http
//!         method: GET
//!       description: Send a HTTP GET request to the home endpoint every half hour.
//!       enabled: true
//!       interval: 0h30m0s
//!       name: GET homepage
//! ```
//! <!--End PulumiCodeChooser -->
//!
//! ## Import
//!
//! ```sh
//! $ pulumi import cloudflare:index/deviceDexTest:DeviceDexTest example <account_id>/<device_dex_test_id>
//! ```
//!

pub struct DeviceDexTestArgs {
    /// The account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The configuration object which contains the details for the WARP client to conduct the test.
    pub data: pulumi_wasm_rust::Output<crate::types::DeviceDexTestData>,
    /// Additional details about the test.
    pub description: pulumi_wasm_rust::Output<String>,
    /// Determines whether or not the test is active.
    pub enabled: pulumi_wasm_rust::Output<bool>,
    /// How often the test will run.
    pub interval: pulumi_wasm_rust::Output<String>,
    /// The name of the Device Dex Test. Must be unique.
    pub name: pulumi_wasm_rust::Output<String>,
}

pub struct DeviceDexTestResult {
    /// The account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// Timestamp of when the Dex Test was created.
    pub created: pulumi_wasm_rust::Output<String>,
    /// The configuration object which contains the details for the WARP client to conduct the test.
    pub data: pulumi_wasm_rust::Output<crate::types::DeviceDexTestData>,
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
pub fn create(name: &str, args: DeviceDexTestArgs) -> DeviceDexTestResult {
    let result = crate::bindings::pulumi::cloudflare::device_dex_test::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::device_dex_test::Args {
            account_id: args.account_id.get_inner(),
            data: args.data.get_inner(),
            description: args.description.get_inner(),
            enabled: args.enabled.get_inner(),
            interval: args.interval.get_inner(),
            name: args.name.get_inner(),
        },
    );

    DeviceDexTestResult {
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
