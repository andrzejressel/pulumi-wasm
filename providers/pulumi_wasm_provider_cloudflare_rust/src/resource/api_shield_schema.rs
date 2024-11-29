//! Provides a resource to manage a schema in API Shield Schema Validation 2.0.
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as cloudflare from "@pulumi/cloudflare";
//! import * as std from "@pulumi/std";
//! 
//! const petstoreSchema = new cloudflare.ApiShieldSchema("petstore_schema", {
//!     zoneId: "0da42c8d2132a9ddaf714f9e7c920711",
//!     name: "myschema",
//!     kind: "openapi_v3",
//!     validationEnabled: true,
//!     source: std.file({
//!         input: "./schemas/petstore.json",
//!     }).then(invoke => invoke.result),
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//! import pulumi_std as std
//! 
//! petstore_schema = cloudflare.ApiShieldSchema("petstore_schema",
//!     zone_id="0da42c8d2132a9ddaf714f9e7c920711",
//!     name="myschema",
//!     kind="openapi_v3",
//!     validation_enabled=True,
//!     source=std.file(input="./schemas/petstore.json").result)
//! ```
//! ### C#
//! ```csharp
//! using System.Collections.Generic;
//! using System.Linq;
//! using Pulumi;
//! using Cloudflare = Pulumi.Cloudflare;
//! using Std = Pulumi.Std;
//! 
//! return await Deployment.RunAsync(() => 
//! {
//!     var petstoreSchema = new Cloudflare.ApiShieldSchema("petstore_schema", new()
//!     {
//!         ZoneId = "0da42c8d2132a9ddaf714f9e7c920711",
//!         Name = "myschema",
//!         Kind = "openapi_v3",
//!         ValidationEnabled = true,
//!         Source = Std.File.Invoke(new()
//!         {
//!             Input = "./schemas/petstore.json",
//!         }).Apply(invoke => invoke.Result),
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
//! 	"github.com/pulumi/pulumi-std/sdk/go/std"
//! 	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
//! )
//! 
//! func main() {
//! 	pulumi.Run(func(ctx *pulumi.Context) error {
//! 		invokeFile, err := std.File(ctx, &std.FileArgs{
//! 			Input: "./schemas/petstore.json",
//! 		}, nil)
//! 		if err != nil {
//! 			return err
//! 		}
//! 		_, err = cloudflare.NewApiShieldSchema(ctx, "petstore_schema", &cloudflare.ApiShieldSchemaArgs{
//! 			ZoneId:            pulumi.String("0da42c8d2132a9ddaf714f9e7c920711"),
//! 			Name:              pulumi.String("myschema"),
//! 			Kind:              pulumi.String("openapi_v3"),
//! 			ValidationEnabled: pulumi.Bool(true),
//! 			Source:            pulumi.String(invokeFile.Result),
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
//! import com.pulumi.cloudflare.ApiShieldSchema;
//! import com.pulumi.cloudflare.ApiShieldSchemaArgs;
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
//!         var petstoreSchema = new ApiShieldSchema("petstoreSchema", ApiShieldSchemaArgs.builder()
//!             .zoneId("0da42c8d2132a9ddaf714f9e7c920711")
//!             .name("myschema")
//!             .kind("openapi_v3")
//!             .validationEnabled(true)
//!             .source(StdFunctions.file(FileArgs.builder()
//!                 .input("./schemas/petstore.json")
//!                 .build()).result())
//!             .build());
//! 
//!     }
//! }
//! ```
//! ### YAML
//! ```yaml
//! resources:
//!   petstoreSchema:
//!     type: cloudflare:ApiShieldSchema
//!     name: petstore_schema
//!     properties:
//!       zoneId: 0da42c8d2132a9ddaf714f9e7c920711
//!       name: myschema
//!       kind: openapi_v3
//!       validationEnabled: true # optional, default false
//!       source:
//!         fn::invoke:
//!           Function: std:file
//!           Arguments:
//!             input: ./schemas/petstore.json
//!           Return: result
//! ```
//! <!--End PulumiCodeChooser -->

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct ApiShieldSchemaArgs {
    /// Kind of schema. Defaults to `openapi_v3`. **Modifying this attribute will force creation of a new resource.**
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub kind: pulumi_wasm_rust::Output<Option<String>>,
    /// Name of the schema. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub name: pulumi_wasm_rust::Output<String>,
    /// Schema file bytes. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub source: pulumi_wasm_rust::Output<String>,
    /// Flag whether schema is enabled for validation.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub validation_enabled: pulumi_wasm_rust::Output<Option<bool>>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct ApiShieldSchemaResult {
    /// Kind of schema. Defaults to `openapi_v3`. **Modifying this attribute will force creation of a new resource.**
    pub kind: pulumi_wasm_rust::Output<Option<String>>,
    /// Name of the schema. **Modifying this attribute will force creation of a new resource.**
    pub name: pulumi_wasm_rust::Output<String>,
    /// Schema file bytes. **Modifying this attribute will force creation of a new resource.**
    pub source: pulumi_wasm_rust::Output<String>,
    /// Flag whether schema is enabled for validation.
    pub validation_enabled: pulumi_wasm_rust::Output<Option<bool>>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: ApiShieldSchemaArgs) -> ApiShieldSchemaResult {

    let result = crate::bindings::pulumi::cloudflare::api_shield_schema::invoke(name, &crate::bindings::pulumi::cloudflare::api_shield_schema::Args {
        kind: &args.kind.get_inner(),
        name: &args.name.get_inner(),
        source: &args.source.get_inner(),
        validation_enabled: &args.validation_enabled.get_inner(),
        zone_id: &args.zone_id.get_inner(),
    });

    ApiShieldSchemaResult {
        kind: crate::into_domain(result.kind),
        name: crate::into_domain(result.name),
        source: crate::into_domain(result.source),
        validation_enabled: crate::into_domain(result.validation_enabled),
        zone_id: crate::into_domain(result.zone_id),
    }
}
