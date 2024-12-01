//! Provides a resource, that manages Keyless certificates.
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ```yaml
//! resources:
//!   example:
//!     type: cloudflare:KeylessCertificate
//!     properties:
//!       zoneId: 0da42c8d2132a9ddaf714f9e7c920711
//!       bundleMethod: ubiquitous
//!       name: example.com Keyless SSL
//!       host: example.com
//!       port: 24008
//!       enabled: true
//!       certificate: '-----INSERT CERTIFICATE-----'
//! ```
//! <!--End PulumiCodeChooser -->
//! 
//! ## Import
//! 
//! ```sh
//! $ pulumi import cloudflare:index/keylessCertificate:KeylessCertificate example <zone_id>/<keyless_certificate_id>
//! ```
//! 

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct KeylessCertificateArgs {
    /// A ubiquitous bundle has the highest probability of being verified everywhere, even by clients using outdated or unusual trust stores. An optimal bundle uses the shortest chain and newest intermediates. And the force bundle verifies the chain, but does not otherwise modify it. Available values: `ubiquitous`, `optimal`, `force`. Defaults to `ubiquitous`. **Modifying this attribute will force creation of a new resource.**
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub bundle_method: pulumi_wasm_rust::Output<Option<String>>,
    /// The zone's SSL certificate or SSL certificate and intermediate(s). **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub certificate: pulumi_wasm_rust::Output<String>,
    /// Whether the KeyLess SSL is on.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
    /// The KeyLess SSL host.
    #[builder(into)]
    pub host: pulumi_wasm_rust::Output<String>,
    /// The KeyLess SSL name.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub name: pulumi_wasm_rust::Output<Option<String>>,
    /// The KeyLess SSL port used to communicate between Cloudflare and the client's KeyLess SSL server. Defaults to `24008`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub port: pulumi_wasm_rust::Output<Option<i32>>,
    /// The zone identifier to target for the resource.
    #[builder(into)]
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct KeylessCertificateResult {
    /// A ubiquitous bundle has the highest probability of being verified everywhere, even by clients using outdated or unusual trust stores. An optimal bundle uses the shortest chain and newest intermediates. And the force bundle verifies the chain, but does not otherwise modify it. Available values: `ubiquitous`, `optimal`, `force`. Defaults to `ubiquitous`. **Modifying this attribute will force creation of a new resource.**
    pub bundle_method: pulumi_wasm_rust::Output<Option<String>>,
    /// The zone's SSL certificate or SSL certificate and intermediate(s). **Modifying this attribute will force creation of a new resource.**
    pub certificate: pulumi_wasm_rust::Output<String>,
    /// Whether the KeyLess SSL is on.
    pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
    /// The KeyLess SSL host.
    pub host: pulumi_wasm_rust::Output<String>,
    /// The KeyLess SSL name.
    pub name: pulumi_wasm_rust::Output<Option<String>>,
    /// The KeyLess SSL port used to communicate between Cloudflare and the client's KeyLess SSL server. Defaults to `24008`.
    pub port: pulumi_wasm_rust::Output<Option<i32>>,
    /// Status of the KeyLess SSL.
    pub status: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource.
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: KeylessCertificateArgs) -> KeylessCertificateResult {

    let result = crate::bindings::pulumi::cloudflare::keyless_certificate::invoke(name, &crate::bindings::pulumi::cloudflare::keyless_certificate::Args {
        bundle_method: &args.bundle_method.get_inner(),
        certificate: &args.certificate.get_inner(),
        enabled: &args.enabled.get_inner(),
        host: &args.host.get_inner(),
        name: &args.name.get_inner(),
        port: &args.port.get_inner(),
        zone_id: &args.zone_id.get_inner(),
    });

    KeylessCertificateResult {
        bundle_method: crate::into_domain(result.bundle_method),
        certificate: crate::into_domain(result.certificate),
        enabled: crate::into_domain(result.enabled),
        host: crate::into_domain(result.host),
        name: crate::into_domain(result.name),
        port: crate::into_domain(result.port),
        status: crate::into_domain(result.status),
        zone_id: crate::into_domain(result.zone_id),
    }
}
