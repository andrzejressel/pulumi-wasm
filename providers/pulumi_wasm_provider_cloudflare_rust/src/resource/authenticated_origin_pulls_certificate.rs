//! Provides a Cloudflare Authenticated Origin Pulls certificate
//! resource. An uploaded client certificate is required to use Per-Zone
//!  or Per-Hostname Authenticated Origin Pulls.
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ```rust
//! use pulumi_wasm_rust::Output;
//! use pulumi_wasm_rust::{add_export, pulumi_main};
//! #[pulumi_main]
//! fn test_main() -> Result<(), Error> {
//!     let myPerHostnameAopCert = authenticated_origin_pulls_certificate::create(
//!         "myPerHostnameAopCert",
//!         AuthenticatedOriginPullsCertificateArgs::builder()
//!             .certificate("-----INSERT CERTIFICATE-----")
//!             .private_key("-----INSERT PRIVATE KEY-----")
//!             .type_("per-hostname")
//!             .zone_id("0da42c8d2132a9ddaf714f9e7c920711")
//!             .build_struct(),
//!     );
//!     let myPerZoneAopCert = authenticated_origin_pulls_certificate::create(
//!         "myPerZoneAopCert",
//!         AuthenticatedOriginPullsCertificateArgs::builder()
//!             .certificate("-----INSERT CERTIFICATE-----")
//!             .private_key("-----INSERT PRIVATE KEY-----")
//!             .type_("per-zone")
//!             .zone_id("0da42c8d2132a9ddaf714f9e7c920711")
//!             .build_struct(),
//!     );
//! }
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
