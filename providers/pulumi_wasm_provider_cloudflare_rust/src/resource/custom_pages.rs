//! Provides a resource which manages Cloudflare custom error pages.
//!
//! ## Example Usage
//!
//! <!--Start PulumiCodeChooser -->
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as cloudflare from "@pulumi/cloudflare";
//!
//! const example = new cloudflare.CustomPages("example", {
//!     state: "customized",
//!     type: "basic_challenge",
//!     url: "https://example.com/challenge.html",
//!     zoneId: "0da42c8d2132a9ddaf714f9e7c920711",
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//!
//! example = cloudflare.CustomPages("example",
//!     state="customized",
//!     type="basic_challenge",
//!     url="https://example.com/challenge.html",
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
//!     var example = new Cloudflare.CustomPages("example", new()
//!     {
//!         State = "customized",
//!         Type = "basic_challenge",
//!         Url = "https://example.com/challenge.html",
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
//! 		_, err := cloudflare.NewCustomPages(ctx, "example", &cloudflare.CustomPagesArgs{
//! 			State:  pulumi.String("customized"),
//! 			Type:   pulumi.String("basic_challenge"),
//! 			Url:    pulumi.String("https://example.com/challenge.html"),
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
//! import com.pulumi.cloudflare.CustomPages;
//! import com.pulumi.cloudflare.CustomPagesArgs;
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
//!         var example = new CustomPages("example", CustomPagesArgs.builder()        
//!             .state("customized")
//!             .type("basic_challenge")
//!             .url("https://example.com/challenge.html")
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
//!     type: cloudflare:CustomPages
//!     properties:
//!       state: customized
//!       type: basic_challenge
//!       url: https://example.com/challenge.html
//!       zoneId: 0da42c8d2132a9ddaf714f9e7c920711
//! ```
//! <!--End PulumiCodeChooser -->
//!
//! ## Import
//!
//! ```sh
//! $ pulumi import cloudflare:index/customPages:CustomPages example <resource_level>/<resource_id>/<custom_page_type>
//! ```
//!

pub struct CustomPagesArgs {
    /// The account identifier to target for the resource. Conflicts with `zone_id`.
    pub account_id: pulumi_wasm_rust::Output<Option<String>>,
    /// Managed state of the custom page. Available values: `default`, `customized`.
    pub state: pulumi_wasm_rust::Output<Option<String>>,
    /// The type of custom page you wish to update. Available values: `basic_challenge`, `waf_challenge`, `waf_block`, `ratelimit_block`, `country_challenge`, `ip_block`, `under_attack`, `500_errors`, `1000_errors`, `managed_challenge`.
    pub type_: pulumi_wasm_rust::Output<String>,
    /// URL of where the custom page source is located.
    pub url: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource. Conflicts with `account_id`.
    pub zone_id: pulumi_wasm_rust::Output<Option<String>>,
}

pub struct CustomPagesResult {
    /// The account identifier to target for the resource. Conflicts with `zone_id`.
    pub account_id: pulumi_wasm_rust::Output<Option<String>>,
    /// Managed state of the custom page. Available values: `default`, `customized`.
    pub state: pulumi_wasm_rust::Output<Option<String>>,
    /// The type of custom page you wish to update. Available values: `basic_challenge`, `waf_challenge`, `waf_block`, `ratelimit_block`, `country_challenge`, `ip_block`, `under_attack`, `500_errors`, `1000_errors`, `managed_challenge`.
    pub type_: pulumi_wasm_rust::Output<String>,
    /// URL of where the custom page source is located.
    pub url: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource. Conflicts with `account_id`.
    pub zone_id: pulumi_wasm_rust::Output<Option<String>>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: CustomPagesArgs) -> CustomPagesResult {
    let result = crate::bindings::pulumi::cloudflare::custom_pages::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::custom_pages::Args {
            account_id: &args.account_id.get_inner(),
            state: &args.state.get_inner(),
            type_: &args.type_.get_inner(),
            url: &args.url.get_inner(),
            zone_id: &args.zone_id.get_inner(),
        },
    );

    CustomPagesResult {
        account_id: crate::into_domain(result.account_id),
        state: crate::into_domain(result.state),
        type_: crate::into_domain(result.type_),
        url: crate::into_domain(result.url),
        zone_id: crate::into_domain(result.zone_id),
    }
}
