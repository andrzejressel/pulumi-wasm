//! Instructs Cloudflare to check a regional hub data center on the way to your upper tier.
//! This can help improve performance for smart and custom tiered cache topologies.
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as cloudflare from "@pulumi/cloudflare";
//! 
//! const example = new cloudflare.RegionalTieredCache("example", {
//!     zoneId: "0da42c8d2132a9ddaf714f9e7c920711",
//!     value: "on",
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//! 
//! example = cloudflare.RegionalTieredCache("example",
//!     zone_id="0da42c8d2132a9ddaf714f9e7c920711",
//!     value="on")
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
//!     var example = new Cloudflare.RegionalTieredCache("example", new()
//!     {
//!         ZoneId = "0da42c8d2132a9ddaf714f9e7c920711",
//!         Value = "on",
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
//! 		_, err := cloudflare.NewRegionalTieredCache(ctx, "example", &cloudflare.RegionalTieredCacheArgs{
//! 			ZoneId: pulumi.String("0da42c8d2132a9ddaf714f9e7c920711"),
//! 			Value:  pulumi.String("on"),
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
//! import com.pulumi.cloudflare.RegionalTieredCache;
//! import com.pulumi.cloudflare.RegionalTieredCacheArgs;
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
//!         var example = new RegionalTieredCache("example", RegionalTieredCacheArgs.builder()
//!             .zoneId("0da42c8d2132a9ddaf714f9e7c920711")
//!             .value("on")
//!             .build());
//! 
//!     }
//! }
//! ```
//! ### YAML
//! ```yaml
//! resources:
//!   example:
//!     type: cloudflare:RegionalTieredCache
//!     properties:
//!       zoneId: 0da42c8d2132a9ddaf714f9e7c920711
//!       value: on
//! ```
//! <!--End PulumiCodeChooser -->
//! 
//! ## Import
//! 
//! ```sh
//! $ pulumi import cloudflare:index/regionalTieredCache:RegionalTieredCache example <zone_id>
//! ```
//! 

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct RegionalTieredCacheArgs {
    /// Value of the Regional Tiered Cache zone setting.
    #[builder(into)]
    pub value: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource.
    #[builder(into)]
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct RegionalTieredCacheResult {
    /// Value of the Regional Tiered Cache zone setting.
    pub value: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource.
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: RegionalTieredCacheArgs) -> RegionalTieredCacheResult {

    let result = crate::bindings::pulumi::cloudflare::regional_tiered_cache::invoke(name, &crate::bindings::pulumi::cloudflare::regional_tiered_cache::Args {
        value: &args.value.get_inner(),
        zone_id: &args.zone_id.get_inner(),
    });

    RegionalTieredCacheResult {
        value: crate::into_domain(result.value),
        zone_id: crate::into_domain(result.zone_id),
    }
}
