//! Provides a resource to customize the pages your end users will see
//! when trying to reach applications behind Cloudflare Access.
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as cloudflare from "@pulumi/cloudflare";
//! 
//! const example = new cloudflare.AccessCustomPage("example", {
//!     customHtml: "<html><body><h1>Forbidden</h1></body></html>",
//!     name: "example",
//!     type: "forbidden",
//!     zoneId: "0da42c8d2132a9ddaf714f9e7c920711",
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//! 
//! example = cloudflare.AccessCustomPage("example",
//!     custom_html="<html><body><h1>Forbidden</h1></body></html>",
//!     name="example",
//!     type="forbidden",
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
//!     var example = new Cloudflare.AccessCustomPage("example", new()
//!     {
//!         CustomHtml = "<html><body><h1>Forbidden</h1></body></html>",
//!         Name = "example",
//!         Type = "forbidden",
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
//! 		_, err := cloudflare.NewAccessCustomPage(ctx, "example", &cloudflare.AccessCustomPageArgs{
//! 			CustomHtml: pulumi.String("<html><body><h1>Forbidden</h1></body></html>"),
//! 			Name:       pulumi.String("example"),
//! 			Type:       pulumi.String("forbidden"),
//! 			ZoneId:     pulumi.String("0da42c8d2132a9ddaf714f9e7c920711"),
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
//! import com.pulumi.cloudflare.AccessCustomPage;
//! import com.pulumi.cloudflare.AccessCustomPageArgs;
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
//!         var example = new AccessCustomPage("example", AccessCustomPageArgs.builder()        
//!             .customHtml("<html><body><h1>Forbidden</h1></body></html>")
//!             .name("example")
//!             .type("forbidden")
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
//!     type: cloudflare:AccessCustomPage
//!     properties:
//!       customHtml: <html><body><h1>Forbidden</h1></body></html>
//!       name: example
//!       type: forbidden
//!       zoneId: 0da42c8d2132a9ddaf714f9e7c920711
//! ```
//! <!--End PulumiCodeChooser -->

pub struct AccessCustomPageArgs {
    /// The account identifier to target for the resource. Conflicts with `zone_id`. **Modifying this attribute will force creation of a new resource.**
    pub account_id: pulumi_wasm_rust::Output<Option<String>>,
    /// Number of apps to display on the custom page.
    pub app_count: pulumi_wasm_rust::Output<Option<i32>>,
    /// Custom HTML to display on the custom page.
    pub custom_html: pulumi_wasm_rust::Output<Option<String>>,
    /// Friendly name of the Access Custom Page configuration.
    pub name: pulumi_wasm_rust::Output<String>,
    /// Type of Access custom page to create. Available values: `identity_denied`, `forbidden`.
    pub type_: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource. Conflicts with `account_id`. **Modifying this attribute will force creation of a new resource.**
    pub zone_id: pulumi_wasm_rust::Output<Option<String>>,
}

pub struct AccessCustomPageResult {
    /// The account identifier to target for the resource. Conflicts with `zone_id`. **Modifying this attribute will force creation of a new resource.**
    pub account_id: pulumi_wasm_rust::Output<Option<String>>,
    /// Number of apps to display on the custom page.
    pub app_count: pulumi_wasm_rust::Output<Option<i32>>,
    /// Custom HTML to display on the custom page.
    pub custom_html: pulumi_wasm_rust::Output<Option<String>>,
    /// Friendly name of the Access Custom Page configuration.
    pub name: pulumi_wasm_rust::Output<String>,
    /// Type of Access custom page to create. Available values: `identity_denied`, `forbidden`.
    pub type_: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource. Conflicts with `account_id`. **Modifying this attribute will force creation of a new resource.**
    pub zone_id: pulumi_wasm_rust::Output<Option<String>>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: AccessCustomPageArgs) -> AccessCustomPageResult {

    let result = crate::bindings::pulumi::cloudflare::access_custom_page::invoke(name, &crate::bindings::pulumi::cloudflare::access_custom_page::Args {
        account_id: args.account_id.get_inner(),
        app_count: args.app_count.get_inner(),
        custom_html: args.custom_html.get_inner(),
        name: args.name.get_inner(),
        type_: args.type_.get_inner(),
        zone_id: args.zone_id.get_inner(),
    });

    AccessCustomPageResult {
        account_id: crate::into_domain(result.account_id),
        app_count: crate::into_domain(result.app_count),
        custom_html: crate::into_domain(result.custom_html),
        name: crate::into_domain(result.name),
        type_: crate::into_domain(result.type_),
        zone_id: crate::into_domain(result.zone_id),
    }
}
