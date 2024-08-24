//! Provides a Cloudflare Teams Proxy Endpoint resource. Teams Proxy
//! Endpoints are used for pointing proxy clients at Cloudflare Secure
//! Gateway.
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as cloudflare from "@pulumi/cloudflare";
//! 
//! const example = new cloudflare.TeamsProxyEndpoint("example", {
//!     accountId: "f037e56e89293a057740de681ac9abbe",
//!     ips: ["192.0.2.0/24"],
//!     name: "office",
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//! 
//! example = cloudflare.TeamsProxyEndpoint("example",
//!     account_id="f037e56e89293a057740de681ac9abbe",
//!     ips=["192.0.2.0/24"],
//!     name="office")
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
//!     var example = new Cloudflare.TeamsProxyEndpoint("example", new()
//!     {
//!         AccountId = "f037e56e89293a057740de681ac9abbe",
//!         Ips = new[]
//!         {
//!             "192.0.2.0/24",
//!         },
//!         Name = "office",
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
//! 		_, err := cloudflare.NewTeamsProxyEndpoint(ctx, "example", &cloudflare.TeamsProxyEndpointArgs{
//! 			AccountId: pulumi.String("f037e56e89293a057740de681ac9abbe"),
//! 			Ips: pulumi.StringArray{
//! 				pulumi.String("192.0.2.0/24"),
//! 			},
//! 			Name: pulumi.String("office"),
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
//! import com.pulumi.cloudflare.TeamsProxyEndpoint;
//! import com.pulumi.cloudflare.TeamsProxyEndpointArgs;
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
//!         var example = new TeamsProxyEndpoint("example", TeamsProxyEndpointArgs.builder()        
//!             .accountId("f037e56e89293a057740de681ac9abbe")
//!             .ips("192.0.2.0/24")
//!             .name("office")
//!             .build());
//! 
//!     }
//! }
//! ```
//! ### YAML
//! ```yaml
//! resources:
//!   example:
//!     type: cloudflare:TeamsProxyEndpoint
//!     properties:
//!       accountId: f037e56e89293a057740de681ac9abbe
//!       ips:
//!         - 192.0.2.0/24
//!       name: office
//! ```
//! <!--End PulumiCodeChooser -->
//! 
//! ## Import
//! 
//! ```sh
//! $ pulumi import cloudflare:index/teamsProxyEndpoint:TeamsProxyEndpoint example <account_id>/<proxy_endpoint_id>
//! ```
//! 

pub struct TeamsProxyEndpointArgs {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The networks CIDRs that will be allowed to initiate proxy connections.
    pub ips: pulumi_wasm_rust::Output<Vec<String>>,
    /// Name of the teams proxy endpoint.
    pub name: pulumi_wasm_rust::Output<String>,
}

pub struct TeamsProxyEndpointResult {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The networks CIDRs that will be allowed to initiate proxy connections.
    pub ips: pulumi_wasm_rust::Output<Vec<String>>,
    /// Name of the teams proxy endpoint.
    pub name: pulumi_wasm_rust::Output<String>,
    /// The FQDN that proxy clients should be pointed at.
    pub subdomain: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: TeamsProxyEndpointArgs) -> TeamsProxyEndpointResult {

    let result = crate::bindings::pulumi::cloudflare::teams_proxy_endpoint::invoke(name, &crate::bindings::pulumi::cloudflare::teams_proxy_endpoint::Args {
        account_id: args.account_id.get_inner(),
        ips: args.ips.get_inner(),
        name: args.name.get_inner(),
    });

    TeamsProxyEndpointResult {
        account_id: crate::into_domain(result.account_id),
        ips: crate::into_domain(result.ips),
        name: crate::into_domain(result.name),
        subdomain: crate::into_domain(result.subdomain),
    }
}
