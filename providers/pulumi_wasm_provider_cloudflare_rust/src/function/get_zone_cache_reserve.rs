//! Provides a Cloudflare data source to look up Cache Reserve
//! status for a given zone.
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as cloudflare from "@pulumi/cloudflare";
//! 
//! const example = cloudflare.getZoneCacheReserve({
//!     zoneId: "0da42c8d2132a9ddaf714f9e7c920711",
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//! 
//! example = cloudflare.get_zone_cache_reserve(zone_id="0da42c8d2132a9ddaf714f9e7c920711")
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
//!     var example = Cloudflare.GetZoneCacheReserve.Invoke(new()
//!     {
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
//! 		_, err := cloudflare.LookupZoneCacheReserve(ctx, &cloudflare.LookupZoneCacheReserveArgs{
//! 			ZoneId: "0da42c8d2132a9ddaf714f9e7c920711",
//! 		}, nil)
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
//! import com.pulumi.cloudflare.CloudflareFunctions;
//! import com.pulumi.cloudflare.inputs.GetZoneCacheReserveArgs;
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
//!         final var example = CloudflareFunctions.getZoneCacheReserve(GetZoneCacheReserveArgs.builder()
//!             .zoneId("0da42c8d2132a9ddaf714f9e7c920711")
//!             .build());
//! 
//!     }
//! }
//! ```
//! ### YAML
//! ```yaml
//! variables:
//!   example:
//!     fn::invoke:
//!       Function: cloudflare:getZoneCacheReserve
//!       Arguments:
//!         zoneId: 0da42c8d2132a9ddaf714f9e7c920711
//! ```
//! <!--End PulumiCodeChooser -->

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct GetZoneCacheReserveArgs {
    /// The zone identifier to target for the resource.
    #[builder(into)]
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct GetZoneCacheReserveResult {
    /// The status of Cache Reserve support.
    pub enabled: pulumi_wasm_rust::Output<bool>,
    /// The provider-assigned unique ID for this managed resource.
    pub id: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource.
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn invoke(
    args: GetZoneCacheReserveArgs
) -> GetZoneCacheReserveResult {

    let result = crate::bindings::pulumi::cloudflare::get_zone_cache_reserve::invoke(
        &crate::bindings::pulumi::cloudflare::get_zone_cache_reserve::Args {
                zone_id: &args.zone_id.get_inner(),
        }
    );

    GetZoneCacheReserveResult {
        enabled: crate::into_domain(result.enabled),
        id: crate::into_domain(result.id),
        zone_id: crate::into_domain(result.zone_id),
    }
}
