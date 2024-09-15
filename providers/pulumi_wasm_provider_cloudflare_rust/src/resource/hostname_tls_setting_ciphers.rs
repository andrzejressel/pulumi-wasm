//! Provides a Cloudflare per-hostname TLS setting resource, specifically for ciphers suites. Used to set ciphers suites for hostnames under the specified zone.
//!
//! ## Example Usage
//!
//! <!--Start PulumiCodeChooser -->
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as cloudflare from "@pulumi/cloudflare";
//!
//! const example = new cloudflare.HostnameTlsSettingCiphers("example", {
//!     hostname: "sub.example.com",
//!     values: ["ECDHE-RSA-AES128-GCM-SHA256"],
//!     zoneId: "0da42c8d2132a9ddaf714f9e7c920711",
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//!
//! example = cloudflare.HostnameTlsSettingCiphers("example",
//!     hostname="sub.example.com",
//!     values=["ECDHE-RSA-AES128-GCM-SHA256"],
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
//!     var example = new Cloudflare.HostnameTlsSettingCiphers("example", new()
//!     {
//!         Hostname = "sub.example.com",
//!         Values = new[]
//!         {
//!             "ECDHE-RSA-AES128-GCM-SHA256",
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
//! 		_, err := cloudflare.NewHostnameTlsSettingCiphers(ctx, "example", &cloudflare.HostnameTlsSettingCiphersArgs{
//! 			Hostname: pulumi.String("sub.example.com"),
//! 			Values: pulumi.StringArray{
//! 				pulumi.String("ECDHE-RSA-AES128-GCM-SHA256"),
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
//! import com.pulumi.cloudflare.HostnameTlsSettingCiphers;
//! import com.pulumi.cloudflare.HostnameTlsSettingCiphersArgs;
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
//!         var example = new HostnameTlsSettingCiphers("example", HostnameTlsSettingCiphersArgs.builder()        
//!             .hostname("sub.example.com")
//!             .values("ECDHE-RSA-AES128-GCM-SHA256")
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
//!     type: cloudflare:HostnameTlsSettingCiphers
//!     properties:
//!       hostname: sub.example.com
//!       values:
//!         - ECDHE-RSA-AES128-GCM-SHA256
//!       zoneId: 0da42c8d2132a9ddaf714f9e7c920711
//! ```
//! <!--End PulumiCodeChooser -->
//!
//! ## Import
//!
//! ```sh
//! $ pulumi import cloudflare:index/hostnameTlsSettingCiphers:HostnameTlsSettingCiphers example <zone_id>/<hostname>
//! ```
//!

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct HostnameTlsSettingCiphersArgs {
    /// Hostname that belongs to this zone name. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub hostname: pulumi_wasm_rust::Output<String>,
    /// Ports to use within the IP rule.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub ports: pulumi_wasm_rust::Output<Option<Vec<i32>>>,
    /// Ciphers suites value.
    #[builder(into)]
    pub values: pulumi_wasm_rust::Output<Vec<String>>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct HostnameTlsSettingCiphersResult {
    pub created_at: pulumi_wasm_rust::Output<String>,
    /// Hostname that belongs to this zone name. **Modifying this attribute will force creation of a new resource.**
    pub hostname: pulumi_wasm_rust::Output<String>,
    /// Ports to use within the IP rule.
    pub ports: pulumi_wasm_rust::Output<Option<Vec<i32>>>,
    pub updated_at: pulumi_wasm_rust::Output<String>,
    /// Ciphers suites value.
    pub values: pulumi_wasm_rust::Output<Vec<String>>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: HostnameTlsSettingCiphersArgs) -> HostnameTlsSettingCiphersResult {
    let result = crate::bindings::pulumi::cloudflare::hostname_tls_setting_ciphers::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::hostname_tls_setting_ciphers::Args {
            hostname: &args.hostname.get_inner(),
            ports: &args.ports.get_inner(),
            values: &args.values.get_inner(),
            zone_id: &args.zone_id.get_inner(),
        },
    );

    HostnameTlsSettingCiphersResult {
        created_at: crate::into_domain(result.created_at),
        hostname: crate::into_domain(result.hostname),
        ports: crate::into_domain(result.ports),
        updated_at: crate::into_domain(result.updated_at),
        values: crate::into_domain(result.values),
        zone_id: crate::into_domain(result.zone_id),
    }
}
