//! Provides a resource, that manages Cloudflare Tiered Cache settings.
//! This allows you to adjust topologies for your zone.
//!
//! ## Example Usage
//!
//! <!--Start PulumiCodeChooser -->
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as cloudflare from "@pulumi/cloudflare";
//!
//! const example = new cloudflare.TieredCache("example", {
//!     cacheType: "smart",
//!     zoneId: "0da42c8d2132a9ddaf714f9e7c920711",
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//!
//! example = cloudflare.TieredCache("example",
//!     cache_type="smart",
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
//!     var example = new Cloudflare.TieredCache("example", new()
//!     {
//!         CacheType = "smart",
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
//! 		_, err := cloudflare.NewTieredCache(ctx, "example", &cloudflare.TieredCacheArgs{
//! 			CacheType: pulumi.String("smart"),
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
//! import com.pulumi.cloudflare.TieredCache;
//! import com.pulumi.cloudflare.TieredCacheArgs;
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
//!         var example = new TieredCache("example", TieredCacheArgs.builder()        
//!             .cacheType("smart")
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
//!     type: cloudflare:TieredCache
//!     properties:
//!       cacheType: smart
//!       zoneId: 0da42c8d2132a9ddaf714f9e7c920711
//! ```
//! <!--End PulumiCodeChooser -->

pub struct TieredCacheArgs {
    /// The typed of tiered cache to utilize on the zone. Available values: `generic`, `smart`, `off`.
    pub cache_type: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct TieredCacheResult {
    /// The typed of tiered cache to utilize on the zone. Available values: `generic`, `smart`, `off`.
    pub cache_type: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: TieredCacheArgs) -> TieredCacheResult {
    let result = crate::bindings::pulumi::cloudflare::tiered_cache::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::tiered_cache::Args {
            cache_type: &args.cache_type.get_inner(),
            zone_id: &args.zone_id.get_inner(),
        },
    );

    TieredCacheResult {
        cache_type: crate::into_domain(result.cache_type),
        zone_id: crate::into_domain(result.zone_id),
    }
}
