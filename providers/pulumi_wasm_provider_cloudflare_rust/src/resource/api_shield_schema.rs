//! Provides a resource to manage a schema in API Shield Schema Validation 2.0.
//!
//! ## Example Usage
//!
//! <!--Start PulumiCodeChooser -->
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as cloudflare from "@pulumi/cloudflare";
//! import * as fs from "fs";
//!
//! const petstoreSchema = new cloudflare.ApiShieldSchema("petstoreSchema", {
//!     zoneId: "0da42c8d2132a9ddaf714f9e7c920711",
//!     name: "myschema",
//!     kind: "openapi_v3",
//!     validationEnabled: true,
//!     source: fs.readFileSync("./schemas/petstore.json", "utf8"),
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//!
//! petstore_schema = cloudflare.ApiShieldSchema("petstoreSchema",
//!     zone_id="0da42c8d2132a9ddaf714f9e7c920711",
//!     name="myschema",
//!     kind="openapi_v3",
//!     validation_enabled=True,
//!     source=(lambda path: open(path).read())("./schemas/petstore.json"))
//! ```
//! ### C#
//! ```csharp
//! using System.Collections.Generic;
//! using System.IO;
//! using System.Linq;
//! using Pulumi;
//! using Cloudflare = Pulumi.Cloudflare;
//!
//! return await Deployment.RunAsync(() =>
//! {
//!     var petstoreSchema = new Cloudflare.ApiShieldSchema("petstoreSchema", new()
//!     {
//!         ZoneId = "0da42c8d2132a9ddaf714f9e7c920711",
//!         Name = "myschema",
//!         Kind = "openapi_v3",
//!         ValidationEnabled = true,
//!         Source = File.ReadAllText("./schemas/petstore.json"),
//!     });
//!
//! });
//! ```
//! ### Go
//! ```go
//! package main
//!
//! import (
//! 	"os"
//!
//! 	"github.com/pulumi/pulumi-cloudflare/sdk/v5/go/cloudflare"
//! 	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
//! )
//!
//! func readFileOrPanic(path string) pulumi.StringPtrInput {
//! 	data, err := os.ReadFile(path)
//! 	if err != nil {
//! 		panic(err.Error())
//! 	}
//! 	return pulumi.String(string(data))
//! }
//!
//! func main() {
//! 	pulumi.Run(func(ctx *pulumi.Context) error {
//! 		_, err := cloudflare.NewApiShieldSchema(ctx, "petstoreSchema", &cloudflare.ApiShieldSchemaArgs{
//! 			ZoneId:            pulumi.String("0da42c8d2132a9ddaf714f9e7c920711"),
//! 			Name:              pulumi.String("myschema"),
//! 			Kind:              pulumi.String("openapi_v3"),
//! 			ValidationEnabled: pulumi.Bool(true),
//! 			Source:            readFileOrPanic("./schemas/petstore.json"),
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
//!             .source(Files.readString(Paths.get("./schemas/petstore.json")))
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
//!     properties:
//!       zoneId: 0da42c8d2132a9ddaf714f9e7c920711
//!       name: myschema
//!       kind: openapi_v3
//!       # optional
//!       validationEnabled: true
//!       # optional, default false
//!       source:
//!         fn::readFile: ./schemas/petstore.json
//! ```
//! <!--End PulumiCodeChooser -->

pub struct ApiShieldSchemaArgs {
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
    let result = crate::bindings::pulumi::cloudflare::api_shield_schema::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::api_shield_schema::Args {
            kind: &args.kind.get_inner(),
            name: &args.name.get_inner(),
            source: &args.source.get_inner(),
            validation_enabled: &args.validation_enabled.get_inner(),
            zone_id: &args.zone_id.get_inner(),
        },
    );

    ApiShieldSchemaResult {
        kind: crate::into_domain(result.kind),
        name: crate::into_domain(result.name),
        source: crate::into_domain(result.source),
        validation_enabled: crate::into_domain(result.validation_enabled),
        zone_id: crate::into_domain(result.zone_id),
    }
}
