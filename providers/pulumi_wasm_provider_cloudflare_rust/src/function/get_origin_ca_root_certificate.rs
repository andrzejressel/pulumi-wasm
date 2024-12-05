//! Use this data source to get the
//! [Origin CA root certificate](https://developers.cloudflare.com/ssl/origin-configuration/origin-ca#4-required-for-some-add-cloudflare-origin-ca-root-certificates)
//! for a given algorithm."
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ```ignore
//! use pulumi_wasm_rust::Output;
//! use pulumi_wasm_rust::{add_export, pulumi_main};
//! #[pulumi_main]
//! fn test_main() -> Result<(), Error> {
//!     let example = get_origin_ca_root_certificate::invoke(
//!         GetOriginCaRootCertificateArgs::builder().algorithm("rsa").build_struct(),
//!     );
//! }
//! ```
//! <!--End PulumiCodeChooser -->

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct GetOriginCaRootCertificateArgs {
    /// The name of the algorithm used when creating an Origin CA certificate. Available values: `rsa`, `ecc`.
    #[builder(into)]
    pub algorithm: pulumi_wasm_rust::Output<String>,
}

pub struct GetOriginCaRootCertificateResult {
    /// The name of the algorithm used when creating an Origin CA certificate. Available values: `rsa`, `ecc`.
    pub algorithm: pulumi_wasm_rust::Output<String>,
    /// The Origin CA root certificate in PEM format.
    pub cert_pem: pulumi_wasm_rust::Output<String>,
    /// The provider-assigned unique ID for this managed resource.
    pub id: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn invoke(
    args: GetOriginCaRootCertificateArgs
) -> GetOriginCaRootCertificateResult {

    let result = crate::bindings::pulumi::cloudflare::get_origin_ca_root_certificate::invoke(
        &crate::bindings::pulumi::cloudflare::get_origin_ca_root_certificate::Args {
                algorithm: &args.algorithm.get_inner(),
        }
    );

    GetOriginCaRootCertificateResult {
        algorithm: crate::into_domain(result.algorithm),
        cert_pem: crate::into_domain(result.cert_pem),
        id: crate::into_domain(result.id),
    }
}
