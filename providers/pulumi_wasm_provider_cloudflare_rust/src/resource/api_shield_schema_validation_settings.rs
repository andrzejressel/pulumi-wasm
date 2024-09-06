//! Provides a resource to manage settings in API Shield Schema Validation 2.0.
//!
//! ## Example Usage
//!
//! <!--Start PulumiCodeChooser -->
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as cloudflare from "@pulumi/cloudflare";
//!
//! const example = new cloudflare.ApiShieldSchemaValidationSettings("example", {
//!     validationDefaultMitigationAction: "log",
//!     validationOverrideMitigationAction: "none",
//!     zoneId: "0da42c8d2132a9ddaf714f9e7c920711",
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//!
//! example = cloudflare.ApiShieldSchemaValidationSettings("example",
//!     validation_default_mitigation_action="log",
//!     validation_override_mitigation_action="none",
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
//!     var example = new Cloudflare.ApiShieldSchemaValidationSettings("example", new()
//!     {
//!         ValidationDefaultMitigationAction = "log",
//!         ValidationOverrideMitigationAction = "none",
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
//! 		_, err := cloudflare.NewApiShieldSchemaValidationSettings(ctx, "example", &cloudflare.ApiShieldSchemaValidationSettingsArgs{
//! 			ValidationDefaultMitigationAction:  pulumi.String("log"),
//! 			ValidationOverrideMitigationAction: pulumi.String("none"),
//! 			ZoneId:                             pulumi.String("0da42c8d2132a9ddaf714f9e7c920711"),
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
//! import com.pulumi.cloudflare.ApiShieldSchemaValidationSettings;
//! import com.pulumi.cloudflare.ApiShieldSchemaValidationSettingsArgs;
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
//!         var example = new ApiShieldSchemaValidationSettings("example", ApiShieldSchemaValidationSettingsArgs.builder()        
//!             .validationDefaultMitigationAction("log")
//!             .validationOverrideMitigationAction("none")
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
//!     type: cloudflare:ApiShieldSchemaValidationSettings
//!     properties:
//!       validationDefaultMitigationAction: log
//!       validationOverrideMitigationAction: none
//!       zoneId: 0da42c8d2132a9ddaf714f9e7c920711
//! ```
//! <!--End PulumiCodeChooser -->

pub struct ApiShieldSchemaValidationSettingsArgs {
    /// The default mitigation action used when there is no mitigation action defined on the operation.
    pub validation_default_mitigation_action: pulumi_wasm_rust::Output<String>,
    /// When set, this overrides both zone level and operation level mitigation actions.
    pub validation_override_mitigation_action: pulumi_wasm_rust::Output<Option<String>>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct ApiShieldSchemaValidationSettingsResult {
    /// The default mitigation action used when there is no mitigation action defined on the operation.
    pub validation_default_mitigation_action: pulumi_wasm_rust::Output<String>,
    /// When set, this overrides both zone level and operation level mitigation actions.
    pub validation_override_mitigation_action: pulumi_wasm_rust::Output<Option<String>>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(
    name: &str,
    args: ApiShieldSchemaValidationSettingsArgs,
) -> ApiShieldSchemaValidationSettingsResult {
    let result = crate::bindings::pulumi::cloudflare::api_shield_schema_validation_settings::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::api_shield_schema_validation_settings::Args {
            validation_default_mitigation_action: &args
                .validation_default_mitigation_action
                .get_inner(),
            validation_override_mitigation_action: &args
                .validation_override_mitigation_action
                .get_inner(),
            zone_id: &args.zone_id.get_inner(),
        },
    );

    ApiShieldSchemaValidationSettingsResult {
        validation_default_mitigation_action: crate::into_domain(
            result.validation_default_mitigation_action,
        ),
        validation_override_mitigation_action: crate::into_domain(
            result.validation_override_mitigation_action,
        ),
        zone_id: crate::into_domain(result.zone_id),
    }
}
