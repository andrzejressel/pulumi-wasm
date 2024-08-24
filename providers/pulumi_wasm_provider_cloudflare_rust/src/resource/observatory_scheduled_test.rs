//! Provides a Cloudflare Observatory Scheduled Test resource.
//!
//! ## Example Usage
//!
//! <!--Start PulumiCodeChooser -->
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as cloudflare from "@pulumi/cloudflare";
//!
//! const example = new cloudflare.ObservatoryScheduledTest("example", {
//!     frequency: "WEEKLY",
//!     region: "us-central1",
//!     url: "example.com",
//!     zoneId: "0da42c8d2132a9ddaf714f9e7c920711",
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//!
//! example = cloudflare.ObservatoryScheduledTest("example",
//!     frequency="WEEKLY",
//!     region="us-central1",
//!     url="example.com",
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
//!     var example = new Cloudflare.ObservatoryScheduledTest("example", new()
//!     {
//!         Frequency = "WEEKLY",
//!         Region = "us-central1",
//!         Url = "example.com",
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
//! 	"github.com/pulumi/pulumi-cloudflare/sdk/v5/go/cloudflare"
//! 	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
//! )
//!
//! func main() {
//! 	pulumi.Run(func(ctx *pulumi.Context) error {
//! 		_, err := cloudflare.NewObservatoryScheduledTest(ctx, "example", &cloudflare.ObservatoryScheduledTestArgs{
//! 			Frequency: pulumi.String("WEEKLY"),
//! 			Region:    pulumi.String("us-central1"),
//! 			Url:       pulumi.String("example.com"),
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
//! import com.pulumi.cloudflare.ObservatoryScheduledTest;
//! import com.pulumi.cloudflare.ObservatoryScheduledTestArgs;
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
//!         var example = new ObservatoryScheduledTest("example", ObservatoryScheduledTestArgs.builder()        
//!             .frequency("WEEKLY")
//!             .region("us-central1")
//!             .url("example.com")
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
//!     type: cloudflare:ObservatoryScheduledTest
//!     properties:
//!       frequency: WEEKLY
//!       region: us-central1
//!       url: example.com
//!       zoneId: 0da42c8d2132a9ddaf714f9e7c920711
//! ```
//! <!--End PulumiCodeChooser -->
//!
//! ## Import
//!
//! ```sh
//! $ pulumi import cloudflare:index/observatoryScheduledTest:ObservatoryScheduledTest example <zone_id>:<url>:<region>
//! ```
//!

pub struct ObservatoryScheduledTestArgs {
    /// The frequency to run the test. Available values: `DAILY`, `WEEKLY`. **Modifying this attribute will force creation of a new resource.**
    pub frequency: pulumi_wasm_rust::Output<String>,
    /// The region to run the test in. Available values: `us-central1`, `us-east1`, `us-east4`, `us-south1`, `us-west1`, `southamerica-east1`, `europe-north1`, `europe-southwest1`, `europe-west1`, `europe-west2`, `europe-west3`, `europe-west4`, `europe-west8`, `europe-west9`, `asia-east1`, `asia-south1`, `asia-southeast1`, `me-west1`, `australia-southeast1`. **Modifying this attribute will force creation of a new resource.**
    pub region: pulumi_wasm_rust::Output<String>,
    /// The page to run the test on. **Modifying this attribute will force creation of a new resource.**
    pub url: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct ObservatoryScheduledTestResult {
    /// The frequency to run the test. Available values: `DAILY`, `WEEKLY`. **Modifying this attribute will force creation of a new resource.**
    pub frequency: pulumi_wasm_rust::Output<String>,
    /// The region to run the test in. Available values: `us-central1`, `us-east1`, `us-east4`, `us-south1`, `us-west1`, `southamerica-east1`, `europe-north1`, `europe-southwest1`, `europe-west1`, `europe-west2`, `europe-west3`, `europe-west4`, `europe-west8`, `europe-west9`, `asia-east1`, `asia-south1`, `asia-southeast1`, `me-west1`, `australia-southeast1`. **Modifying this attribute will force creation of a new resource.**
    pub region: pulumi_wasm_rust::Output<String>,
    /// The page to run the test on. **Modifying this attribute will force creation of a new resource.**
    pub url: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: ObservatoryScheduledTestArgs) -> ObservatoryScheduledTestResult {
    let result = crate::bindings::pulumi::cloudflare::observatory_scheduled_test::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::observatory_scheduled_test::Args {
            frequency: args.frequency.get_inner(),
            region: args.region.get_inner(),
            url: args.url.get_inner(),
            zone_id: args.zone_id.get_inner(),
        },
    );

    ObservatoryScheduledTestResult {
        frequency: crate::into_domain(result.frequency),
        region: crate::into_domain(result.region),
        url: crate::into_domain(result.url),
        zone_id: crate::into_domain(result.zone_id),
    }
}
