//! Provides a Cloudflare Authenticated Origin Pulls certificate
//! resource. An uploaded client certificate is required to use Per-Zone
//!  or Per-Hostname Authenticated Origin Pulls.
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as cloudflare from "@pulumi/cloudflare";
//! 
//! // Per-Zone Authenticated Origin Pulls certificate
//! const myPerZoneAopCert = new cloudflare.AuthenticatedOriginPullsCertificate("my_per_zone_aop_cert", {
//!     zoneId: "0da42c8d2132a9ddaf714f9e7c920711",
//!     certificate: "-----INSERT CERTIFICATE-----",
//!     privateKey: "-----INSERT PRIVATE KEY-----",
//!     type: "per-zone",
//! });
//! // Per-Hostname Authenticated Origin Pulls certificate
//! const myPerHostnameAopCert = new cloudflare.AuthenticatedOriginPullsCertificate("my_per_hostname_aop_cert", {
//!     zoneId: "0da42c8d2132a9ddaf714f9e7c920711",
//!     certificate: "-----INSERT CERTIFICATE-----",
//!     privateKey: "-----INSERT PRIVATE KEY-----",
//!     type: "per-hostname",
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//! 
//! # Per-Zone Authenticated Origin Pulls certificate
//! my_per_zone_aop_cert = cloudflare.AuthenticatedOriginPullsCertificate("my_per_zone_aop_cert",
//!     zone_id="0da42c8d2132a9ddaf714f9e7c920711",
//!     certificate="-----INSERT CERTIFICATE-----",
//!     private_key="-----INSERT PRIVATE KEY-----",
//!     type="per-zone")
//! # Per-Hostname Authenticated Origin Pulls certificate
//! my_per_hostname_aop_cert = cloudflare.AuthenticatedOriginPullsCertificate("my_per_hostname_aop_cert",
//!     zone_id="0da42c8d2132a9ddaf714f9e7c920711",
//!     certificate="-----INSERT CERTIFICATE-----",
//!     private_key="-----INSERT PRIVATE KEY-----",
//!     type="per-hostname")
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
//!     // Per-Zone Authenticated Origin Pulls certificate
//!     var myPerZoneAopCert = new Cloudflare.AuthenticatedOriginPullsCertificate("my_per_zone_aop_cert", new()
//!     {
//!         ZoneId = "0da42c8d2132a9ddaf714f9e7c920711",
//!         Certificate = "-----INSERT CERTIFICATE-----",
//!         PrivateKey = "-----INSERT PRIVATE KEY-----",
//!         Type = "per-zone",
//!     });
//! 
//!     // Per-Hostname Authenticated Origin Pulls certificate
//!     var myPerHostnameAopCert = new Cloudflare.AuthenticatedOriginPullsCertificate("my_per_hostname_aop_cert", new()
//!     {
//!         ZoneId = "0da42c8d2132a9ddaf714f9e7c920711",
//!         Certificate = "-----INSERT CERTIFICATE-----",
//!         PrivateKey = "-----INSERT PRIVATE KEY-----",
//!         Type = "per-hostname",
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
//! 		// Per-Zone Authenticated Origin Pulls certificate
//! 		_, err := cloudflare.NewAuthenticatedOriginPullsCertificate(ctx, "my_per_zone_aop_cert", &cloudflare.AuthenticatedOriginPullsCertificateArgs{
//! 			ZoneId:      pulumi.String("0da42c8d2132a9ddaf714f9e7c920711"),
//! 			Certificate: pulumi.String("-----INSERT CERTIFICATE-----"),
//! 			PrivateKey:  pulumi.String("-----INSERT PRIVATE KEY-----"),
//! 			Type:        pulumi.String("per-zone"),
//! 		})
//! 		if err != nil {
//! 			return err
//! 		}
//! 		// Per-Hostname Authenticated Origin Pulls certificate
//! 		_, err = cloudflare.NewAuthenticatedOriginPullsCertificate(ctx, "my_per_hostname_aop_cert", &cloudflare.AuthenticatedOriginPullsCertificateArgs{
//! 			ZoneId:      pulumi.String("0da42c8d2132a9ddaf714f9e7c920711"),
//! 			Certificate: pulumi.String("-----INSERT CERTIFICATE-----"),
//! 			PrivateKey:  pulumi.String("-----INSERT PRIVATE KEY-----"),
//! 			Type:        pulumi.String("per-hostname"),
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
//! import com.pulumi.cloudflare.AuthenticatedOriginPullsCertificate;
//! import com.pulumi.cloudflare.AuthenticatedOriginPullsCertificateArgs;
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
//!         // Per-Zone Authenticated Origin Pulls certificate
//!         var myPerZoneAopCert = new AuthenticatedOriginPullsCertificate("myPerZoneAopCert", AuthenticatedOriginPullsCertificateArgs.builder()
//!             .zoneId("0da42c8d2132a9ddaf714f9e7c920711")
//!             .certificate("-----INSERT CERTIFICATE-----")
//!             .privateKey("-----INSERT PRIVATE KEY-----")
//!             .type("per-zone")
//!             .build());
//! 
//!         // Per-Hostname Authenticated Origin Pulls certificate
//!         var myPerHostnameAopCert = new AuthenticatedOriginPullsCertificate("myPerHostnameAopCert", AuthenticatedOriginPullsCertificateArgs.builder()
//!             .zoneId("0da42c8d2132a9ddaf714f9e7c920711")
//!             .certificate("-----INSERT CERTIFICATE-----")
//!             .privateKey("-----INSERT PRIVATE KEY-----")
//!             .type("per-hostname")
//!             .build());
//! 
//!     }
//! }
//! ```
//! ### YAML
//! ```yaml
//! resources:
//!   # Per-Zone Authenticated Origin Pulls certificate
//!   myPerZoneAopCert:
//!     type: cloudflare:AuthenticatedOriginPullsCertificate
//!     name: my_per_zone_aop_cert
//!     properties:
//!       zoneId: 0da42c8d2132a9ddaf714f9e7c920711
//!       certificate: '-----INSERT CERTIFICATE-----'
//!       privateKey: '-----INSERT PRIVATE KEY-----'
//!       type: per-zone
//!   # Per-Hostname Authenticated Origin Pulls certificate
//!   myPerHostnameAopCert:
//!     type: cloudflare:AuthenticatedOriginPullsCertificate
//!     name: my_per_hostname_aop_cert
//!     properties:
//!       zoneId: 0da42c8d2132a9ddaf714f9e7c920711
//!       certificate: '-----INSERT CERTIFICATE-----'
//!       privateKey: '-----INSERT PRIVATE KEY-----'
//!       type: per-hostname
//! ```
//! <!--End PulumiCodeChooser -->
//! 
//! ## Import
//! 
//! ```sh
//! $ pulumi import cloudflare:index/authenticatedOriginPullsCertificate:AuthenticatedOriginPullsCertificate example <zone_id>/<certificate_type>/<certificate_id>
//! ```
//! 

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct AuthenticatedOriginPullsCertificateArgs {
    /// The public client certificate. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub certificate: pulumi_wasm_rust::Output<String>,
    /// The private key of the client certificate. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub private_key: pulumi_wasm_rust::Output<String>,
    /// The form of Authenticated Origin Pulls to upload the certificate to. Available values: `per-zone`, `per-hostname`. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub type_: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct AuthenticatedOriginPullsCertificateResult {
    /// The public client certificate. **Modifying this attribute will force creation of a new resource.**
    pub certificate: pulumi_wasm_rust::Output<String>,
    /// **Modifying this attribute will force creation of a new resource.**
    pub expires_on: pulumi_wasm_rust::Output<String>,
    /// **Modifying this attribute will force creation of a new resource.**
    pub issuer: pulumi_wasm_rust::Output<String>,
    /// The private key of the client certificate. **Modifying this attribute will force creation of a new resource.**
    pub private_key: pulumi_wasm_rust::Output<String>,
    /// **Modifying this attribute will force creation of a new resource.**
    pub serial_number: pulumi_wasm_rust::Output<String>,
    /// **Modifying this attribute will force creation of a new resource.**
    pub signature: pulumi_wasm_rust::Output<String>,
    /// **Modifying this attribute will force creation of a new resource.**
    pub status: pulumi_wasm_rust::Output<String>,
    /// The form of Authenticated Origin Pulls to upload the certificate to. Available values: `per-zone`, `per-hostname`. **Modifying this attribute will force creation of a new resource.**
    pub type_: pulumi_wasm_rust::Output<String>,
    /// **Modifying this attribute will force creation of a new resource.**
    pub uploaded_on: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: AuthenticatedOriginPullsCertificateArgs) -> AuthenticatedOriginPullsCertificateResult {

    let result = crate::bindings::pulumi::cloudflare::authenticated_origin_pulls_certificate::invoke(name, &crate::bindings::pulumi::cloudflare::authenticated_origin_pulls_certificate::Args {
        certificate: &args.certificate.get_inner(),
        private_key: &args.private_key.get_inner(),
        type_: &args.type_.get_inner(),
        zone_id: &args.zone_id.get_inner(),
    });

    AuthenticatedOriginPullsCertificateResult {
        certificate: crate::into_domain(result.certificate),
        expires_on: crate::into_domain(result.expires_on),
        issuer: crate::into_domain(result.issuer),
        private_key: crate::into_domain(result.private_key),
        serial_number: crate::into_domain(result.serial_number),
        signature: crate::into_domain(result.signature),
        status: crate::into_domain(result.status),
        type_: crate::into_domain(result.type_),
        uploaded_on: crate::into_domain(result.uploaded_on),
        zone_id: crate::into_domain(result.zone_id),
    }
}
