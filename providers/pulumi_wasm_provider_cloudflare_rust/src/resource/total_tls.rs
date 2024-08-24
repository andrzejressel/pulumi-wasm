//! Provides a resource which manages Total TLS for a zone.
//!
//! ## Example Usage
//!
//! <!--Start PulumiCodeChooser -->
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as cloudflare from "@pulumi/cloudflare";
//!
//! const example = new cloudflare.TotalTls("example", {
//!     certificateAuthority: "lets_encrypt",
//!     enabled: true,
//!     zoneId: "0da42c8d2132a9ddaf714f9e7c920711",
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//!
//! example = cloudflare.TotalTls("example",
//!     certificate_authority="lets_encrypt",
//!     enabled=True,
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
//!     var example = new Cloudflare.TotalTls("example", new()
//!     {
//!         CertificateAuthority = "lets_encrypt",
//!         Enabled = true,
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
//! 		_, err := cloudflare.NewTotalTls(ctx, "example", &cloudflare.TotalTlsArgs{
//! 			CertificateAuthority: pulumi.String("lets_encrypt"),
//! 			Enabled:              pulumi.Bool(true),
//! 			ZoneId:               pulumi.String("0da42c8d2132a9ddaf714f9e7c920711"),
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
//! import com.pulumi.cloudflare.TotalTls;
//! import com.pulumi.cloudflare.TotalTlsArgs;
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
//!         var example = new TotalTls("example", TotalTlsArgs.builder()        
//!             .certificateAuthority("lets_encrypt")
//!             .enabled(true)
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
//!     type: cloudflare:TotalTls
//!     properties:
//!       certificateAuthority: lets_encrypt
//!       enabled: true
//!       zoneId: 0da42c8d2132a9ddaf714f9e7c920711
//! ```
//! <!--End PulumiCodeChooser -->
//!
//! ## Import
//!
//! ```sh
//! $ pulumi import cloudflare:index/totalTls:TotalTls example <zone_id>
//! ```
//!

pub struct TotalTlsArgs {
    /// The Certificate Authority that Total TLS certificates will be issued through. Available values: `google`, `lets_encrypt`.
    pub certificate_authority: pulumi_wasm_rust::Output<Option<String>>,
    /// Enable Total TLS for the zone.
    pub enabled: pulumi_wasm_rust::Output<bool>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct TotalTlsResult {
    /// The Certificate Authority that Total TLS certificates will be issued through. Available values: `google`, `lets_encrypt`.
    pub certificate_authority: pulumi_wasm_rust::Output<Option<String>>,
    /// Enable Total TLS for the zone.
    pub enabled: pulumi_wasm_rust::Output<bool>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: TotalTlsArgs) -> TotalTlsResult {
    let result = crate::bindings::pulumi::cloudflare::total_tls::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::total_tls::Args {
            certificate_authority: args.certificate_authority.get_inner(),
            enabled: args.enabled.get_inner(),
            zone_id: args.zone_id.get_inner(),
        },
    );

    TotalTlsResult {
        certificate_authority: crate::into_domain(result.certificate_authority),
        enabled: crate::into_domain(result.enabled),
        zone_id: crate::into_domain(result.zone_id),
    }
}
