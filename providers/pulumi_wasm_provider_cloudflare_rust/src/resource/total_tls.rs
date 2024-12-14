//! Provides a resource which manages Total TLS for a zone.
//! 
//! ## Example Usage
//! 
//! ```ignore
//! use pulumi_wasm_rust::Output;
//! use pulumi_wasm_rust::{add_export, pulumi_main};
//! #[pulumi_main]
//! fn test_main() -> Result<(), Error> {
//!     let example = total_tls::create(
//!         "example",
//!         TotalTlsArgs::builder()
//!             .certificate_authority("lets_encrypt")
//!             .enabled(true)
//!             .zone_id("0da42c8d2132a9ddaf714f9e7c920711")
//!             .build_struct(),
//!     );
//! }
//! ```
//! 
//! ## Import
//! 
//! ```sh
//! $ pulumi import cloudflare:index/totalTls:TotalTls example <zone_id>
//! ```
//! 

#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct TotalTlsArgs {
    /// The Certificate Authority that Total TLS certificates will be issued through. Available values: `google`, `lets_encrypt`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub certificate_authority: pulumi_wasm_rust::Output<Option<String>>,
    /// Enable Total TLS for the zone.
    #[builder(into)]
    pub enabled: pulumi_wasm_rust::Output<bool>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct TotalTlsResult {
    /// The Certificate Authority that Total TLS certificates will be issued through. Available values: `google`, `lets_encrypt`.
    pub certificate_authority: pulumi_wasm_rust::Output<Option<String>>,
    /// Enable Total TLS for the zone.
    pub enabled: pulumi_wasm_rust::Output<bool>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: TotalTlsArgs) -> TotalTlsResult {

    let result = crate::bindings::pulumi::cloudflare::total_tls::invoke(name, &crate::bindings::pulumi::cloudflare::total_tls::Args {
        certificate_authority: &args.certificate_authority.get_inner(),
        enabled: &args.enabled.get_inner(),
        zone_id: &args.zone_id.get_inner(),
    });

    TotalTlsResult {
        certificate_authority: crate::into_domain(result.certificate_authority),
        enabled: crate::into_domain(result.enabled),
        zone_id: crate::into_domain(result.zone_id),
    }
}
