//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ### Typescript
//! ```typescript
//! import * as pulumi from "@pulumi/pulumi";
//! import * as cloudflare from "@pulumi/cloudflare";
//! 
//! // Advanced certificate manager for Let's Encrypt
//! const example = new cloudflare.CertificatePack("example", {
//!     certificateAuthority: "lets_encrypt",
//!     cloudflareBranding: false,
//!     hosts: [
//!         "example.com",
//!         "*.example.com",
//!     ],
//!     type: "advanced",
//!     validationMethod: "http",
//!     validityDays: 90,
//!     waitForActiveStatus: true,
//!     zoneId: "0da42c8d2132a9ddaf714f9e7c920711",
//! });
//! ```
//! ### Python
//! ```python
//! import pulumi
//! import pulumi_cloudflare as cloudflare
//! 
//! # Advanced certificate manager for Let's Encrypt
//! example = cloudflare.CertificatePack("example",
//!     certificate_authority="lets_encrypt",
//!     cloudflare_branding=False,
//!     hosts=[
//!         "example.com",
//!         "*.example.com",
//!     ],
//!     type="advanced",
//!     validation_method="http",
//!     validity_days=90,
//!     wait_for_active_status=True,
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
//!     // Advanced certificate manager for Let's Encrypt
//!     var example = new Cloudflare.CertificatePack("example", new()
//!     {
//!         CertificateAuthority = "lets_encrypt",
//!         CloudflareBranding = false,
//!         Hosts = new[]
//!         {
//!             "example.com",
//!             "*.example.com",
//!         },
//!         Type = "advanced",
//!         ValidationMethod = "http",
//!         ValidityDays = 90,
//!         WaitForActiveStatus = true,
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
//! 		// Advanced certificate manager for Let's Encrypt
//! 		_, err := cloudflare.NewCertificatePack(ctx, "example", &cloudflare.CertificatePackArgs{
//! 			CertificateAuthority: pulumi.String("lets_encrypt"),
//! 			CloudflareBranding:   pulumi.Bool(false),
//! 			Hosts: pulumi.StringArray{
//! 				pulumi.String("example.com"),
//! 				pulumi.String("*.example.com"),
//! 			},
//! 			Type:                pulumi.String("advanced"),
//! 			ValidationMethod:    pulumi.String("http"),
//! 			ValidityDays:        pulumi.Int(90),
//! 			WaitForActiveStatus: pulumi.Bool(true),
//! 			ZoneId:              pulumi.String("0da42c8d2132a9ddaf714f9e7c920711"),
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
//! import com.pulumi.cloudflare.CertificatePack;
//! import com.pulumi.cloudflare.CertificatePackArgs;
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
//!         // Advanced certificate manager for Let's Encrypt
//!         var example = new CertificatePack("example", CertificatePackArgs.builder()        
//!             .certificateAuthority("lets_encrypt")
//!             .cloudflareBranding(false)
//!             .hosts(            
//!                 "example.com",
//!                 "*.example.com")
//!             .type("advanced")
//!             .validationMethod("http")
//!             .validityDays(90)
//!             .waitForActiveStatus(true)
//!             .zoneId("0da42c8d2132a9ddaf714f9e7c920711")
//!             .build());
//! 
//!     }
//! }
//! ```
//! ### YAML
//! ```yaml
//! resources:
//!   # Advanced certificate manager for Let's Encrypt
//!   example:
//!     type: cloudflare:CertificatePack
//!     properties:
//!       certificateAuthority: lets_encrypt
//!       cloudflareBranding: false
//!       hosts:
//!         - example.com
//!         - '*.example.com'
//!       type: advanced
//!       validationMethod: http
//!       validityDays: 90
//!       waitForActiveStatus: true
//!       zoneId: 0da42c8d2132a9ddaf714f9e7c920711
//! ```
//! <!--End PulumiCodeChooser -->
//! 
//! ## Import
//! 
//! ```sh
//! $ pulumi import cloudflare:index/certificatePack:CertificatePack example <zone_id>/<certificate_pack_id>
//! ```
//! 
//! While supported, importing isn't recommended and it is advised to replace the
//! 
//! certificate entirely instead.
//! 

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct CertificatePackArgs {
    /// Which certificate authority to issue the certificate pack. Available values: `digicert`, `lets_encrypt`, `google`. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub certificate_authority: pulumi_wasm_rust::Output<String>,
    /// Whether or not to include Cloudflare branding. This will add `sni.cloudflaressl.com` as the Common Name if set to `true`. **Modifying this attribute will force creation of a new resource.**
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub cloudflare_branding: pulumi_wasm_rust::Output<Option<bool>>,
    /// List of hostnames to provision the certificate pack for. The zone name must be included as a host. Note: If using Let's Encrypt, you cannot use individual subdomains and only a wildcard for subdomain is available. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub hosts: pulumi_wasm_rust::Output<Vec<String>>,
    /// Certificate pack configuration type. Available values: `advanced`. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub type_: pulumi_wasm_rust::Output<String>,
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub validation_errors: pulumi_wasm_rust::Output<Option<Vec<crate::types::CertificatePackValidationError>>>,
    /// Which validation method to use in order to prove domain ownership. Available values: `txt`, `http`, `email`. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub validation_method: pulumi_wasm_rust::Output<String>,
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub validation_records: pulumi_wasm_rust::Output<Option<Vec<crate::types::CertificatePackValidationRecord>>>,
    /// How long the certificate is valid for. Note: If using Let's Encrypt, this value can only be 90 days. Available values: `14`, `30`, `90`, `365`. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub validity_days: pulumi_wasm_rust::Output<i32>,
    /// Whether or not to wait for a certificate pack to reach status `active` during creation. Defaults to `false`. **Modifying this attribute will force creation of a new resource.**
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub wait_for_active_status: pulumi_wasm_rust::Output<Option<bool>>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct CertificatePackResult {
    /// Which certificate authority to issue the certificate pack. Available values: `digicert`, `lets_encrypt`, `google`. **Modifying this attribute will force creation of a new resource.**
    pub certificate_authority: pulumi_wasm_rust::Output<String>,
    /// Whether or not to include Cloudflare branding. This will add `sni.cloudflaressl.com` as the Common Name if set to `true`. **Modifying this attribute will force creation of a new resource.**
    pub cloudflare_branding: pulumi_wasm_rust::Output<Option<bool>>,
    /// List of hostnames to provision the certificate pack for. The zone name must be included as a host. Note: If using Let's Encrypt, you cannot use individual subdomains and only a wildcard for subdomain is available. **Modifying this attribute will force creation of a new resource.**
    pub hosts: pulumi_wasm_rust::Output<Vec<String>>,
    /// Certificate pack configuration type. Available values: `advanced`. **Modifying this attribute will force creation of a new resource.**
    pub type_: pulumi_wasm_rust::Output<String>,
    pub validation_errors: pulumi_wasm_rust::Output<Vec<crate::types::CertificatePackValidationError>>,
    /// Which validation method to use in order to prove domain ownership. Available values: `txt`, `http`, `email`. **Modifying this attribute will force creation of a new resource.**
    pub validation_method: pulumi_wasm_rust::Output<String>,
    pub validation_records: pulumi_wasm_rust::Output<Vec<crate::types::CertificatePackValidationRecord>>,
    /// How long the certificate is valid for. Note: If using Let's Encrypt, this value can only be 90 days. Available values: `14`, `30`, `90`, `365`. **Modifying this attribute will force creation of a new resource.**
    pub validity_days: pulumi_wasm_rust::Output<i32>,
    /// Whether or not to wait for a certificate pack to reach status `active` during creation. Defaults to `false`. **Modifying this attribute will force creation of a new resource.**
    pub wait_for_active_status: pulumi_wasm_rust::Output<Option<bool>>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: CertificatePackArgs) -> CertificatePackResult {

    let result = crate::bindings::pulumi::cloudflare::certificate_pack::invoke(name, &crate::bindings::pulumi::cloudflare::certificate_pack::Args {
        certificate_authority: &args.certificate_authority.get_inner(),
        cloudflare_branding: &args.cloudflare_branding.get_inner(),
        hosts: &args.hosts.get_inner(),
        type_: &args.type_.get_inner(),
        validation_errors: &args.validation_errors.get_inner(),
        validation_method: &args.validation_method.get_inner(),
        validation_records: &args.validation_records.get_inner(),
        validity_days: &args.validity_days.get_inner(),
        wait_for_active_status: &args.wait_for_active_status.get_inner(),
        zone_id: &args.zone_id.get_inner(),
    });

    CertificatePackResult {
        certificate_authority: crate::into_domain(result.certificate_authority),
        cloudflare_branding: crate::into_domain(result.cloudflare_branding),
        hosts: crate::into_domain(result.hosts),
        type_: crate::into_domain(result.type_),
        validation_errors: crate::into_domain(result.validation_errors),
        validation_method: crate::into_domain(result.validation_method),
        validation_records: crate::into_domain(result.validation_records),
        validity_days: crate::into_domain(result.validity_days),
        wait_for_active_status: crate::into_domain(result.wait_for_active_status),
        zone_id: crate::into_domain(result.zone_id),
    }
}
