//! Tunnel exposes applications running on your local web server on any
//! network with an internet connection without manually adding DNS
//! records or configuring a firewall or router.
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as cloudflare from "@pulumi/cloudflare";
//! 
//! const example = new cloudflare.Tunnel("example", {
//!     accountId: "f037e56e89293a057740de681ac9abbe",
//!     name: "my-tunnel",
//!     secret: "AQIDBAUGBwgBAgMEBQYHCAECAwQFBgcIAQIDBAUGBwg=",
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//! 
//! example = cloudflare.Tunnel("example",
//!     account_id="f037e56e89293a057740de681ac9abbe",
//!     name="my-tunnel",
//!     secret="AQIDBAUGBwgBAgMEBQYHCAECAwQFBgcIAQIDBAUGBwg=")
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
//!     var example = new Cloudflare.Tunnel("example", new()
//!     {
//!         AccountId = "f037e56e89293a057740de681ac9abbe",
//!         Name = "my-tunnel",
//!         Secret = "AQIDBAUGBwgBAgMEBQYHCAECAwQFBgcIAQIDBAUGBwg=",
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
//! 		_, err := cloudflare.NewTunnel(ctx, "example", &cloudflare.TunnelArgs{
//! 			AccountId: pulumi.String("f037e56e89293a057740de681ac9abbe"),
//! 			Name:      pulumi.String("my-tunnel"),
//! 			Secret:    pulumi.String("AQIDBAUGBwgBAgMEBQYHCAECAwQFBgcIAQIDBAUGBwg="),
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
//! import com.pulumi.cloudflare.Tunnel;
//! import com.pulumi.cloudflare.TunnelArgs;
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
//!         var example = new Tunnel("example", TunnelArgs.builder()
//!             .accountId("f037e56e89293a057740de681ac9abbe")
//!             .name("my-tunnel")
//!             .secret("AQIDBAUGBwgBAgMEBQYHCAECAwQFBgcIAQIDBAUGBwg=")
//!             .build());
//! 
//!     }
//! }
//! ```
//! ### YAML
//! ```yaml
//! resources:
//!   example:
//!     type: cloudflare:Tunnel
//!     properties:
//!       accountId: f037e56e89293a057740de681ac9abbe
//!       name: my-tunnel
//!       secret: AQIDBAUGBwgBAgMEBQYHCAECAwQFBgcIAQIDBAUGBwg=
//! ```
//! <!--End PulumiCodeChooser -->
//! 
//! ## Import
//! 
//! ```sh
//! $ pulumi import cloudflare:index/tunnel:Tunnel example <account_id>/<tunnel_id>
//! ```
//! 

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct TunnelArgs {
    /// The account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// Indicates if this is a locally or remotely configured tunnel. If `local`, manage the tunnel using a YAML file on the origin machine. If `cloudflare`, manage the tunnel on the Zero Trust dashboard or using tunnel*config, tunnel*route or tunnel*virtual*network resources. Available values: `local`, `cloudflare`. **Modifying this attribute will force creation of a new resource.**
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub config_src: pulumi_wasm_rust::Output<Option<String>>,
    /// A user-friendly name chosen when the tunnel is created. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub name: pulumi_wasm_rust::Output<String>,
    /// 32 or more bytes, encoded as a base64 string. The Create Argo Tunnel endpoint sets this as the tunnel's password. Anyone wishing to run the tunnel needs this password. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub secret: pulumi_wasm_rust::Output<String>,
}

pub struct TunnelResult {
    /// The account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// Usable CNAME for accessing the Tunnel.
    pub cname: pulumi_wasm_rust::Output<String>,
    /// Indicates if this is a locally or remotely configured tunnel. If `local`, manage the tunnel using a YAML file on the origin machine. If `cloudflare`, manage the tunnel on the Zero Trust dashboard or using tunnel*config, tunnel*route or tunnel*virtual*network resources. Available values: `local`, `cloudflare`. **Modifying this attribute will force creation of a new resource.**
    pub config_src: pulumi_wasm_rust::Output<Option<String>>,
    /// A user-friendly name chosen when the tunnel is created. **Modifying this attribute will force creation of a new resource.**
    pub name: pulumi_wasm_rust::Output<String>,
    /// 32 or more bytes, encoded as a base64 string. The Create Argo Tunnel endpoint sets this as the tunnel's password. Anyone wishing to run the tunnel needs this password. **Modifying this attribute will force creation of a new resource.**
    pub secret: pulumi_wasm_rust::Output<String>,
    /// Token used by a connector to authenticate and run the tunnel.
    pub tunnel_token: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: TunnelArgs) -> TunnelResult {

    let result = crate::bindings::pulumi::cloudflare::tunnel::invoke(name, &crate::bindings::pulumi::cloudflare::tunnel::Args {
        account_id: &args.account_id.get_inner(),
        config_src: &args.config_src.get_inner(),
        name: &args.name.get_inner(),
        secret: &args.secret.get_inner(),
    });

    TunnelResult {
        account_id: crate::into_domain(result.account_id),
        cname: crate::into_domain(result.cname),
        config_src: crate::into_domain(result.config_src),
        name: crate::into_domain(result.name),
        secret: crate::into_domain(result.secret),
        tunnel_token: crate::into_domain(result.tunnel_token),
    }
}
