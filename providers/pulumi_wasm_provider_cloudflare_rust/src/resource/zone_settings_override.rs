//! Provides a resource which customizes Cloudflare zone settings.
//! 
//! > You **should not** use this resource to manage every zone setting. This
//!   resource is only intended to override those which you do not want the default.
//!   Attempting to manage all settings will result in problems with the resource
//!   applying in a consistent manner.
//! 
//! ## Plan-Dependent Settings
//! 
//! Note that some settings are only available on certain plans. Setting an argument
//! for a feature that is not available on the plan configured for the zone will
//! result in an error:
//! 
//! ```
//! Error: invalid zone setting "\<argument\>" (value: \<value\>) found - cannot be set as it is read only
//! ```
//! 
//! This is true even when setting the argument to its default value. These values
//! should either be omitted or set to `null` for zones with plans that don't
//! support the feature. See the [plan feature matrices](https://www.cloudflare.com/plans/) for details on
//! feature support by plan.
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as cloudflare from "@pulumi/cloudflare";
//! 
//! const test = new cloudflare.ZoneSettingsOverride("test", {
//!     zoneId: "0da42c8d2132a9ddaf714f9e7c920711",
//!     settings: {
//!         brotli: "on",
//!         challengeTtl: 2700,
//!         securityLevel: "high",
//!         opportunisticEncryption: "on",
//!         automaticHttpsRewrites: "on",
//!         mirage: "on",
//!         waf: "on",
//!         minify: {
//!             css: "on",
//!             js: "off",
//!             html: "off",
//!         },
//!         securityHeader: {
//!             enabled: true,
//!         },
//!     },
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//! 
//! test = cloudflare.ZoneSettingsOverride("test",
//!     zone_id="0da42c8d2132a9ddaf714f9e7c920711",
//!     settings={
//!         "brotli": "on",
//!         "challenge_ttl": 2700,
//!         "security_level": "high",
//!         "opportunistic_encryption": "on",
//!         "automatic_https_rewrites": "on",
//!         "mirage": "on",
//!         "waf": "on",
//!         "minify": {
//!             "css": "on",
//!             "js": "off",
//!             "html": "off",
//!         },
//!         "security_header": {
//!             "enabled": True,
//!         },
//!     })
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
//!     var test = new Cloudflare.ZoneSettingsOverride("test", new()
//!     {
//!         ZoneId = "0da42c8d2132a9ddaf714f9e7c920711",
//!         Settings = new Cloudflare.Inputs.ZoneSettingsOverrideSettingsArgs
//!         {
//!             Brotli = "on",
//!             ChallengeTtl = 2700,
//!             SecurityLevel = "high",
//!             OpportunisticEncryption = "on",
//!             AutomaticHttpsRewrites = "on",
//!             Mirage = "on",
//!             Waf = "on",
//!             Minify = new Cloudflare.Inputs.ZoneSettingsOverrideSettingsMinifyArgs
//!             {
//!                 Css = "on",
//!                 Js = "off",
//!                 Html = "off",
//!             },
//!             SecurityHeader = new Cloudflare.Inputs.ZoneSettingsOverrideSettingsSecurityHeaderArgs
//!             {
//!                 Enabled = true,
//!             },
//!         },
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
//! 		_, err := cloudflare.NewZoneSettingsOverride(ctx, "test", &cloudflare.ZoneSettingsOverrideArgs{
//! 			ZoneId: pulumi.String("0da42c8d2132a9ddaf714f9e7c920711"),
//! 			Settings: &cloudflare.ZoneSettingsOverrideSettingsArgs{
//! 				Brotli:                  pulumi.String("on"),
//! 				ChallengeTtl:            pulumi.Int(2700),
//! 				SecurityLevel:           pulumi.String("high"),
//! 				OpportunisticEncryption: pulumi.String("on"),
//! 				AutomaticHttpsRewrites:  pulumi.String("on"),
//! 				Mirage:                  pulumi.String("on"),
//! 				Waf:                     pulumi.String("on"),
//! 				Minify: &cloudflare.ZoneSettingsOverrideSettingsMinifyArgs{
//! 					Css:  pulumi.String("on"),
//! 					Js:   pulumi.String("off"),
//! 					Html: pulumi.String("off"),
//! 				},
//! 				SecurityHeader: &cloudflare.ZoneSettingsOverrideSettingsSecurityHeaderArgs{
//! 					Enabled: pulumi.Bool(true),
//! 				},
//! 			},
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
//! import com.pulumi.cloudflare.ZoneSettingsOverride;
//! import com.pulumi.cloudflare.ZoneSettingsOverrideArgs;
//! import com.pulumi.cloudflare.inputs.ZoneSettingsOverrideSettingsArgs;
//! import com.pulumi.cloudflare.inputs.ZoneSettingsOverrideSettingsMinifyArgs;
//! import com.pulumi.cloudflare.inputs.ZoneSettingsOverrideSettingsSecurityHeaderArgs;
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
//!         var test = new ZoneSettingsOverride("test", ZoneSettingsOverrideArgs.builder()
//!             .zoneId("0da42c8d2132a9ddaf714f9e7c920711")
//!             .settings(ZoneSettingsOverrideSettingsArgs.builder()
//!                 .brotli("on")
//!                 .challengeTtl(2700)
//!                 .securityLevel("high")
//!                 .opportunisticEncryption("on")
//!                 .automaticHttpsRewrites("on")
//!                 .mirage("on")
//!                 .waf("on")
//!                 .minify(ZoneSettingsOverrideSettingsMinifyArgs.builder()
//!                     .css("on")
//!                     .js("off")
//!                     .html("off")
//!                     .build())
//!                 .securityHeader(ZoneSettingsOverrideSettingsSecurityHeaderArgs.builder()
//!                     .enabled(true)
//!                     .build())
//!                 .build())
//!             .build());
//! 
//!     }
//! }
//! ```
//! ### YAML
//! ```yaml
//! resources:
//!   test:
//!     type: cloudflare:ZoneSettingsOverride
//!     properties:
//!       zoneId: 0da42c8d2132a9ddaf714f9e7c920711
//!       settings:
//!         brotli: on
//!         challengeTtl: 2700
//!         securityLevel: high
//!         opportunisticEncryption: on
//!         automaticHttpsRewrites: on
//!         mirage: on
//!         waf: on
//!         minify:
//!           css: on
//!           js: off
//!           html: off
//!         securityHeader:
//!           enabled: true
//! ```
//! <!--End PulumiCodeChooser -->

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct ZoneSettingsOverrideArgs {
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub settings: pulumi_wasm_rust::Output<Option<crate::types::ZoneSettingsOverrideSettings>>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct ZoneSettingsOverrideResult {
    pub initial_settings: pulumi_wasm_rust::Output<Vec<crate::types::ZoneSettingsOverrideInitialSetting>>,
    pub initial_settings_read_at: pulumi_wasm_rust::Output<String>,
    pub readonly_settings: pulumi_wasm_rust::Output<Vec<String>>,
    pub settings: pulumi_wasm_rust::Output<crate::types::ZoneSettingsOverrideSettings>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub zone_id: pulumi_wasm_rust::Output<String>,
    pub zone_status: pulumi_wasm_rust::Output<String>,
    pub zone_type: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: ZoneSettingsOverrideArgs) -> ZoneSettingsOverrideResult {

    let result = crate::bindings::pulumi::cloudflare::zone_settings_override::invoke(name, &crate::bindings::pulumi::cloudflare::zone_settings_override::Args {
        settings: &args.settings.get_inner(),
        zone_id: &args.zone_id.get_inner(),
    });

    ZoneSettingsOverrideResult {
        initial_settings: crate::into_domain(result.initial_settings),
        initial_settings_read_at: crate::into_domain(result.initial_settings_read_at),
        readonly_settings: crate::into_domain(result.readonly_settings),
        settings: crate::into_domain(result.settings),
        zone_id: crate::into_domain(result.zone_id),
        zone_status: crate::into_domain(result.zone_status),
        zone_type: crate::into_domain(result.zone_type),
    }
}
