//! Use this data source to retrieve an existing origin ca certificate.
//! 
//! ## Example Usage
//! 
//! ```ignore
//! use pulumi_wasm_rust::Output;
//! use pulumi_wasm_rust::{add_export, pulumi_main};
//! #[pulumi_main]
//! fn test_main() -> Result<(), Error> {
//!     let example = get_origin_ca_certificate::invoke(
//!         GetOriginCaCertificateArgs::builder().id("REPLACE_ME").build_struct(),
//!     );
//! }
//! ```

#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct GetOriginCaCertificateArgs {
    /// The Origin CA Certificate unique identifier.
    #[builder(into)]
    pub id: pulumi_wasm_rust::Output<String>,
}

pub struct GetOriginCaCertificateResult {
    /// The Origin CA certificate.
    pub certificate: pulumi_wasm_rust::Output<String>,
    /// The timestamp when the certificate will expire.
    pub expires_on: pulumi_wasm_rust::Output<String>,
    /// A list of hostnames or wildcard names bound to the certificate.
    pub hostnames: pulumi_wasm_rust::Output<Vec<String>>,
    /// The Origin CA Certificate unique identifier.
    pub id: pulumi_wasm_rust::Output<String>,
    /// The signature type desired on the certificate. Available values: `origin-rsa`, `origin-ecc`, `keyless-certificate`
    pub request_type: pulumi_wasm_rust::Output<String>,
    /// The timestamp when the certificate was revoked.
    pub revoked_at: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn invoke(
    args: GetOriginCaCertificateArgs
) -> GetOriginCaCertificateResult {

    let result = crate::bindings::pulumi::cloudflare::get_origin_ca_certificate::invoke(
        &crate::bindings::pulumi::cloudflare::get_origin_ca_certificate::Args {
                id: &args.id.get_inner(),
        }
    );

    GetOriginCaCertificateResult {
        certificate: crate::into_domain(result.certificate),
        expires_on: crate::into_domain(result.expires_on),
        hostnames: crate::into_domain(result.hostnames),
        id: crate::into_domain(result.id),
        request_type: crate::into_domain(result.request_type),
        revoked_at: crate::into_domain(result.revoked_at),
    }
}
