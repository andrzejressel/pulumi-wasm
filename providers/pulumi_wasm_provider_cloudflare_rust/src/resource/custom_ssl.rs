//! Provides a Cloudflare custom SSL resource.
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as cloudflare from "@pulumi/cloudflare";
//! 
//! const example = new cloudflare.CustomSsl("example", {
//!     zoneId: "0da42c8d2132a9ddaf714f9e7c920711",
//!     customSslOptions: {
//!         certificate: "-----INSERT CERTIFICATE-----",
//!         privateKey: "-----INSERT PRIVATE KEY-----",
//!         bundleMethod: "ubiquitous",
//!         geoRestrictions: "us",
//!         type: "legacy_custom",
//!     },
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//! 
//! example = cloudflare.CustomSsl("example",
//!     zone_id="0da42c8d2132a9ddaf714f9e7c920711",
//!     custom_ssl_options={
//!         "certificate": "-----INSERT CERTIFICATE-----",
//!         "private_key": "-----INSERT PRIVATE KEY-----",
//!         "bundle_method": "ubiquitous",
//!         "geo_restrictions": "us",
//!         "type": "legacy_custom",
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
//!     var example = new Cloudflare.CustomSsl("example", new()
//!     {
//!         ZoneId = "0da42c8d2132a9ddaf714f9e7c920711",
//!         CustomSslOptions = new Cloudflare.Inputs.CustomSslCustomSslOptionsArgs
//!         {
//!             Certificate = "-----INSERT CERTIFICATE-----",
//!             PrivateKey = "-----INSERT PRIVATE KEY-----",
//!             BundleMethod = "ubiquitous",
//!             GeoRestrictions = "us",
//!             Type = "legacy_custom",
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
//! 		_, err := cloudflare.NewCustomSsl(ctx, "example", &cloudflare.CustomSslArgs{
//! 			ZoneId: pulumi.String("0da42c8d2132a9ddaf714f9e7c920711"),
//! 			CustomSslOptions: &cloudflare.CustomSslCustomSslOptionsArgs{
//! 				Certificate:     pulumi.String("-----INSERT CERTIFICATE-----"),
//! 				PrivateKey:      pulumi.String("-----INSERT PRIVATE KEY-----"),
//! 				BundleMethod:    pulumi.String("ubiquitous"),
//! 				GeoRestrictions: pulumi.String("us"),
//! 				Type:            pulumi.String("legacy_custom"),
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
//! import com.pulumi.cloudflare.CustomSsl;
//! import com.pulumi.cloudflare.CustomSslArgs;
//! import com.pulumi.cloudflare.inputs.CustomSslCustomSslOptionsArgs;
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
//!         var example = new CustomSsl("example", CustomSslArgs.builder()
//!             .zoneId("0da42c8d2132a9ddaf714f9e7c920711")
//!             .customSslOptions(CustomSslCustomSslOptionsArgs.builder()
//!                 .certificate("-----INSERT CERTIFICATE-----")
//!                 .privateKey("-----INSERT PRIVATE KEY-----")
//!                 .bundleMethod("ubiquitous")
//!                 .geoRestrictions("us")
//!                 .type("legacy_custom")
//!                 .build())
//!             .build());
//! 
//!     }
//! }
//! ```
//! ### YAML
//! ```yaml
//! resources:
//!   example:
//!     type: cloudflare:CustomSsl
//!     properties:
//!       zoneId: 0da42c8d2132a9ddaf714f9e7c920711
//!       customSslOptions:
//!         certificate: '-----INSERT CERTIFICATE-----'
//!         privateKey: '-----INSERT PRIVATE KEY-----'
//!         bundleMethod: ubiquitous
//!         geoRestrictions: us
//!         type: legacy_custom
//! ```
//! <!--End PulumiCodeChooser -->
//! 
//! ## Import
//! 
//! ```sh
//! $ pulumi import cloudflare:index/customSsl:CustomSsl example <zone_id>/<certificate_id>
//! ```
//! 

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct CustomSslArgs {
    /// The certificate associated parameters. **Modifying this attribute will force creation of a new resource.**
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub custom_ssl_options: pulumi_wasm_rust::Output<Option<crate::types::CustomSslCustomSslOptions>>,
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub custom_ssl_priorities: pulumi_wasm_rust::Output<Option<Vec<crate::types::CustomSslCustomSslPriority>>>,
    /// The zone identifier to target for the resource.
    #[builder(into)]
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct CustomSslResult {
    /// The certificate associated parameters. **Modifying this attribute will force creation of a new resource.**
    pub custom_ssl_options: pulumi_wasm_rust::Output<Option<crate::types::CustomSslCustomSslOptions>>,
    pub custom_ssl_priorities: pulumi_wasm_rust::Output<Option<Vec<crate::types::CustomSslCustomSslPriority>>>,
    pub expires_on: pulumi_wasm_rust::Output<String>,
    pub hosts: pulumi_wasm_rust::Output<Vec<String>>,
    pub issuer: pulumi_wasm_rust::Output<String>,
    pub modified_on: pulumi_wasm_rust::Output<String>,
    pub priority: pulumi_wasm_rust::Output<i32>,
    pub signature: pulumi_wasm_rust::Output<String>,
    pub status: pulumi_wasm_rust::Output<String>,
    pub uploaded_on: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource.
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: CustomSslArgs) -> CustomSslResult {

    let result = crate::bindings::pulumi::cloudflare::custom_ssl::invoke(name, &crate::bindings::pulumi::cloudflare::custom_ssl::Args {
        custom_ssl_options: &args.custom_ssl_options.get_inner(),
        custom_ssl_priorities: &args.custom_ssl_priorities.get_inner(),
        zone_id: &args.zone_id.get_inner(),
    });

    CustomSslResult {
        custom_ssl_options: crate::into_domain(result.custom_ssl_options),
        custom_ssl_priorities: crate::into_domain(result.custom_ssl_priorities),
        expires_on: crate::into_domain(result.expires_on),
        hosts: crate::into_domain(result.hosts),
        issuer: crate::into_domain(result.issuer),
        modified_on: crate::into_domain(result.modified_on),
        priority: crate::into_domain(result.priority),
        signature: crate::into_domain(result.signature),
        status: crate::into_domain(result.status),
        uploaded_on: crate::into_domain(result.uploaded_on),
        zone_id: crate::into_domain(result.zone_id),
    }
}
