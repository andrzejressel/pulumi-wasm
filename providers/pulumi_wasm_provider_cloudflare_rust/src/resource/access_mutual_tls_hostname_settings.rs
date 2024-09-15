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
//!     settings: [{
//!         chinaNetwork: false,
//!         clientCertificateForwarding: true,
//!         hostname: "example.com",
//!     }],
//!     zoneId: "0da42c8d2132a9ddaf714f9e7c920711",
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//!
//! example = cloudflare.AccessMutualTlsHostnameSettings("example",
//!     settings=[cloudflare.AccessMutualTlsHostnameSettingsSettingArgs(
//!         china_network=False,
//!         client_certificate_forwarding=True,
//!         hostname="example.com",
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
//!     var example = new Cloudflare.AccessMutualTlsHostnameSettings("example", new()
//!     {
//!         Settings = new[]
//!         {
//!             new Cloudflare.Inputs.AccessMutualTlsHostnameSettingsSettingArgs
//!             {
//!                 ChinaNetwork = false,
//!                 ClientCertificateForwarding = true,
//!                 Hostname = "example.com",
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
//! 		_, err := cloudflare.NewAccessMutualTlsHostnameSettings(ctx, "example", &cloudflare.AccessMutualTlsHostnameSettingsArgs{
//! 			Settings: cloudflare.AccessMutualTlsHostnameSettingsSettingArray{
//! 				&cloudflare.AccessMutualTlsHostnameSettingsSettingArgs{
//! 					ChinaNetwork:                pulumi.Bool(false),
//! 					ClientCertificateForwarding: pulumi.Bool(true),
//! 					Hostname:                    pulumi.String("example.com"),
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
//!             .settings(AccessMutualTlsHostnameSettingsSettingArgs.builder()
//!                 .chinaNetwork(false)
//!                 .clientCertificateForwarding(true)
//!                 .hostname("example.com")
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
//!   example:
//!     type: cloudflare:AccessMutualTlsHostnameSettings
//!     properties:
//!       settings:
//!         - chinaNetwork: false
//!           clientCertificateForwarding: true
//!           hostname: example.com
//!       zoneId: 0da42c8d2132a9ddaf714f9e7c920711
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
    pub settings:
        pulumi_wasm_rust::Output<Option<Vec<crate::types::AccessMutualTlsHostnameSettingsSetting>>>,
    /// The zone identifier to target for the resource.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub zone_id: pulumi_wasm_rust::Output<Option<String>>,
}

pub struct AccessMutualTlsHostnameSettingsResult {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<Option<String>>,
    pub settings:
        pulumi_wasm_rust::Output<Option<Vec<crate::types::AccessMutualTlsHostnameSettingsSetting>>>,
    /// The zone identifier to target for the resource.
    pub zone_id: pulumi_wasm_rust::Output<Option<String>>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(
    name: &str,
    args: AccessMutualTlsHostnameSettingsArgs,
) -> AccessMutualTlsHostnameSettingsResult {
    let result = crate::bindings::pulumi::cloudflare::access_mutual_tls_hostname_settings::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::access_mutual_tls_hostname_settings::Args {
            account_id: &args.account_id.get_inner(),
            settings: &args.settings.get_inner(),
            zone_id: &args.zone_id.get_inner(),
        },
    );

    AccessMutualTlsHostnameSettingsResult {
        account_id: crate::into_domain(result.account_id),
        settings: crate::into_domain(result.settings),
        zone_id: crate::into_domain(result.zone_id),
    }
}
