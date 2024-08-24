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
//!     managedRequestHeaders: [{
//!         enabled: true,
//!         id: "add_true_client_ip_headers",
//!     }],
//!     managedResponseHeaders: [{
//!         enabled: true,
//!         id: "remove_x-powered-by_header",
//!     }],
//!     zoneId: "0da42c8d2132a9ddaf714f9e7c920711",
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//!
//! # Enable security headers using Managed Meaders
//! example = cloudflare.ManagedHeaders("example",
//!     managed_request_headers=[cloudflare.ManagedHeadersManagedRequestHeaderArgs(
//!         enabled=True,
//!         id="add_true_client_ip_headers",
//!     )],
//!     managed_response_headers=[cloudflare.ManagedHeadersManagedResponseHeaderArgs(
//!         enabled=True,
//!         id="remove_x-powered-by_header",
//!     )],
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
//!     // Enable security headers using Managed Meaders
//!     var example = new Cloudflare.ManagedHeaders("example", new()
//!     {
//!         ManagedRequestHeaders = new[]
//!         {
//!             new Cloudflare.Inputs.ManagedHeadersManagedRequestHeaderArgs
//!             {
//!                 Enabled = true,
//!                 Id = "add_true_client_ip_headers",
//!             },
//!         },
//!         ManagedResponseHeaders = new[]
//!         {
//!             new Cloudflare.Inputs.ManagedHeadersManagedResponseHeaderArgs
//!             {
//!                 Enabled = true,
//!                 Id = "remove_x-powered-by_header",
//!             },
//!         },
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
//! 		// Enable security headers using Managed Meaders
//! 		_, err := cloudflare.NewManagedHeaders(ctx, "example", &cloudflare.ManagedHeadersArgs{
//! 			ManagedRequestHeaders: cloudflare.ManagedHeadersManagedRequestHeaderArray{
//! 				&cloudflare.ManagedHeadersManagedRequestHeaderArgs{
//! 					Enabled: pulumi.Bool(true),
//! 					Id:      pulumi.String("add_true_client_ip_headers"),
//! 				},
//! 			},
//! 			ManagedResponseHeaders: cloudflare.ManagedHeadersManagedResponseHeaderArray{
//! 				&cloudflare.ManagedHeadersManagedResponseHeaderArgs{
//! 					Enabled: pulumi.Bool(true),
//! 					Id:      pulumi.String("remove_x-powered-by_header"),
//! 				},
//! 			},
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
//!             .managedRequestHeaders(ManagedHeadersManagedRequestHeaderArgs.builder()
//!                 .enabled(true)
//!                 .id("add_true_client_ip_headers")
//!                 .build())
//!             .managedResponseHeaders(ManagedHeadersManagedResponseHeaderArgs.builder()
//!                 .enabled(true)
//!                 .id("remove_x-powered-by_header")
//!                 .build())
//!             .zoneId("0da42c8d2132a9ddaf714f9e7c920711")
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
//!       managedRequestHeaders:
//!         - enabled: true
//!           id: add_true_client_ip_headers
//!       managedResponseHeaders:
//!         - enabled: true
//!           id: remove_x-powered-by_header
//!       zoneId: 0da42c8d2132a9ddaf714f9e7c920711
//! ```
//! <!--End PulumiCodeChooser -->

pub struct ManagedHeadersArgs {
    /// The list of managed request headers.
    pub managed_request_headers:
        pulumi_wasm_rust::Output<Option<Vec<crate::types::ManagedHeadersManagedRequestHeader>>>,
    /// The list of managed response headers.
    pub managed_response_headers:
        pulumi_wasm_rust::Output<Option<Vec<crate::types::ManagedHeadersManagedResponseHeader>>>,
    /// The zone identifier to target for the resource.
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct ManagedHeadersResult {
    /// The list of managed request headers.
    pub managed_request_headers:
        pulumi_wasm_rust::Output<Option<Vec<crate::types::ManagedHeadersManagedRequestHeader>>>,
    /// The list of managed response headers.
    pub managed_response_headers:
        pulumi_wasm_rust::Output<Option<Vec<crate::types::ManagedHeadersManagedResponseHeader>>>,
    /// The zone identifier to target for the resource.
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: ManagedHeadersArgs) -> ManagedHeadersResult {
    let result = crate::bindings::pulumi::cloudflare::managed_headers::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::managed_headers::Args {
            managed_request_headers: args.managed_request_headers.get_inner(),
            managed_response_headers: args.managed_response_headers.get_inner(),
            zone_id: args.zone_id.get_inner(),
        },
    );

    ManagedHeadersResult {
        managed_request_headers: crate::into_domain(result.managed_request_headers),
        managed_response_headers: crate::into_domain(result.managed_response_headers),
        zone_id: crate::into_domain(result.zone_id),
    }
}
