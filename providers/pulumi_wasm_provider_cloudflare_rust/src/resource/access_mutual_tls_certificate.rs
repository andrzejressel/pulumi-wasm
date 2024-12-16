//! Provides a Cloudflare Access Mutual TLS Certificate resource.
//! Mutual TLS authentication ensures that the traffic is secure and
//! trusted in both directions between a client and server and can be
//!  used with Access to only allows requests from devices with a
//!  corresponding client certificate.
//! 
//! > It's required that an `account_id` or `zone_id` is provided and in
//!    most cases using either is fine. However, if you're using a scoped
//!    access token, you must provide the argument that matches the token's
//!    scope. For example, an access token that is scoped to the "example.com"
//!    zone needs to use the `zone_id` argument.
//! 
//! ## Example Usage
//! 
//! ```ignore
//! use pulumi_wasm_rust::Output;
//! use pulumi_wasm_rust::{add_export, pulumi_main};
//! #[pulumi_main]
//! fn test_main() -> Result<(), Error> {
//!     let myCert = access_mutual_tls_certificate::create(
//!         "myCert",
//!         AccessMutualTlsCertificateArgs::builder()
//!             .associated_hostnames(vec!["staging.example.com",])
//!             .certificate("${caPem}")
//!             .name("My Root Cert")
//!             .zone_id("0da42c8d2132a9ddaf714f9e7c920711")
//!             .build_struct(),
//!     );
//! }
//! ```
//! 
//! ## Import
//! 
//! Account level import.
//! 
//! ```sh
//! $ pulumi import cloudflare:index/accessMutualTlsCertificate:AccessMutualTlsCertificate example account/<account_id>/<mutual_tls_certificate_id>
//! ```
//! 
//! Zone level import.
//! 
//! ```sh
//! $ pulumi import cloudflare:index/accessMutualTlsCertificate:AccessMutualTlsCertificate example zone/<zone_id>/<mutual_tls_certificate_id>
//! ```
//! 

#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct AccessMutualTlsCertificateArgs {
    /// The account identifier to target for the resource. Conflicts with `zone_id`.
    #[builder(into, default)]
    pub account_id: pulumi_wasm_rust::Output<Option<String>>,
    /// The hostnames that will be prompted for this certificate.
    #[builder(into, default)]
    pub associated_hostnames: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    /// The Root CA for your certificates.
    #[builder(into, default)]
    pub certificate: pulumi_wasm_rust::Output<Option<String>>,
    /// The name of the certificate.
    #[builder(into)]
    pub name: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource. Conflicts with `account_id`.
    #[builder(into, default)]
    pub zone_id: pulumi_wasm_rust::Output<Option<String>>,
}

pub struct AccessMutualTlsCertificateResult {
    /// The account identifier to target for the resource. Conflicts with `zone_id`.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The hostnames that will be prompted for this certificate.
    pub associated_hostnames: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    /// The Root CA for your certificates.
    pub certificate: pulumi_wasm_rust::Output<Option<String>>,
    pub fingerprint: pulumi_wasm_rust::Output<String>,
    /// The name of the certificate.
    pub name: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource. Conflicts with `account_id`.
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: AccessMutualTlsCertificateArgs) -> AccessMutualTlsCertificateResult {

    let result = crate::bindings::pulumi::cloudflare::access_mutual_tls_certificate::invoke(name, &crate::bindings::pulumi::cloudflare::access_mutual_tls_certificate::Args {
        account_id: &args.account_id.get_inner(),
        associated_hostnames: &args.associated_hostnames.get_inner(),
        certificate: &args.certificate.get_inner(),
        name: &args.name.get_inner(),
        zone_id: &args.zone_id.get_inner(),
    });

    AccessMutualTlsCertificateResult {
        account_id: crate::into_domain(result.account_id),
        associated_hostnames: crate::into_domain(result.associated_hostnames),
        certificate: crate::into_domain(result.certificate),
        fingerprint: crate::into_domain(result.fingerprint),
        name: crate::into_domain(result.name),
        zone_id: crate::into_domain(result.zone_id),
    }
}
