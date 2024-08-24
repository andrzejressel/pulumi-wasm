//! Provides a Cloudflare custom hostname fallback origin resource.
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as cloudflare from "@pulumi/cloudflare";
//! 
//! const example = new cloudflare.CustomHostnameFallbackOrigin("example", {
//!     origin: "fallback.example.com",
//!     zoneId: "0da42c8d2132a9ddaf714f9e7c920711",
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//! 
//! example = cloudflare.CustomHostnameFallbackOrigin("example",
//!     origin="fallback.example.com",
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
//!     var example = new Cloudflare.CustomHostnameFallbackOrigin("example", new()
//!     {
//!         Origin = "fallback.example.com",
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
//! 		_, err := cloudflare.NewCustomHostnameFallbackOrigin(ctx, "example", &cloudflare.CustomHostnameFallbackOriginArgs{
//! 			Origin: pulumi.String("fallback.example.com"),
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
//! import com.pulumi.cloudflare.CustomHostnameFallbackOrigin;
//! import com.pulumi.cloudflare.CustomHostnameFallbackOriginArgs;
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
//!         var example = new CustomHostnameFallbackOrigin("example", CustomHostnameFallbackOriginArgs.builder()        
//!             .origin("fallback.example.com")
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
//!     type: cloudflare:CustomHostnameFallbackOrigin
//!     properties:
//!       origin: fallback.example.com
//!       zoneId: 0da42c8d2132a9ddaf714f9e7c920711
//! ```
//! <!--End PulumiCodeChooser -->
//! 
//! ## Import
//! 
//! ```sh
//! $ pulumi import cloudflare:index/customHostnameFallbackOrigin:CustomHostnameFallbackOrigin example <zone_id>/<fallback_hostname>
//! ```
//! 

pub struct CustomHostnameFallbackOriginArgs {
    /// Hostname you intend to fallback requests to. Origin must be a proxied A/AAAA/CNAME DNS record within Clouldflare.
    pub origin: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct CustomHostnameFallbackOriginResult {
    /// Hostname you intend to fallback requests to. Origin must be a proxied A/AAAA/CNAME DNS record within Clouldflare.
    pub origin: pulumi_wasm_rust::Output<String>,
    /// Status of the fallback origin's activation.
    pub status: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: CustomHostnameFallbackOriginArgs) -> CustomHostnameFallbackOriginResult {

    let result = crate::bindings::pulumi::cloudflare::custom_hostname_fallback_origin::invoke(name, &crate::bindings::pulumi::cloudflare::custom_hostname_fallback_origin::Args {
        origin: args.origin.get_inner(),
        zone_id: args.zone_id.get_inner(),
    });

    CustomHostnameFallbackOriginResult {
        origin: crate::into_domain(result.origin),
        status: crate::into_domain(result.status),
        zone_id: crate::into_domain(result.zone_id),
    }
}
