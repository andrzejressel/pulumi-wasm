//! Provides a Cloudflare Cache Reserve resource. Cache Reserve can
//! increase cache lifetimes by automatically storing all cacheable
//! files in Cloudflare's persistent object storage buckets.
//!
//! Note: Using Cache Reserve without Tiered Cache is not recommended.
//!
//! ## Example Usage
//!
//! <!--Start PulumiCodeChooser -->
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as cloudflare from "@pulumi/cloudflare";
//!
//! const example = new cloudflare.ZoneCacheReserve("example", {
//!     enabled: true,
//!     zoneId: "0da42c8d2132a9ddaf714f9e7c920711",
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//!
//! example = cloudflare.ZoneCacheReserve("example",
//!     enabled=True,
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
//!     var example = new Cloudflare.ZoneCacheReserve("example", new()
//!     {
//!         Enabled = true,
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
//! 		_, err := cloudflare.NewZoneCacheReserve(ctx, "example", &cloudflare.ZoneCacheReserveArgs{
//! 			Enabled: pulumi.Bool(true),
//! 			ZoneId:  pulumi.String("0da42c8d2132a9ddaf714f9e7c920711"),
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
//! import com.pulumi.cloudflare.ZoneCacheReserve;
//! import com.pulumi.cloudflare.ZoneCacheReserveArgs;
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
//!         var example = new ZoneCacheReserve("example", ZoneCacheReserveArgs.builder()        
//!             .enabled(true)
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
//!     type: cloudflare:ZoneCacheReserve
//!     properties:
//!       enabled: true
//!       zoneId: 0da42c8d2132a9ddaf714f9e7c920711
//! ```
//! <!--End PulumiCodeChooser -->
//!
//! ## Import
//!
//! ```sh
//! $ pulumi import cloudflare:index/zoneCacheReserve:ZoneCacheReserve example <zone_id>
//! ```
//!

pub struct ZoneCacheReserveArgs {
    /// Whether to enable or disable Cache Reserve support for a given zone.
    pub enabled: pulumi_wasm_rust::Output<bool>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct ZoneCacheReserveResult {
    /// Whether to enable or disable Cache Reserve support for a given zone.
    pub enabled: pulumi_wasm_rust::Output<bool>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: ZoneCacheReserveArgs) -> ZoneCacheReserveResult {
    let result = crate::bindings::pulumi::cloudflare::zone_cache_reserve::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::zone_cache_reserve::Args {
            enabled: args.enabled.get_inner(),
            zone_id: args.zone_id.get_inner(),
        },
    );

    ZoneCacheReserveResult {
        enabled: crate::into_domain(result.enabled),
        zone_id: crate::into_domain(result.zone_id),
    }
}
