//! The [Cloudflare Managed Headers](https://developers.cloudflare.com/rules/transform/managed-transforms/)
//! allows you to add or remove some predefined headers to one's
//! requests or origin responses.
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as cloudflare from "@pulumi/cloudflare";
//! 
//! // Enable security headers using Managed Meaders
//! const example = new cloudflare.ManagedHeaders("example", {
//!     zoneId: "0da42c8d2132a9ddaf714f9e7c920711",
//!     managedRequestHeaders: [{
//!         id: "add_true_client_ip_headers",
//!         enabled: true,
//!     }],
//!     managedResponseHeaders: [{
//!         id: "remove_x-powered-by_header",
//!         enabled: true,
//!     }],
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//! 
//! # Enable security headers using Managed Meaders
//! example = cloudflare.ManagedHeaders("example",
//!     zone_id="0da42c8d2132a9ddaf714f9e7c920711",
//!     managed_request_headers=[{
//!         "id": "add_true_client_ip_headers",
//!         "enabled": True,
//!     }],
//!     managed_response_headers=[{
//!         "id": "remove_x-powered-by_header",
//!         "enabled": True,
//!     }])
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
//!     // Enable security headers using Managed Meaders
//!     var example = new Cloudflare.ManagedHeaders("example", new()
//!     {
//!         ZoneId = "0da42c8d2132a9ddaf714f9e7c920711",
//!         ManagedRequestHeaders = new[]
//!         {
//!             new Cloudflare.Inputs.ManagedHeadersManagedRequestHeaderArgs
//!             {
//!                 Id = "add_true_client_ip_headers",
//!                 Enabled = true,
//!             },
//!         },
//!         ManagedResponseHeaders = new[]
//!         {
//!             new Cloudflare.Inputs.ManagedHeadersManagedResponseHeaderArgs
//!             {
//!                 Id = "remove_x-powered-by_header",
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
//! 		// Enable security headers using Managed Meaders
//! 		_, err := cloudflare.NewManagedHeaders(ctx, "example", &cloudflare.ManagedHeadersArgs{
//! 			ZoneId: pulumi.String("0da42c8d2132a9ddaf714f9e7c920711"),
//! 			ManagedRequestHeaders: cloudflare.ManagedHeadersManagedRequestHeaderArray{
//! 				&cloudflare.ManagedHeadersManagedRequestHeaderArgs{
//! 					Id:      pulumi.String("add_true_client_ip_headers"),
//! 					Enabled: pulumi.Bool(true),
//! 				},
//! 			},
//! 			ManagedResponseHeaders: cloudflare.ManagedHeadersManagedResponseHeaderArray{
//! 				&cloudflare.ManagedHeadersManagedResponseHeaderArgs{
//! 					Id:      pulumi.String("remove_x-powered-by_header"),
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
//! import com.pulumi.cloudflare.ManagedHeaders;
//! import com.pulumi.cloudflare.ManagedHeadersArgs;
//! import com.pulumi.cloudflare.inputs.ManagedHeadersManagedRequestHeaderArgs;
//! import com.pulumi.cloudflare.inputs.ManagedHeadersManagedResponseHeaderArgs;
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
//!         // Enable security headers using Managed Meaders
//!         var example = new ManagedHeaders("example", ManagedHeadersArgs.builder()
//!             .zoneId("0da42c8d2132a9ddaf714f9e7c920711")
//!             .managedRequestHeaders(ManagedHeadersManagedRequestHeaderArgs.builder()
//!                 .id("add_true_client_ip_headers")
//!                 .enabled(true)
//!                 .build())
//!             .managedResponseHeaders(ManagedHeadersManagedResponseHeaderArgs.builder()
//!                 .id("remove_x-powered-by_header")
//!                 .enabled(true)
//!                 .build())
//!             .build());
//! 
//!     }
//! }
//! ```
//! ### YAML
//! ```yaml
//! resources:
//!   # Enable security headers using Managed Meaders
//!   example:
//!     type: cloudflare:ManagedHeaders
//!     properties:
//!       zoneId: 0da42c8d2132a9ddaf714f9e7c920711
//!       managedRequestHeaders:
//!         - id: add_true_client_ip_headers
//!           enabled: true
//!       managedResponseHeaders:
//!         - id: remove_x-powered-by_header
//!           enabled: true
//! ```
//! <!--End PulumiCodeChooser -->

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct ManagedHeadersArgs {
    /// The list of managed request headers.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub managed_request_headers: pulumi_wasm_rust::Output<Option<Vec<crate::types::ManagedHeadersManagedRequestHeader>>>,
    /// The list of managed response headers.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub managed_response_headers: pulumi_wasm_rust::Output<Option<Vec<crate::types::ManagedHeadersManagedResponseHeader>>>,
    /// The zone identifier to target for the resource.
    #[builder(into)]
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct ManagedHeadersResult {
    /// The list of managed request headers.
    pub managed_request_headers: pulumi_wasm_rust::Output<Option<Vec<crate::types::ManagedHeadersManagedRequestHeader>>>,
    /// The list of managed response headers.
    pub managed_response_headers: pulumi_wasm_rust::Output<Option<Vec<crate::types::ManagedHeadersManagedResponseHeader>>>,
    /// The zone identifier to target for the resource.
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: ManagedHeadersArgs) -> ManagedHeadersResult {

    let result = crate::bindings::pulumi::cloudflare::managed_headers::invoke(name, &crate::bindings::pulumi::cloudflare::managed_headers::Args {
        managed_request_headers: &args.managed_request_headers.get_inner(),
        managed_response_headers: &args.managed_response_headers.get_inner(),
        zone_id: &args.zone_id.get_inner(),
    });

    ManagedHeadersResult {
        managed_request_headers: crate::into_domain(result.managed_request_headers),
        managed_response_headers: crate::into_domain(result.managed_response_headers),
        zone_id: crate::into_domain(result.zone_id),
    }
}
