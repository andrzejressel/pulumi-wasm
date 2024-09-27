//! Use this datasource to lookup a tunnel in an account.
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as cloudflare from "@pulumi/cloudflare";
//! 
//! const example = cloudflare.getTunnel({
//!     accountId: "f037e56e89293a057740de681ac9abbe",
//!     name: "my-tunnel",
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//! 
//! example = cloudflare.get_tunnel(account_id="f037e56e89293a057740de681ac9abbe",
//!     name="my-tunnel")
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
//!     var example = Cloudflare.GetTunnel.Invoke(new()
//!     {
//!         AccountId = "f037e56e89293a057740de681ac9abbe",
//!         Name = "my-tunnel",
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
//! 		_, err := cloudflare.LookupTunnel(ctx, &cloudflare.LookupTunnelArgs{
//! 			AccountId: "f037e56e89293a057740de681ac9abbe",
//! 			Name:      "my-tunnel",
//! 		}, nil)
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
//! import com.pulumi.cloudflare.CloudflareFunctions;
//! import com.pulumi.cloudflare.inputs.GetTunnelArgs;
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
//!         final var example = CloudflareFunctions.getTunnel(GetTunnelArgs.builder()
//!             .accountId("f037e56e89293a057740de681ac9abbe")
//!             .name("my-tunnel")
//!             .build());
//! 
//!     }
//! }
//! ```
//! ### YAML
//! ```yaml
//! variables:
//!   example:
//!     fn::invoke:
//!       Function: cloudflare:getTunnel
//!       Arguments:
//!         accountId: f037e56e89293a057740de681ac9abbe
//!         name: my-tunnel
//! ```
//! <!--End PulumiCodeChooser -->

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct GetTunnelArgs {
    /// The account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// Name of the tunnel. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub name: pulumi_wasm_rust::Output<String>,
}

pub struct GetTunnelResult {
    /// The account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// ID of the tunnel.
    pub id: pulumi_wasm_rust::Output<String>,
    /// Name of the tunnel. **Modifying this attribute will force creation of a new resource.**
    pub name: pulumi_wasm_rust::Output<String>,
    /// Whether the tunnel can be configured remotely from the Zero Trust dashboard.
    pub remote_config: pulumi_wasm_rust::Output<bool>,
    /// The status of the tunnel. Available values: `inactive`, `degraded`, `healthy`, `down`.
    pub status: pulumi_wasm_rust::Output<String>,
    /// The type of the tunnel. Available values: `cfd_tunnel`, `warp_connector`.
    pub tunnel_type: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn invoke(args: GetTunnelArgs) -> GetTunnelResult {

    let result = crate::bindings::pulumi::cloudflare::get_tunnel::invoke(&crate::bindings::pulumi::cloudflare::get_tunnel::Args {
        account_id: &args.account_id.get_inner(),
        name: &args.name.get_inner(),
    });

    GetTunnelResult {
        account_id: crate::into_domain(result.account_id),
        id: crate::into_domain(result.id),
        name: crate::into_domain(result.name),
        remote_config: crate::into_domain(result.remote_config),
        status: crate::into_domain(result.status),
        tunnel_type: crate::into_domain(result.tunnel_type),
    }
}
