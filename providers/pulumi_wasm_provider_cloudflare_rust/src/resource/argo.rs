//! Cloudflare Argo controls the routing to your origin and tiered
//! caching options to speed up your website browsing experience.
//!
//! ## Example Usage
//!
//! <!--Start PulumiCodeChooser -->
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as cloudflare from "@pulumi/cloudflare";
//!
//! const example = new cloudflare.Argo("example", {
//!     smartRouting: "on",
//!     tieredCaching: "on",
//!     zoneId: "0da42c8d2132a9ddaf714f9e7c920711",
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//!
//! example = cloudflare.Argo("example",
//!     smart_routing="on",
//!     tiered_caching="on",
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
//!     var example = new Cloudflare.Argo("example", new()
//!     {
//!         SmartRouting = "on",
//!         TieredCaching = "on",
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
//! 		_, err := cloudflare.NewArgo(ctx, "example", &cloudflare.ArgoArgs{
//! 			SmartRouting:  pulumi.String("on"),
//! 			TieredCaching: pulumi.String("on"),
//! 			ZoneId:        pulumi.String("0da42c8d2132a9ddaf714f9e7c920711"),
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
//! import com.pulumi.cloudflare.Argo;
//! import com.pulumi.cloudflare.ArgoArgs;
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
//!         var example = new Argo("example", ArgoArgs.builder()        
//!             .smartRouting("on")
//!             .tieredCaching("on")
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
//!     type: cloudflare:Argo
//!     properties:
//!       smartRouting: on
//!       tieredCaching: on
//!       zoneId: 0da42c8d2132a9ddaf714f9e7c920711
//! ```
//! <!--End PulumiCodeChooser -->
//!
//! ## Import
//!
//! ```sh
//! $ pulumi import cloudflare:index/argo:Argo example <zone_id>
//! ```
//!

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct ArgoArgs {
    /// Whether smart routing is enabled. Available values: `on`, `off`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub smart_routing: pulumi_wasm_rust::Output<Option<String>>,
    /// Whether tiered caching is enabled. Available values: `on`, `off`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub tiered_caching: pulumi_wasm_rust::Output<Option<String>>,
    /// The zone identifier to target for the resource.
    #[builder(into)]
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct ArgoResult {
    /// Whether smart routing is enabled. Available values: `on`, `off`.
    pub smart_routing: pulumi_wasm_rust::Output<Option<String>>,
    /// Whether tiered caching is enabled. Available values: `on`, `off`.
    pub tiered_caching: pulumi_wasm_rust::Output<Option<String>>,
    /// The zone identifier to target for the resource.
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: ArgoArgs) -> ArgoResult {
    let result = crate::bindings::pulumi::cloudflare::argo::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::argo::Args {
            smart_routing: &args.smart_routing.get_inner(),
            tiered_caching: &args.tiered_caching.get_inner(),
            zone_id: &args.zone_id.get_inner(),
        },
    );

    ArgoResult {
        smart_routing: crate::into_domain(result.smart_routing),
        tiered_caching: crate::into_domain(result.tiered_caching),
        zone_id: crate::into_domain(result.zone_id),
    }
}
