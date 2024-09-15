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
//! const exampleRecord = new cloudflare.Record("exampleRecord", {
//!     name: "example.com",
//!     ttl: 3600,
//!     type: "A",
//!     value: "192.0.2.1",
//!     zoneId: "0da42c8d2132a9ddaf714f9e7c920711",
//! });
//! // The cloudflare_regional_hostname resource may exist with or without its
//! // corresponding record resource.
//! const exampleRegionalHostname = new cloudflare.RegionalHostname("exampleRegionalHostname", {
//!     hostname: "example.com",
//!     regionKey: "eu",
//!     zoneId: "0da42c8d2132a9ddaf714f9e7c920711",
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//! 
//! # Regionalized hostname record resources are managed independently from the
//! # Regionalized Hostname resources.
//! example_record = cloudflare.Record("exampleRecord",
//!     name="example.com",
//!     ttl=3600,
//!     type="A",
//!     value="192.0.2.1",
//!     zone_id="0da42c8d2132a9ddaf714f9e7c920711")
//! # The cloudflare_regional_hostname resource may exist with or without its
//! # corresponding record resource.
//! example_regional_hostname = cloudflare.RegionalHostname("exampleRegionalHostname",
//!     hostname="example.com",
//!     region_key="eu",
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
//!     // Regionalized hostname record resources are managed independently from the
//!     // Regionalized Hostname resources.
//!     var exampleRecord = new Cloudflare.Record("exampleRecord", new()
//!     {
//!         Name = "example.com",
//!         Ttl = 3600,
//!         Type = "A",
//!         Value = "192.0.2.1",
//!         ZoneId = "0da42c8d2132a9ddaf714f9e7c920711",
//!     });
//! 
//!     // The cloudflare_regional_hostname resource may exist with or without its
//!     // corresponding record resource.
//!     var exampleRegionalHostname = new Cloudflare.RegionalHostname("exampleRegionalHostname", new()
//!     {
//!         Hostname = "example.com",
//!         RegionKey = "eu",
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
//! 		// Regionalized hostname record resources are managed independently from the
//! 		// Regionalized Hostname resources.
//! 		_, err := cloudflare.NewRecord(ctx, "exampleRecord", &cloudflare.RecordArgs{
//! 			Name:   pulumi.String("example.com"),
//! 			Ttl:    pulumi.Int(3600),
//! 			Type:   pulumi.String("A"),
//! 			Value:  pulumi.String("192.0.2.1"),
//! 			ZoneId: pulumi.String("0da42c8d2132a9ddaf714f9e7c920711"),
//! 		})
//! 		if err != nil {
//! 			return err
//! 		}
//! 		// The cloudflare_regional_hostname resource may exist with or without its
//! 		// corresponding record resource.
//! 		_, err = cloudflare.NewRegionalHostname(ctx, "exampleRegionalHostname", &cloudflare.RegionalHostnameArgs{
//! 			Hostname:  pulumi.String("example.com"),
//! 			RegionKey: pulumi.String("eu"),
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
//!         var exampleRecord = new Record("exampleRecord", RecordArgs.builder()        
//!             .name("example.com")
//!             .ttl(3600)
//!             .type("A")
//!             .value("192.0.2.1")
//!             .zoneId("0da42c8d2132a9ddaf714f9e7c920711")
//!             .build());
//! 
//!         // The cloudflare_regional_hostname resource may exist with or without its
//!         // corresponding record resource.
//!         var exampleRegionalHostname = new RegionalHostname("exampleRegionalHostname", RegionalHostnameArgs.builder()        
//!             .hostname("example.com")
//!             .regionKey("eu")
//!             .zoneId("0da42c8d2132a9ddaf714f9e7c920711")
//!             .build());
//! 
//!     }
//! }
//! ```
//! ### YAML
//! ```yaml
//! resources:
//!   # Regionalized hostname record resources are managed independently from the
//!   # // Regionalized Hostname resources.
//!   exampleRecord:
//!     type: cloudflare:Record
//!     properties:
//!       name: example.com
//!       ttl: 3600
//!       type: A
//!       value: 192.0.2.1
//!       zoneId: 0da42c8d2132a9ddaf714f9e7c920711
//!   # The cloudflare_regional_hostname resource may exist with or without its
//!   # // corresponding record resource.
//!   exampleRegionalHostname:
//!     type: cloudflare:RegionalHostname
//!     properties:
//!       hostname: example.com
//!       regionKey: eu
//!       zoneId: 0da42c8d2132a9ddaf714f9e7c920711
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
