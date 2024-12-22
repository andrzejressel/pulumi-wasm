//! Provides a Cloudflare Origin CA certificate used to protect traffic to your origin without involving a third party Certificate Authority.
//! 
//! > Since v3.32.0
//!    all authentication schemes are supported for managing Origin CA certificates.
//!    Versions prior to v3.32.0 will still need to use `api_user_service_key`.
//! 
//! ## Example Usage
//! 
//! ```yaml
//! resources:
//!   example:
//!     type: tls:privateKey
//!     properties:
//!       algorithm: RSA
//!   exampleCertRequest:
//!     type: tls:certRequest
//!     name: example
//!     properties:
//!       privateKeyPem: ${example.privateKeyPem}
//!       subject:
//!         - commonName:
//!           organization: Terraform Test
//!   exampleOriginCaCertificate:
//!     type: cloudflare:OriginCaCertificate
//!     name: example
//!     properties:
//!       csr: ${exampleCertRequest.certRequestPem}
//!       hostnames:
//!         - example.com
//!       requestType: origin-rsa
//!       requestedValidity: 7
//! ```
//! 
//! ## Import
//! 
//! ```sh
//! $ pulumi import cloudflare:index/originCaCertificate:OriginCaCertificate example <certificate_id>
//! ```
//! 

#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct OriginCaCertificateArgs {
    /// The Certificate Signing Request. Must be newline-encoded. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub csr: pulumi_wasm_rust::Output<String>,
    /// A list of hostnames or wildcard names bound to the certificate. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub hostnames: pulumi_wasm_rust::Output<Vec<String>>,
    #[builder(into, default)]
    pub min_days_for_renewal: pulumi_wasm_rust::Output<Option<i32>>,
    /// The signature type desired on the certificate. Available values: `origin-rsa`, `origin-ecc`, `keyless-certificate`. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub request_type: pulumi_wasm_rust::Output<String>,
    /// The number of days for which the certificate should be valid. Available values: `7`, `30`, `90`, `365`, `730`, `1095`, `5475`. **Modifying this attribute will force creation of a new resource.**
    #[builder(into, default)]
    pub requested_validity: pulumi_wasm_rust::Output<Option<i32>>,
}

pub struct OriginCaCertificateResult {
    /// The Origin CA certificate.
    pub certificate: pulumi_wasm_rust::Output<String>,
    /// The Certificate Signing Request. Must be newline-encoded. **Modifying this attribute will force creation of a new resource.**
    pub csr: pulumi_wasm_rust::Output<String>,
    /// The datetime when the certificate will expire.
    pub expires_on: pulumi_wasm_rust::Output<String>,
    /// A list of hostnames or wildcard names bound to the certificate. **Modifying this attribute will force creation of a new resource.**
    pub hostnames: pulumi_wasm_rust::Output<Vec<String>>,
    pub min_days_for_renewal: pulumi_wasm_rust::Output<Option<i32>>,
    /// The signature type desired on the certificate. Available values: `origin-rsa`, `origin-ecc`, `keyless-certificate`. **Modifying this attribute will force creation of a new resource.**
    pub request_type: pulumi_wasm_rust::Output<String>,
    /// The number of days for which the certificate should be valid. Available values: `7`, `30`, `90`, `365`, `730`, `1095`, `5475`. **Modifying this attribute will force creation of a new resource.**
    pub requested_validity: pulumi_wasm_rust::Output<i32>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(
    name: &str,
    args: OriginCaCertificateArgs
) -> OriginCaCertificateResult {

    let result = crate::bindings::pulumi::cloudflare::origin_ca_certificate::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::origin_ca_certificate::Args {
                csr: &args.csr.get_inner(),
                hostnames: &args.hostnames.get_inner(),
                min_days_for_renewal: &args.min_days_for_renewal.get_inner(),
                request_type: &args.request_type.get_inner(),
                requested_validity: &args.requested_validity.get_inner(),
        }
    );

    OriginCaCertificateResult {
        certificate: crate::into_domain(result.certificate),
        csr: crate::into_domain(result.csr),
        expires_on: crate::into_domain(result.expires_on),
        hostnames: crate::into_domain(result.hostnames),
        min_days_for_renewal: crate::into_domain(result.min_days_for_renewal),
        request_type: crate::into_domain(result.request_type),
        requested_validity: crate::into_domain(result.requested_validity),
    }
}
