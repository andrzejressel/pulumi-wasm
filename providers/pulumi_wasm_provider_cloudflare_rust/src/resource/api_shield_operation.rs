//! Provides a resource to manage an operation in API Shield Endpoint Management.
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as cloudflare from "@pulumi/cloudflare";
//! 
//! const example = new cloudflare.ApiShieldOperation("example", {
//!     endpoint: "/path",
//!     host: "api.example.com",
//!     method: "GET",
//!     zoneId: "0da42c8d2132a9ddaf714f9e7c920711",
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//! 
//! example = cloudflare.ApiShieldOperation("example",
//!     endpoint="/path",
//!     host="api.example.com",
//!     method="GET",
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
//!     var example = new Cloudflare.ApiShieldOperation("example", new()
//!     {
//!         Endpoint = "/path",
//!         Host = "api.example.com",
//!         Method = "GET",
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
//! 		_, err := cloudflare.NewApiShieldOperation(ctx, "example", &cloudflare.ApiShieldOperationArgs{
//! 			Endpoint: pulumi.String("/path"),
//! 			Host:     pulumi.String("api.example.com"),
//! 			Method:   pulumi.String("GET"),
//! 			ZoneId:   pulumi.String("0da42c8d2132a9ddaf714f9e7c920711"),
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
//!         var example = new ApiShieldOperation("example", ApiShieldOperationArgs.builder()        
//!             .endpoint("/path")
//!             .host("api.example.com")
//!             .method("GET")
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
//!     type: cloudflare:ApiShieldOperation
//!     properties:
//!       endpoint: /path
//!       host: api.example.com
//!       method: GET
//!       zoneId: 0da42c8d2132a9ddaf714f9e7c920711
//! ```
//! <!--End PulumiCodeChooser -->

pub struct ApiShieldOperationArgs {
    /// The endpoint which can contain path parameter templates in curly braces, each will be replaced from left to right with `{varN}`, starting with `{var1}`. This will then be [Cloudflare-normalized](https://developers.cloudflare.com/rules/normalization/how-it-works/). **Modifying this attribute will force creation of a new resource.**
    pub endpoint: pulumi_wasm_rust::Output<String>,
    /// RFC3986-compliant host. **Modifying this attribute will force creation of a new resource.**
    pub host: pulumi_wasm_rust::Output<String>,
    /// The HTTP method used to access the endpoint. **Modifying this attribute will force creation of a new resource.**
    pub method: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct ApiShieldOperationResult {
    /// The endpoint which can contain path parameter templates in curly braces, each will be replaced from left to right with `{varN}`, starting with `{var1}`. This will then be [Cloudflare-normalized](https://developers.cloudflare.com/rules/normalization/how-it-works/). **Modifying this attribute will force creation of a new resource.**
    pub endpoint: pulumi_wasm_rust::Output<String>,
    /// RFC3986-compliant host. **Modifying this attribute will force creation of a new resource.**
    pub host: pulumi_wasm_rust::Output<String>,
    /// The HTTP method used to access the endpoint. **Modifying this attribute will force creation of a new resource.**
    pub method: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: ApiShieldOperationArgs) -> ApiShieldOperationResult {

    let result = crate::bindings::pulumi::cloudflare::api_shield_operation::invoke(name, &crate::bindings::pulumi::cloudflare::api_shield_operation::Args {
        endpoint: args.endpoint.get_inner(),
        host: args.host.get_inner(),
        method: args.method.get_inner(),
        zone_id: args.zone_id.get_inner(),
    });

    ApiShieldOperationResult {
        endpoint: crate::into_domain(result.endpoint),
        host: crate::into_domain(result.host),
        method: crate::into_domain(result.method),
        zone_id: crate::into_domain(result.zone_id),
    }
}
