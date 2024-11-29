//! Provides a resource for managing Email Routing settings.
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as cloudflare from "@pulumi/cloudflare";
//! 
//! const myZone = new cloudflare.EmailRoutingSettings("my_zone", {
//!     zoneId: "0da42c8d2132a9ddaf714f9e7c920711",
//!     enabled: true,
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//! 
//! my_zone = cloudflare.EmailRoutingSettings("my_zone",
//!     zone_id="0da42c8d2132a9ddaf714f9e7c920711",
//!     enabled=True)
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
//!     var myZone = new Cloudflare.EmailRoutingSettings("my_zone", new()
//!     {
//!         ZoneId = "0da42c8d2132a9ddaf714f9e7c920711",
//!         Enabled = true,
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
//! 		_, err := cloudflare.NewEmailRoutingSettings(ctx, "my_zone", &cloudflare.EmailRoutingSettingsArgs{
//! 			ZoneId:  pulumi.String("0da42c8d2132a9ddaf714f9e7c920711"),
//! 			Enabled: pulumi.Bool(true),
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
//! import com.pulumi.cloudflare.EmailRoutingSettings;
//! import com.pulumi.cloudflare.EmailRoutingSettingsArgs;
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
//!         var myZone = new EmailRoutingSettings("myZone", EmailRoutingSettingsArgs.builder()
//!             .zoneId("0da42c8d2132a9ddaf714f9e7c920711")
//!             .enabled("true")
//!             .build());
//! 
//!     }
//! }
//! ```
//! ### YAML
//! ```yaml
//! resources:
//!   myZone:
//!     type: cloudflare:EmailRoutingSettings
//!     name: my_zone
//!     properties:
//!       zoneId: 0da42c8d2132a9ddaf714f9e7c920711
//!       enabled: 'true'
//! ```
//! <!--End PulumiCodeChooser -->

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct EmailRoutingSettingsArgs {
    /// State of the zone settings for Email Routing. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub enabled: pulumi_wasm_rust::Output<bool>,
    /// Flag to check if the user skipped the configuration wizard.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub skip_wizard: pulumi_wasm_rust::Output<Option<bool>>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct EmailRoutingSettingsResult {
    /// The date and time the settings have been created.
    pub created: pulumi_wasm_rust::Output<String>,
    /// State of the zone settings for Email Routing. **Modifying this attribute will force creation of a new resource.**
    pub enabled: pulumi_wasm_rust::Output<bool>,
    /// The date and time the settings have been modified.
    pub modified: pulumi_wasm_rust::Output<String>,
    /// Domain of your zone.
    pub name: pulumi_wasm_rust::Output<String>,
    /// Flag to check if the user skipped the configuration wizard.
    pub skip_wizard: pulumi_wasm_rust::Output<bool>,
    /// Show the state of your account, and the type or configuration error.
    pub status: pulumi_wasm_rust::Output<String>,
    /// Email Routing settings identifier.
    pub tag: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: EmailRoutingSettingsArgs) -> EmailRoutingSettingsResult {

    let result = crate::bindings::pulumi::cloudflare::email_routing_settings::invoke(name, &crate::bindings::pulumi::cloudflare::email_routing_settings::Args {
        enabled: &args.enabled.get_inner(),
        skip_wizard: &args.skip_wizard.get_inner(),
        zone_id: &args.zone_id.get_inner(),
    });

    EmailRoutingSettingsResult {
        created: crate::into_domain(result.created),
        enabled: crate::into_domain(result.enabled),
        modified: crate::into_domain(result.modified),
        name: crate::into_domain(result.name),
        skip_wizard: crate::into_domain(result.skip_wizard),
        status: crate::into_domain(result.status),
        tag: crate::into_domain(result.tag),
        zone_id: crate::into_domain(result.zone_id),
    }
}
