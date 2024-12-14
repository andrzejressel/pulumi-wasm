//! Provides a Cloudflare Authenticated Origin Pulls resource. A `cloudflare.AuthenticatedOriginPulls`
//! resource is required to use Per-Zone or Per-Hostname Authenticated
//! Origin Pulls.
//! 
//! ## Example Usage
//! 
//! ```ignore
//! use pulumi_wasm_rust::Output;
//! use pulumi_wasm_rust::{add_export, pulumi_main};
//! #[pulumi_main]
//! fn test_main() -> Result<(), Error> {
//!     let myAop = authenticated_origin_pulls::create(
//!         "myAop",
//!         AuthenticatedOriginPullsArgs::builder()
//!             .enabled(true)
//!             .zone_id("0da42c8d2132a9ddaf714f9e7c920711")
//!             .build_struct(),
//!     );
//!     let myPerHostnameAop = authenticated_origin_pulls::create(
//!         "myPerHostnameAop",
//!         AuthenticatedOriginPullsArgs::builder()
//!             .authenticated_origin_pulls_certificate("${myPerHostnameAopCert.id}")
//!             .enabled(true)
//!             .hostname("aop.example.com")
//!             .zone_id("0da42c8d2132a9ddaf714f9e7c920711")
//!             .build_struct(),
//!     );
//!     let myPerHostnameAopCert = authenticated_origin_pulls_certificate::create(
//!         "myPerHostnameAopCert",
//!         AuthenticatedOriginPullsCertificateArgs::builder()
//!             .certificate("-----INSERT CERTIFICATE-----")
//!             .private_key("-----INSERT PRIVATE KEY-----")
//!             .type_("per-hostname")
//!             .zone_id("0da42c8d2132a9ddaf714f9e7c920711")
//!             .build_struct(),
//!     );
//!     let myPerZoneAop = authenticated_origin_pulls::create(
//!         "myPerZoneAop",
//!         AuthenticatedOriginPullsArgs::builder()
//!             .authenticated_origin_pulls_certificate("${myPerZoneAopCert.id}")
//!             .enabled(true)
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

#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct AuthenticatedOriginPullsArgs {
    /// The ID of an uploaded Authenticated Origin Pulls certificate. If no hostname is provided, this certificate will be used zone wide as Per-Zone Authenticated Origin Pulls.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub authenticated_origin_pulls_certificate: pulumi_wasm_rust::Output<Option<String>>,
    /// Whether to enable Authenticated Origin Pulls on the given zone or hostname.
    #[builder(into)]
    pub enabled: pulumi_wasm_rust::Output<bool>,
    /// Specify a hostname to enable Per-Hostname Authenticated Origin Pulls on, using the provided certificate.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub hostname: pulumi_wasm_rust::Output<Option<String>>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
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
        authenticated_origin_pulls_certificate: &args.authenticated_origin_pulls_certificate.get_inner(),
        enabled: &args.enabled.get_inner(),
        hostname: &args.hostname.get_inner(),
        zone_id: &args.zone_id.get_inner(),
    });

    AuthenticatedOriginPullsResult {
        authenticated_origin_pulls_certificate: crate::into_domain(result.authenticated_origin_pulls_certificate),
        enabled: crate::into_domain(result.enabled),
        hostname: crate::into_domain(result.hostname),
        zone_id: crate::into_domain(result.zone_id),
    }
}
