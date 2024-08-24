//! Provides a Cloudflare Authenticated Origin Pulls resource. A `cloudflare.AuthenticatedOriginPulls`
//! resource is required to use Per-Zone or Per-Hostname Authenticated
//! Origin Pulls.
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as cloudflare from "@pulumi/cloudflare";
//! 
//! // Authenticated Origin Pulls
//! const myAop = new cloudflare.AuthenticatedOriginPulls("myAop", {
//!     zoneId: "0da42c8d2132a9ddaf714f9e7c920711",
//!     enabled: true,
//! });
//! // Per-Zone Authenticated Origin Pulls
//! const myPerZoneAopCert = new cloudflare.AuthenticatedOriginPullsCertificate("myPerZoneAopCert", {
//!     zoneId: "0da42c8d2132a9ddaf714f9e7c920711",
//!     certificate: "-----INSERT CERTIFICATE-----",
//!     privateKey: "-----INSERT PRIVATE KEY-----",
//!     type: "per-zone",
//! });
//! const myPerZoneAop = new cloudflare.AuthenticatedOriginPulls("myPerZoneAop", {
//!     zoneId: "0da42c8d2132a9ddaf714f9e7c920711",
//!     authenticatedOriginPullsCertificate: myPerZoneAopCert.id,
//!     enabled: true,
//! });
//! // Per-Hostname Authenticated Origin Pulls
//! const myPerHostnameAopCert = new cloudflare.AuthenticatedOriginPullsCertificate("myPerHostnameAopCert", {
//!     zoneId: "0da42c8d2132a9ddaf714f9e7c920711",
//!     certificate: "-----INSERT CERTIFICATE-----",
//!     privateKey: "-----INSERT PRIVATE KEY-----",
//!     type: "per-hostname",
//! });
//! const myPerHostnameAop = new cloudflare.AuthenticatedOriginPulls("myPerHostnameAop", {
//!     zoneId: "0da42c8d2132a9ddaf714f9e7c920711",
//!     authenticatedOriginPullsCertificate: myPerHostnameAopCert.id,
//!     hostname: "aop.example.com",
//!     enabled: true,
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//! 
//! # Authenticated Origin Pulls
//! my_aop = cloudflare.AuthenticatedOriginPulls("myAop",
//!     zone_id="0da42c8d2132a9ddaf714f9e7c920711",
//!     enabled=True)
//! # Per-Zone Authenticated Origin Pulls
//! my_per_zone_aop_cert = cloudflare.AuthenticatedOriginPullsCertificate("myPerZoneAopCert",
//!     zone_id="0da42c8d2132a9ddaf714f9e7c920711",
//!     certificate="-----INSERT CERTIFICATE-----",
//!     private_key="-----INSERT PRIVATE KEY-----",
//!     type="per-zone")
//! my_per_zone_aop = cloudflare.AuthenticatedOriginPulls("myPerZoneAop",
//!     zone_id="0da42c8d2132a9ddaf714f9e7c920711",
//!     authenticated_origin_pulls_certificate=my_per_zone_aop_cert.id,
//!     enabled=True)
//! # Per-Hostname Authenticated Origin Pulls
//! my_per_hostname_aop_cert = cloudflare.AuthenticatedOriginPullsCertificate("myPerHostnameAopCert",
//!     zone_id="0da42c8d2132a9ddaf714f9e7c920711",
//!     certificate="-----INSERT CERTIFICATE-----",
//!     private_key="-----INSERT PRIVATE KEY-----",
//!     type="per-hostname")
//! my_per_hostname_aop = cloudflare.AuthenticatedOriginPulls("myPerHostnameAop",
//!     zone_id="0da42c8d2132a9ddaf714f9e7c920711",
//!     authenticated_origin_pulls_certificate=my_per_hostname_aop_cert.id,
//!     hostname="aop.example.com",
//!     enabled=True)
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
//!     // Authenticated Origin Pulls
//!     var myAop = new Cloudflare.AuthenticatedOriginPulls("myAop", new()
//!     {
//!         ZoneId = "0da42c8d2132a9ddaf714f9e7c920711",
//!         Enabled = true,
//!     });
//! 
//!     // Per-Zone Authenticated Origin Pulls
//!     var myPerZoneAopCert = new Cloudflare.AuthenticatedOriginPullsCertificate("myPerZoneAopCert", new()
//!     {
//!         ZoneId = "0da42c8d2132a9ddaf714f9e7c920711",
//!         Certificate = "-----INSERT CERTIFICATE-----",
//!         PrivateKey = "-----INSERT PRIVATE KEY-----",
//!         Type = "per-zone",
//!     });
//! 
//!     var myPerZoneAop = new Cloudflare.AuthenticatedOriginPulls("myPerZoneAop", new()
//!     {
//!         ZoneId = "0da42c8d2132a9ddaf714f9e7c920711",
//!         AuthenticatedOriginPullsCertificate = myPerZoneAopCert.Id,
//!         Enabled = true,
//!     });
//! 
//!     // Per-Hostname Authenticated Origin Pulls
//!     var myPerHostnameAopCert = new Cloudflare.AuthenticatedOriginPullsCertificate("myPerHostnameAopCert", new()
//!     {
//!         ZoneId = "0da42c8d2132a9ddaf714f9e7c920711",
//!         Certificate = "-----INSERT CERTIFICATE-----",
//!         PrivateKey = "-----INSERT PRIVATE KEY-----",
//!         Type = "per-hostname",
//!     });
//! 
//!     var myPerHostnameAop = new Cloudflare.AuthenticatedOriginPulls("myPerHostnameAop", new()
//!     {
//!         ZoneId = "0da42c8d2132a9ddaf714f9e7c920711",
//!         AuthenticatedOriginPullsCertificate = myPerHostnameAopCert.Id,
//!         Hostname = "aop.example.com",
//!         Enabled = true,
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
//! 		// Authenticated Origin Pulls
//! 		_, err := cloudflare.NewAuthenticatedOriginPulls(ctx, "myAop", &cloudflare.AuthenticatedOriginPullsArgs{
//! 			ZoneId:  pulumi.String("0da42c8d2132a9ddaf714f9e7c920711"),
//! 			Enabled: pulumi.Bool(true),
//! 		})
//! 		if err != nil {
//! 			return err
//! 		}
//! 		// Per-Zone Authenticated Origin Pulls
//! 		myPerZoneAopCert, err := cloudflare.NewAuthenticatedOriginPullsCertificate(ctx, "myPerZoneAopCert", &cloudflare.AuthenticatedOriginPullsCertificateArgs{
//! 			ZoneId:      pulumi.String("0da42c8d2132a9ddaf714f9e7c920711"),
//! 			Certificate: pulumi.String("-----INSERT CERTIFICATE-----"),
//! 			PrivateKey:  pulumi.String("-----INSERT PRIVATE KEY-----"),
//! 			Type:        pulumi.String("per-zone"),
//! 		})
//! 		if err != nil {
//! 			return err
//! 		}
//! 		_, err = cloudflare.NewAuthenticatedOriginPulls(ctx, "myPerZoneAop", &cloudflare.AuthenticatedOriginPullsArgs{
//! 			ZoneId:                              pulumi.String("0da42c8d2132a9ddaf714f9e7c920711"),
//! 			AuthenticatedOriginPullsCertificate: myPerZoneAopCert.ID(),
//! 			Enabled:                             pulumi.Bool(true),
//! 		})
//! 		if err != nil {
//! 			return err
//! 		}
//! 		// Per-Hostname Authenticated Origin Pulls
//! 		myPerHostnameAopCert, err := cloudflare.NewAuthenticatedOriginPullsCertificate(ctx, "myPerHostnameAopCert", &cloudflare.AuthenticatedOriginPullsCertificateArgs{
//! 			ZoneId:      pulumi.String("0da42c8d2132a9ddaf714f9e7c920711"),
//! 			Certificate: pulumi.String("-----INSERT CERTIFICATE-----"),
//! 			PrivateKey:  pulumi.String("-----INSERT PRIVATE KEY-----"),
//! 			Type:        pulumi.String("per-hostname"),
//! 		})
//! 		if err != nil {
//! 			return err
//! 		}
//! 		_, err = cloudflare.NewAuthenticatedOriginPulls(ctx, "myPerHostnameAop", &cloudflare.AuthenticatedOriginPullsArgs{
//! 			ZoneId:                              pulumi.String("0da42c8d2132a9ddaf714f9e7c920711"),
//! 			AuthenticatedOriginPullsCertificate: myPerHostnameAopCert.ID(),
//! 			Hostname:                            pulumi.String("aop.example.com"),
//! 			Enabled:                             pulumi.Bool(true),
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
//! import com.pulumi.cloudflare.AuthenticatedOriginPulls;
//! import com.pulumi.cloudflare.AuthenticatedOriginPullsArgs;
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
//!         // Authenticated Origin Pulls
//!         var myAop = new AuthenticatedOriginPulls("myAop", AuthenticatedOriginPullsArgs.builder()        
//!             .zoneId("0da42c8d2132a9ddaf714f9e7c920711")
//!             .enabled(true)
//!             .build());
//! 
//!         // Per-Zone Authenticated Origin Pulls
//!         var myPerZoneAopCert = new AuthenticatedOriginPullsCertificate("myPerZoneAopCert", AuthenticatedOriginPullsCertificateArgs.builder()        
//!             .zoneId("0da42c8d2132a9ddaf714f9e7c920711")
//!             .certificate("-----INSERT CERTIFICATE-----")
//!             .privateKey("-----INSERT PRIVATE KEY-----")
//!             .type("per-zone")
//!             .build());
//! 
//!         var myPerZoneAop = new AuthenticatedOriginPulls("myPerZoneAop", AuthenticatedOriginPullsArgs.builder()        
//!             .zoneId("0da42c8d2132a9ddaf714f9e7c920711")
//!             .authenticatedOriginPullsCertificate(myPerZoneAopCert.id())
//!             .enabled(true)
//!             .build());
//! 
//!         // Per-Hostname Authenticated Origin Pulls
//!         var myPerHostnameAopCert = new AuthenticatedOriginPullsCertificate("myPerHostnameAopCert", AuthenticatedOriginPullsCertificateArgs.builder()        
//!             .zoneId("0da42c8d2132a9ddaf714f9e7c920711")
//!             .certificate("-----INSERT CERTIFICATE-----")
//!             .privateKey("-----INSERT PRIVATE KEY-----")
//!             .type("per-hostname")
//!             .build());
//! 
//!         var myPerHostnameAop = new AuthenticatedOriginPulls("myPerHostnameAop", AuthenticatedOriginPullsArgs.builder()        
//!             .zoneId("0da42c8d2132a9ddaf714f9e7c920711")
//!             .authenticatedOriginPullsCertificate(myPerHostnameAopCert.id())
//!             .hostname("aop.example.com")
//!             .enabled(true)
//!             .build());
//! 
//!     }
//! }
//! ```
//! ### YAML
//! ```yaml
//! resources:
//!   # Authenticated Origin Pulls
//!   myAop:
//!     type: cloudflare:AuthenticatedOriginPulls
//!     properties:
//!       zoneId: 0da42c8d2132a9ddaf714f9e7c920711
//!       enabled: true
//!   # Per-Zone Authenticated Origin Pulls
//!   myPerZoneAopCert:
//!     type: cloudflare:AuthenticatedOriginPullsCertificate
//!     properties:
//!       zoneId: 0da42c8d2132a9ddaf714f9e7c920711
//!       certificate: '-----INSERT CERTIFICATE-----'
//!       privateKey: '-----INSERT PRIVATE KEY-----'
//!       type: per-zone
//!   myPerZoneAop:
//!     type: cloudflare:AuthenticatedOriginPulls
//!     properties:
//!       zoneId: 0da42c8d2132a9ddaf714f9e7c920711
//!       authenticatedOriginPullsCertificate: ${myPerZoneAopCert.id}
//!       enabled: true
//!   # Per-Hostname Authenticated Origin Pulls
//!   myPerHostnameAopCert:
//!     type: cloudflare:AuthenticatedOriginPullsCertificate
//!     properties:
//!       zoneId: 0da42c8d2132a9ddaf714f9e7c920711
//!       certificate: '-----INSERT CERTIFICATE-----'
//!       privateKey: '-----INSERT PRIVATE KEY-----'
//!       type: per-hostname
//!   myPerHostnameAop:
//!     type: cloudflare:AuthenticatedOriginPulls
//!     properties:
//!       zoneId: 0da42c8d2132a9ddaf714f9e7c920711
//!       authenticatedOriginPullsCertificate: ${myPerHostnameAopCert.id}
//!       hostname: aop.example.com
//!       enabled: true
//! ```
//! <!--End PulumiCodeChooser -->
//! 
//! ## Import
//! 
//! global
//! 
//! ```sh
//! $ pulumi import cloudflare:index/authenticatedOriginPulls:AuthenticatedOriginPulls example <zone_id>
//! ```
//! 
//! per zone
//! 
//! ```sh
//! $ pulumi import cloudflare:index/authenticatedOriginPulls:AuthenticatedOriginPulls example <zone_id>/<certificate_id>
//! ```
//! 
//! per hostname
//! 
//! ```sh
//! $ pulumi import cloudflare:index/authenticatedOriginPulls:AuthenticatedOriginPulls example <zone_id>/<certificate_id>/<hostname>
//! ```
//! 

pub struct AuthenticatedOriginPullsArgs {
    /// The ID of an uploaded Authenticated Origin Pulls certificate. If no hostname is provided, this certificate will be used zone wide as Per-Zone Authenticated Origin Pulls.
    pub authenticated_origin_pulls_certificate: pulumi_wasm_rust::Output<Option<String>>,
    /// Whether to enable Authenticated Origin Pulls on the given zone or hostname.
    pub enabled: pulumi_wasm_rust::Output<bool>,
    /// Specify a hostname to enable Per-Hostname Authenticated Origin Pulls on, using the provided certificate.
    pub hostname: pulumi_wasm_rust::Output<Option<String>>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct AuthenticatedOriginPullsResult {
    /// The ID of an uploaded Authenticated Origin Pulls certificate. If no hostname is provided, this certificate will be used zone wide as Per-Zone Authenticated Origin Pulls.
    pub authenticated_origin_pulls_certificate: pulumi_wasm_rust::Output<Option<String>>,
    /// Whether to enable Authenticated Origin Pulls on the given zone or hostname.
    pub enabled: pulumi_wasm_rust::Output<bool>,
    /// Specify a hostname to enable Per-Hostname Authenticated Origin Pulls on, using the provided certificate.
    pub hostname: pulumi_wasm_rust::Output<Option<String>>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: AuthenticatedOriginPullsArgs) -> AuthenticatedOriginPullsResult {

    let result = crate::bindings::pulumi::cloudflare::authenticated_origin_pulls::invoke(name, &crate::bindings::pulumi::cloudflare::authenticated_origin_pulls::Args {
        authenticated_origin_pulls_certificate: args.authenticated_origin_pulls_certificate.get_inner(),
        enabled: args.enabled.get_inner(),
        hostname: args.hostname.get_inner(),
        zone_id: args.zone_id.get_inner(),
    });

    AuthenticatedOriginPullsResult {
        authenticated_origin_pulls_certificate: crate::into_domain(result.authenticated_origin_pulls_certificate),
        enabled: crate::into_domain(result.enabled),
        hostname: crate::into_domain(result.hostname),
        zone_id: crate::into_domain(result.zone_id),
    }
}
