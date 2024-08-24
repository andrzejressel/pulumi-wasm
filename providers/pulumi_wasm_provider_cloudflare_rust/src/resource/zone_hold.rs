//! Provides a Cloudflare Zone Hold resource that prevents adding
//! the hostname to another account for use.
//!
//! ## Example Usage
//!
//! <!--Start PulumiCodeChooser -->
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as cloudflare from "@pulumi/cloudflare";
//!
//! const example = new cloudflare.ZoneHold("example", {
//!     hold: true,
//!     zoneId: "0da42c8d2132a9ddaf714f9e7c920711",
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//!
//! example = cloudflare.ZoneHold("example",
//!     hold=True,
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
//!     var example = new Cloudflare.ZoneHold("example", new()
//!     {
//!         Hold = true,
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
//! 		_, err := cloudflare.NewZoneHold(ctx, "example", &cloudflare.ZoneHoldArgs{
//! 			Hold:   pulumi.Bool(true),
//! 			ZoneId: pulumi.String("0da42c8d2132a9ddaf714f9e7c920711"),
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
//! import com.pulumi.cloudflare.ZoneHold;
//! import com.pulumi.cloudflare.ZoneHoldArgs;
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
//!         var example = new ZoneHold("example", ZoneHoldArgs.builder()        
//!             .hold(true)
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
//!     type: cloudflare:ZoneHold
//!     properties:
//!       hold: true
//!       zoneId: 0da42c8d2132a9ddaf714f9e7c920711
//! ```
//! <!--End PulumiCodeChooser -->
//!
//! ## Import
//!
//! ```sh
//! $ pulumi import cloudflare:index/zoneHold:ZoneHold example <zone_id>
//! ```
//!

pub struct ZoneHoldArgs {
    /// Enablement status of the zone hold.
    pub hold: pulumi_wasm_rust::Output<bool>,
    /// The RFC3339 compatible timestamp when to automatically re-enable the zone hold.
    pub hold_after: pulumi_wasm_rust::Output<Option<String>>,
    /// Whether to extend to block any subdomain of the given zone.
    pub include_subdomains: pulumi_wasm_rust::Output<Option<bool>>,
    /// The zone identifier to target for the resource.
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct ZoneHoldResult {
    /// Enablement status of the zone hold.
    pub hold: pulumi_wasm_rust::Output<bool>,
    /// The RFC3339 compatible timestamp when to automatically re-enable the zone hold.
    pub hold_after: pulumi_wasm_rust::Output<String>,
    /// Whether to extend to block any subdomain of the given zone.
    pub include_subdomains: pulumi_wasm_rust::Output<Option<bool>>,
    /// The zone identifier to target for the resource.
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: ZoneHoldArgs) -> ZoneHoldResult {
    let result = crate::bindings::pulumi::cloudflare::zone_hold::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::zone_hold::Args {
            hold: &args.hold.get_inner(),
            hold_after: &args.hold_after.get_inner(),
            include_subdomains: &args.include_subdomains.get_inner(),
            zone_id: &args.zone_id.get_inner(),
        },
    );

    ZoneHoldResult {
        hold: crate::into_domain(result.hold),
        hold_after: crate::into_domain(result.hold_after),
        include_subdomains: crate::into_domain(result.include_subdomains),
        zone_id: crate::into_domain(result.zone_id),
    }
}
