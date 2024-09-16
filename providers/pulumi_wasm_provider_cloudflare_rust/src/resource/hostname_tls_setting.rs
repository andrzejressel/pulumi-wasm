//! Provides a Cloudflare per-hostname TLS setting resource. Used to set TLS settings for hostnames under the specified zone.
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as cloudflare from "@pulumi/cloudflare";
//! 
//! const example = new cloudflare.HostnameTlsSetting("example", {
//!     hostname: "sub.example.com",
//!     setting: "min_tls_version",
//!     value: "1.2",
//!     zoneId: "0da42c8d2132a9ddaf714f9e7c920711",
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//! 
//! example = cloudflare.HostnameTlsSetting("example",
//!     hostname="sub.example.com",
//!     setting="min_tls_version",
//!     value="1.2",
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
//!     var example = new Cloudflare.HostnameTlsSetting("example", new()
//!     {
//!         Hostname = "sub.example.com",
//!         Setting = "min_tls_version",
//!         Value = "1.2",
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
//! 		_, err := cloudflare.NewHostnameTlsSetting(ctx, "example", &cloudflare.HostnameTlsSettingArgs{
//! 			Hostname: pulumi.String("sub.example.com"),
//! 			Setting:  pulumi.String("min_tls_version"),
//! 			Value:    pulumi.String("1.2"),
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
//! import com.pulumi.cloudflare.HostnameTlsSetting;
//! import com.pulumi.cloudflare.HostnameTlsSettingArgs;
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
//!         var example = new HostnameTlsSetting("example", HostnameTlsSettingArgs.builder()        
//!             .hostname("sub.example.com")
//!             .setting("min_tls_version")
//!             .value("1.2")
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
//!     type: cloudflare:HostnameTlsSetting
//!     properties:
//!       hostname: sub.example.com
//!       setting: min_tls_version
//!       value: '1.2'
//!       zoneId: 0da42c8d2132a9ddaf714f9e7c920711
//! ```
//! <!--End PulumiCodeChooser -->
//! 
//! ## Import
//! 
//! ```sh
//! $ pulumi import cloudflare:index/hostnameTlsSetting:HostnameTlsSetting example <zone_id>/<hostname>/<setting_name>
//! ```
//! 

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct HostnameTlsSettingArgs {
    /// Hostname that belongs to this zone name. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub hostname: pulumi_wasm_rust::Output<String>,
    /// TLS setting name. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub setting: pulumi_wasm_rust::Output<String>,
    /// TLS setting value.
    #[builder(into)]
    pub value: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct HostnameTlsSettingResult {
    pub created_at: pulumi_wasm_rust::Output<String>,
    /// Hostname that belongs to this zone name. **Modifying this attribute will force creation of a new resource.**
    pub hostname: pulumi_wasm_rust::Output<String>,
    /// TLS setting name. **Modifying this attribute will force creation of a new resource.**
    pub setting: pulumi_wasm_rust::Output<String>,
    pub updated_at: pulumi_wasm_rust::Output<String>,
    /// TLS setting value.
    pub value: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: HostnameTlsSettingArgs) -> HostnameTlsSettingResult {

    let result = crate::bindings::pulumi::cloudflare::hostname_tls_setting::invoke(name, &crate::bindings::pulumi::cloudflare::hostname_tls_setting::Args {
        hostname: &args.hostname.get_inner(),
        setting: &args.setting.get_inner(),
        value: &args.value.get_inner(),
        zone_id: &args.zone_id.get_inner(),
    });

    HostnameTlsSettingResult {
        created_at: crate::into_domain(result.created_at),
        hostname: crate::into_domain(result.hostname),
        setting: crate::into_domain(result.setting),
        updated_at: crate::into_domain(result.updated_at),
        value: crate::into_domain(result.value),
        zone_id: crate::into_domain(result.zone_id),
    }
}
