//! Provides a Data Localization Suite Regional Hostname.
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as cloudflare from "@pulumi/cloudflare";
//! 
//! // Regionalized hostname record resources are managed independently from the
//! // Regionalized Hostname resources.
//! const example = new cloudflare.Record("example", {
//!     zoneId: "0da42c8d2132a9ddaf714f9e7c920711",
//!     name: "example.com",
//!     content: "192.0.2.1",
//!     type: "A",
//!     ttl: 3600,
//! });
//! // The cloudflare_regional_hostname resource may exist with or without its
//! // corresponding record resource.
//! const exampleRegionalHostname = new cloudflare.RegionalHostname("example", {
//!     zoneId: "0da42c8d2132a9ddaf714f9e7c920711",
//!     hostname: "example.com",
//!     regionKey: "eu",
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//! 
//! # Regionalized hostname record resources are managed independently from the
//! # Regionalized Hostname resources.
//! example = cloudflare.Record("example",
//!     zone_id="0da42c8d2132a9ddaf714f9e7c920711",
//!     name="example.com",
//!     content="192.0.2.1",
//!     type="A",
//!     ttl=3600)
//! # The cloudflare_regional_hostname resource may exist with or without its
//! # corresponding record resource.
//! example_regional_hostname = cloudflare.RegionalHostname("example",
//!     zone_id="0da42c8d2132a9ddaf714f9e7c920711",
//!     hostname="example.com",
//!     region_key="eu")
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
//!     // Regionalized hostname record resources are managed independently from the
//!     // Regionalized Hostname resources.
//!     var example = new Cloudflare.Record("example", new()
//!     {
//!         ZoneId = "0da42c8d2132a9ddaf714f9e7c920711",
//!         Name = "example.com",
//!         Content = "192.0.2.1",
//!         Type = "A",
//!         Ttl = 3600,
//!     });
//! 
//!     // The cloudflare_regional_hostname resource may exist with or without its
//!     // corresponding record resource.
//!     var exampleRegionalHostname = new Cloudflare.RegionalHostname("example", new()
//!     {
//!         ZoneId = "0da42c8d2132a9ddaf714f9e7c920711",
//!         Hostname = "example.com",
//!         RegionKey = "eu",
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
//! 		// Regionalized hostname record resources are managed independently from the
//! 		// Regionalized Hostname resources.
//! 		_, err := cloudflare.NewRecord(ctx, "example", &cloudflare.RecordArgs{
//! 			ZoneId:  pulumi.String("0da42c8d2132a9ddaf714f9e7c920711"),
//! 			Name:    pulumi.String("example.com"),
//! 			Content: pulumi.String("192.0.2.1"),
//! 			Type:    pulumi.String("A"),
//! 			Ttl:     pulumi.Int(3600),
//! 		})
//! 		if err != nil {
//! 			return err
//! 		}
//! 		// The cloudflare_regional_hostname resource may exist with or without its
//! 		// corresponding record resource.
//! 		_, err = cloudflare.NewRegionalHostname(ctx, "example", &cloudflare.RegionalHostnameArgs{
//! 			ZoneId:    pulumi.String("0da42c8d2132a9ddaf714f9e7c920711"),
//! 			Hostname:  pulumi.String("example.com"),
//! 			RegionKey: pulumi.String("eu"),
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
//! import com.pulumi.cloudflare.Record;
//! import com.pulumi.cloudflare.RecordArgs;
//! import com.pulumi.cloudflare.RegionalHostname;
//! import com.pulumi.cloudflare.RegionalHostnameArgs;
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
//!         // Regionalized hostname record resources are managed independently from the
//!         // Regionalized Hostname resources.
//!         var example = new Record("example", RecordArgs.builder()
//!             .zoneId("0da42c8d2132a9ddaf714f9e7c920711")
//!             .name("example.com")
//!             .content("192.0.2.1")
//!             .type("A")
//!             .ttl(3600)
//!             .build());
//! 
//!         // The cloudflare_regional_hostname resource may exist with or without its
//!         // corresponding record resource.
//!         var exampleRegionalHostname = new RegionalHostname("exampleRegionalHostname", RegionalHostnameArgs.builder()
//!             .zoneId("0da42c8d2132a9ddaf714f9e7c920711")
//!             .hostname("example.com")
//!             .regionKey("eu")
//!             .build());
//! 
//!     }
//! }
//! ```
//! ### YAML
//! ```yaml
//! resources:
//!   # Regionalized hostname record resources are managed independently from the
//!   # Regionalized Hostname resources.
//!   example:
//!     type: cloudflare:Record
//!     properties:
//!       zoneId: 0da42c8d2132a9ddaf714f9e7c920711
//!       name: example.com
//!       content: 192.0.2.1
//!       type: A
//!       ttl: 3600
//!   # The cloudflare_regional_hostname resource may exist with or without its
//!   # corresponding record resource.
//!   exampleRegionalHostname:
//!     type: cloudflare:RegionalHostname
//!     name: example
//!     properties:
//!       zoneId: 0da42c8d2132a9ddaf714f9e7c920711
//!       hostname: example.com
//!       regionKey: eu
//! ```
//! <!--End PulumiCodeChooser -->

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct RegionalHostnameArgs {
    /// The hostname to regionalize.
    #[builder(into)]
    pub hostname: pulumi_wasm_rust::Output<String>,
    /// The region key. See [the full region list](https://developers.cloudflare.com/data-localization/regional-services/get-started/).
    #[builder(into)]
    pub region_key: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource.
    #[builder(into)]
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct RegionalHostnameResult {
    /// The RFC3339 timestamp of when the hostname was created.
    pub created_on: pulumi_wasm_rust::Output<String>,
    /// The hostname to regionalize.
    pub hostname: pulumi_wasm_rust::Output<String>,
    /// The region key. See [the full region list](https://developers.cloudflare.com/data-localization/regional-services/get-started/).
    pub region_key: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource.
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: RegionalHostnameArgs) -> RegionalHostnameResult {

    let result = crate::bindings::pulumi::cloudflare::regional_hostname::invoke(name, &crate::bindings::pulumi::cloudflare::regional_hostname::Args {
        hostname: &args.hostname.get_inner(),
        region_key: &args.region_key.get_inner(),
        zone_id: &args.zone_id.get_inner(),
    });

    RegionalHostnameResult {
        created_on: crate::into_domain(result.created_on),
        hostname: crate::into_domain(result.hostname),
        region_key: crate::into_domain(result.region_key),
        zone_id: crate::into_domain(result.zone_id),
    }
}
