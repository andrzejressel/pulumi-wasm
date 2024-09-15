//! Provides a resource to manage operation-level settings in API Shield Schema Validation 2.0.
//!
//! ## Example Usage
//!
//! <!--Start PulumiCodeChooser -->
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as cloudflare from "@pulumi/cloudflare";
//!
//! const exampleApiShieldOperation = new cloudflare.ApiShieldOperation("exampleApiShieldOperation", {
//!     zoneId: "0da42c8d2132a9ddaf714f9e7c920711",
//!     method: "GET",
//!     host: "api.example.com",
//!     endpoint: "/path",
//! });
//! const exampleApiShieldOperationSchemaValidationSettings = new cloudflare.ApiShieldOperationSchemaValidationSettings("exampleApiShieldOperationSchemaValidationSettings", {
//!     zoneId: "0da42c8d2132a9ddaf714f9e7c920711",
//!     operationId: exampleApiShieldOperation.id,
//!     mitigationAction: "block",
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//!
//! example_api_shield_operation = cloudflare.ApiShieldOperation("exampleApiShieldOperation",
//!     zone_id="0da42c8d2132a9ddaf714f9e7c920711",
//!     method="GET",
//!     host="api.example.com",
//!     endpoint="/path")
//! example_api_shield_operation_schema_validation_settings = cloudflare.ApiShieldOperationSchemaValidationSettings("exampleApiShieldOperationSchemaValidationSettings",
//!     zone_id="0da42c8d2132a9ddaf714f9e7c920711",
//!     operation_id=example_api_shield_operation.id,
//!     mitigation_action="block")
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
//!     var exampleApiShieldOperation = new Cloudflare.ApiShieldOperation("exampleApiShieldOperation", new()
//!     {
//!         ZoneId = "0da42c8d2132a9ddaf714f9e7c920711",
//!         Method = "GET",
//!         Host = "api.example.com",
//!         Endpoint = "/path",
//!     });
//!
//!     var exampleApiShieldOperationSchemaValidationSettings = new Cloudflare.ApiShieldOperationSchemaValidationSettings("exampleApiShieldOperationSchemaValidationSettings", new()
//!     {
//!         ZoneId = "0da42c8d2132a9ddaf714f9e7c920711",
//!         OperationId = exampleApiShieldOperation.Id,
//!         MitigationAction = "block",
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
//! 		exampleApiShieldOperation, err := cloudflare.NewApiShieldOperation(ctx, "exampleApiShieldOperation", &cloudflare.ApiShieldOperationArgs{
//! 			ZoneId:   pulumi.String("0da42c8d2132a9ddaf714f9e7c920711"),
//! 			Method:   pulumi.String("GET"),
//! 			Host:     pulumi.String("api.example.com"),
//! 			Endpoint: pulumi.String("/path"),
//! 		})
//! 		if err != nil {
//! 			return err
//! 		}
//! 		_, err = cloudflare.NewApiShieldOperationSchemaValidationSettings(ctx, "exampleApiShieldOperationSchemaValidationSettings", &cloudflare.ApiShieldOperationSchemaValidationSettingsArgs{
//! 			ZoneId:           pulumi.String("0da42c8d2132a9ddaf714f9e7c920711"),
//! 			OperationId:      exampleApiShieldOperation.ID(),
//! 			MitigationAction: pulumi.String("block"),
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
//! import com.pulumi.cloudflare.ApiShieldOperation;
//! import com.pulumi.cloudflare.ApiShieldOperationArgs;
//! import com.pulumi.cloudflare.ApiShieldOperationSchemaValidationSettings;
//! import com.pulumi.cloudflare.ApiShieldOperationSchemaValidationSettingsArgs;
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
//!         var exampleApiShieldOperation = new ApiShieldOperation("exampleApiShieldOperation", ApiShieldOperationArgs.builder()        
//!             .zoneId("0da42c8d2132a9ddaf714f9e7c920711")
//!             .method("GET")
//!             .host("api.example.com")
//!             .endpoint("/path")
//!             .build());
//!
//!         var exampleApiShieldOperationSchemaValidationSettings = new ApiShieldOperationSchemaValidationSettings("exampleApiShieldOperationSchemaValidationSettings", ApiShieldOperationSchemaValidationSettingsArgs.builder()        
//!             .zoneId("0da42c8d2132a9ddaf714f9e7c920711")
//!             .operationId(exampleApiShieldOperation.id())
//!             .mitigationAction("block")
//!             .build());
//!
//!     }
//! }
//! ```
//! ### YAML
//! ```yaml
//! resources:
//!   exampleApiShieldOperation:
//!     type: cloudflare:ApiShieldOperation
//!     properties:
//!       zoneId: 0da42c8d2132a9ddaf714f9e7c920711
//!       method: GET
//!       host: api.example.com
//!       endpoint: /path
//!   exampleApiShieldOperationSchemaValidationSettings:
//!     type: cloudflare:ApiShieldOperationSchemaValidationSettings
//!     properties:
//!       zoneId: 0da42c8d2132a9ddaf714f9e7c920711
//!       operationId: ${exampleApiShieldOperation.id}
//!       mitigationAction: block
//! ```
//! <!--End PulumiCodeChooser -->

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct ApiShieldOperationSchemaValidationSettingsArgs {
    /// The mitigation action to apply to this operation.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub mitigation_action: pulumi_wasm_rust::Output<Option<String>>,
    /// Operation ID these settings should apply to. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub operation_id: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct ApiShieldOperationSchemaValidationSettingsResult {
    /// The mitigation action to apply to this operation.
    pub mitigation_action: pulumi_wasm_rust::Output<Option<String>>,
    /// Operation ID these settings should apply to. **Modifying this attribute will force creation of a new resource.**
    pub operation_id: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(
    name: &str,
    args: ApiShieldOperationSchemaValidationSettingsArgs,
) -> ApiShieldOperationSchemaValidationSettingsResult {
    let result = crate::bindings::pulumi::cloudflare::api_shield_operation_schema_validation_settings::invoke(name, &crate::bindings::pulumi::cloudflare::api_shield_operation_schema_validation_settings::Args {
        mitigation_action: &args.mitigation_action.get_inner(),
        operation_id: &args.operation_id.get_inner(),
        zone_id: &args.zone_id.get_inner(),
    });

    ApiShieldOperationSchemaValidationSettingsResult {
        mitigation_action: crate::into_domain(result.mitigation_action),
        operation_id: crate::into_domain(result.operation_id),
        zone_id: crate::into_domain(result.zone_id),
    }
}
