//! Provides a resource to manage URL Normalization Settings.
//!
//! ## Example Usage
//!
//! <!--Start PulumiCodeChooser -->
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as cloudflare from "@pulumi/cloudflare";
//!
//! const example = new cloudflare.UrlNormalizationSettings("example", {
//!     scope: "incoming",
//!     type: "cloudflare",
//!     zoneId: "0da42c8d2132a9ddaf714f9e7c920711",
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//!
//! example = cloudflare.UrlNormalizationSettings("example",
//!     scope="incoming",
//!     type="cloudflare",
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
//!     var example = new Cloudflare.UrlNormalizationSettings("example", new()
//!     {
//!         Scope = "incoming",
//!         Type = "cloudflare",
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
//! 		_, err := cloudflare.NewUrlNormalizationSettings(ctx, "example", &cloudflare.UrlNormalizationSettingsArgs{
//! 			Scope:  pulumi.String("incoming"),
//! 			Type:   pulumi.String("cloudflare"),
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
//! import com.pulumi.cloudflare.UrlNormalizationSettings;
//! import com.pulumi.cloudflare.UrlNormalizationSettingsArgs;
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
//!         var example = new UrlNormalizationSettings("example", UrlNormalizationSettingsArgs.builder()        
//!             .scope("incoming")
//!             .type("cloudflare")
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
//!     type: cloudflare:UrlNormalizationSettings
//!     properties:
//!       scope: incoming
//!       type: cloudflare
//!       zoneId: 0da42c8d2132a9ddaf714f9e7c920711
//! ```
//! <!--End PulumiCodeChooser -->

pub struct UrlNormalizationSettingsArgs {
    /// The scope of the URL normalization.
    pub scope: pulumi_wasm_rust::Output<String>,
    /// The type of URL normalization performed by Cloudflare.
    pub type_: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct UrlNormalizationSettingsResult {
    /// The scope of the URL normalization.
    pub scope: pulumi_wasm_rust::Output<String>,
    /// The type of URL normalization performed by Cloudflare.
    pub type_: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: UrlNormalizationSettingsArgs) -> UrlNormalizationSettingsResult {
    let result = crate::bindings::pulumi::cloudflare::url_normalization_settings::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::url_normalization_settings::Args {
            scope: &args.scope.get_inner(),
            type_: &args.type_.get_inner(),
            zone_id: &args.zone_id.get_inner(),
        },
    );

    UrlNormalizationSettingsResult {
        scope: crate::into_domain(result.scope),
        type_: crate::into_domain(result.type_),
        zone_id: crate::into_domain(result.zone_id),
    }
}
