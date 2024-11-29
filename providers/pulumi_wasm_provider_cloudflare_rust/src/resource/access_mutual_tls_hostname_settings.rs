//! Provides a Cloudflare Access Mutual TLS Certificate Settings resource.
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as cloudflare from "@pulumi/cloudflare";
//! 
//! const example = new cloudflare.AccessMutualTlsHostnameSettings("example", {
//!     zoneId: "0da42c8d2132a9ddaf714f9e7c920711",
//!     settings: [{
//!         hostname: "example.com",
//!         clientCertificateForwarding: true,
//!         chinaNetwork: false,
//!     }],
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//! 
//! example = cloudflare.AccessMutualTlsHostnameSettings("example",
//!     zone_id="0da42c8d2132a9ddaf714f9e7c920711",
//!     settings=[{
//!         "hostname": "example.com",
//!         "client_certificate_forwarding": True,
//!         "china_network": False,
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
//!     var example = new Cloudflare.AccessMutualTlsHostnameSettings("example", new()
//!     {
//!         ZoneId = "0da42c8d2132a9ddaf714f9e7c920711",
//!         Settings = new[]
//!         {
//!             new Cloudflare.Inputs.AccessMutualTlsHostnameSettingsSettingArgs
//!             {
//!                 Hostname = "example.com",
//!                 ClientCertificateForwarding = true,
//!                 ChinaNetwork = false,
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
//! 		_, err := cloudflare.NewAccessMutualTlsHostnameSettings(ctx, "example", &cloudflare.AccessMutualTlsHostnameSettingsArgs{
//! 			ZoneId: pulumi.String("0da42c8d2132a9ddaf714f9e7c920711"),
//! 			Settings: cloudflare.AccessMutualTlsHostnameSettingsSettingArray{
//! 				&cloudflare.AccessMutualTlsHostnameSettingsSettingArgs{
//! 					Hostname:                    pulumi.String("example.com"),
//! 					ClientCertificateForwarding: pulumi.Bool(true),
//! 					ChinaNetwork:                pulumi.Bool(false),
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
//! import com.pulumi.cloudflare.AccessMutualTlsHostnameSettings;
//! import com.pulumi.cloudflare.AccessMutualTlsHostnameSettingsArgs;
//! import com.pulumi.cloudflare.inputs.AccessMutualTlsHostnameSettingsSettingArgs;
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
//!         var example = new AccessMutualTlsHostnameSettings("example", AccessMutualTlsHostnameSettingsArgs.builder()
//!             .zoneId("0da42c8d2132a9ddaf714f9e7c920711")
//!             .settings(AccessMutualTlsHostnameSettingsSettingArgs.builder()
//!                 .hostname("example.com")
//!                 .clientCertificateForwarding(true)
//!                 .chinaNetwork(false)
//!                 .build())
//!             .build());
//! 
//!     }
//! }
//! ```
//! ### YAML
//! ```yaml
//! resources:
//!   example:
//!     type: cloudflare:AccessMutualTlsHostnameSettings
//!     properties:
//!       zoneId: 0da42c8d2132a9ddaf714f9e7c920711
//!       settings:
//!         - hostname: example.com
//!           clientCertificateForwarding: true
//!           chinaNetwork: false
//! ```
//! <!--End PulumiCodeChooser -->
//! 
//! ## Import
//! 
//! Account level mTLS hostname settings import.
//! 
//! ```sh
//! $ pulumi import cloudflare:index/accessMutualTlsHostnameSettings:AccessMutualTlsHostnameSettings example account/<account_id>
//! ```
//! 
//! Zone level mTLS hostname settings import.
//! 
//! ```sh
//! $ pulumi import cloudflare:index/accessMutualTlsHostnameSettings:AccessMutualTlsHostnameSettings example zone/<zone_id>
//! ```
//! 

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct AccessMutualTlsHostnameSettingsArgs {
    /// The account identifier to target for the resource.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub account_id: pulumi_wasm_rust::Output<Option<String>>,
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub settings: pulumi_wasm_rust::Output<Option<Vec<crate::types::AccessMutualTlsHostnameSettingsSetting>>>,
    /// The zone identifier to target for the resource.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub zone_id: pulumi_wasm_rust::Output<Option<String>>,
}

pub struct AccessMutualTlsHostnameSettingsResult {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<Option<String>>,
    pub settings: pulumi_wasm_rust::Output<Option<Vec<crate::types::AccessMutualTlsHostnameSettingsSetting>>>,
    /// The zone identifier to target for the resource.
    pub zone_id: pulumi_wasm_rust::Output<Option<String>>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: AccessMutualTlsHostnameSettingsArgs) -> AccessMutualTlsHostnameSettingsResult {

    let result = crate::bindings::pulumi::cloudflare::access_mutual_tls_hostname_settings::invoke(name, &crate::bindings::pulumi::cloudflare::access_mutual_tls_hostname_settings::Args {
        account_id: &args.account_id.get_inner(),
        settings: &args.settings.get_inner(),
        zone_id: &args.zone_id.get_inner(),
    });

    AccessMutualTlsHostnameSettingsResult {
        account_id: crate::into_domain(result.account_id),
        settings: crate::into_domain(result.settings),
        zone_id: crate::into_domain(result.zone_id),
    }
}
