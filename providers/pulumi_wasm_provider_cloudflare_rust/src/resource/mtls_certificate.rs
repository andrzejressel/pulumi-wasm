//! Provides a Cloudflare mTLS certificate resource. These certificates may be used with mTLS enabled Cloudflare services.
//! 
//! ## Example Usage
//! 
//! ```ignore
//! use pulumi_wasm_rust::Output;
//! use pulumi_wasm_rust::{add_export, pulumi_main};
//! #[pulumi_main]
//! fn test_main() -> Result<(), Error> {
//!     let example = mtls_certificate::create(
//!         "example",
//!         MtlsCertificateArgs::builder()
//!             .account_id("f037e56e89293a057740de681ac9abbe")
//!             .ca(true)
//!             .certificates(
//!                 "-----BEGIN CERTIFICATE-----\nMIIDmDCCAoCgAwIBAgIUKTOAZNj...i4JhqeoTewsxndhDDE\n-----END CERTIFICATE-----",
//!             )
//!             .name("example")
//!             .private_key(
//!                 "-----BEGIN PRIVATE KEY-----\nMIIEvQIBADANBgkqhkiG9w0BAQE...1IS3EnQRrz6WMYA=\n-----END PRIVATE KEY-----",
//!             )
//!             .build_struct(),
//!     );
//! }
//! ```
//! 
//! ## Import
//! 
//! ```sh
//! $ pulumi import cloudflare:index/mtlsCertificate:MtlsCertificate example <account_id>/<mtls_certificate_id>
//! ```
//! 

#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct MtlsCertificateArgs {
    /// The account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// Whether this is a CA or leaf certificate. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub ca: pulumi_wasm_rust::Output<bool>,
    /// Certificate you intend to use with mTLS-enabled services. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub certificates: pulumi_wasm_rust::Output<String>,
    /// Optional unique name for the certificate. **Modifying this attribute will force creation of a new resource.**
    #[builder(into, default)]
    pub name: pulumi_wasm_rust::Output<Option<String>>,
    /// The certificate's private key. **Modifying this attribute will force creation of a new resource.**
    #[builder(into, default)]
    pub private_key: pulumi_wasm_rust::Output<Option<String>>,
}

pub struct MtlsCertificateResult {
    /// The account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// Whether this is a CA or leaf certificate. **Modifying this attribute will force creation of a new resource.**
    pub ca: pulumi_wasm_rust::Output<bool>,
    /// Certificate you intend to use with mTLS-enabled services. **Modifying this attribute will force creation of a new resource.**
    pub certificates: pulumi_wasm_rust::Output<String>,
    /// **Modifying this attribute will force creation of a new resource.**
    pub expires_on: pulumi_wasm_rust::Output<String>,
    /// **Modifying this attribute will force creation of a new resource.**
    pub issuer: pulumi_wasm_rust::Output<String>,
    /// Optional unique name for the certificate. **Modifying this attribute will force creation of a new resource.**
    pub name: pulumi_wasm_rust::Output<Option<String>>,
    /// The certificate's private key. **Modifying this attribute will force creation of a new resource.**
    pub private_key: pulumi_wasm_rust::Output<Option<String>>,
    /// **Modifying this attribute will force creation of a new resource.**
    pub serial_number: pulumi_wasm_rust::Output<String>,
    /// **Modifying this attribute will force creation of a new resource.**
    pub signature: pulumi_wasm_rust::Output<String>,
    /// **Modifying this attribute will force creation of a new resource.**
    pub uploaded_on: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(
    name: &str,
    args: MtlsCertificateArgs
) -> MtlsCertificateResult {

    let result = crate::bindings::pulumi::cloudflare::mtls_certificate::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::mtls_certificate::Args {
                account_id: &args.account_id.get_inner(),
                ca: &args.ca.get_inner(),
                certificates: &args.certificates.get_inner(),
                name: &args.name.get_inner(),
                private_key: &args.private_key.get_inner(),
        }
    );

    MtlsCertificateResult {
        account_id: crate::into_domain(result.account_id),
        ca: crate::into_domain(result.ca),
        certificates: crate::into_domain(result.certificates),
        expires_on: crate::into_domain(result.expires_on),
        issuer: crate::into_domain(result.issuer),
        name: crate::into_domain(result.name),
        private_key: crate::into_domain(result.private_key),
        serial_number: crate::into_domain(result.serial_number),
        signature: crate::into_domain(result.signature),
        uploaded_on: crate::into_domain(result.uploaded_on),
    }
}
