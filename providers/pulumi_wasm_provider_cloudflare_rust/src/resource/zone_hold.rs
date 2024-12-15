//! Provides a Cloudflare Zone Hold resource that prevents adding
//! the hostname to another account for use.
//! 
//! ## Example Usage
//! 
//! ```ignore
//! use pulumi_wasm_rust::Output;
//! use pulumi_wasm_rust::{add_export, pulumi_main};
//! #[pulumi_main]
//! fn test_main() -> Result<(), Error> {
//!     let example = zone_hold::create(
//!         "example",
//!         ZoneHoldArgs::builder()
//!             .hold(true)
//!             .zone_id("0da42c8d2132a9ddaf714f9e7c920711")
//!             .build_struct(),
//!     );
//! }
//! ```
//! 
//! ## Import
//! 
//! ```sh
//! $ pulumi import cloudflare:index/zoneHold:ZoneHold example <zone_id>
//! ```
//! 

#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct ZoneHoldArgs {
    /// Enablement status of the zone hold.
    #[builder(into)]
    pub hold: pulumi_wasm_rust::Output<bool>,
    /// The RFC3339 compatible timestamp when to automatically re-enable the zone hold.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub hold_after: pulumi_wasm_rust::Output<Option<String>>,
    /// Whether to extend to block any subdomain of the given zone.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub include_subdomains: pulumi_wasm_rust::Output<Option<bool>>,
    /// The zone identifier to target for the resource.
    #[builder(into)]
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct ZoneHoldResult {
    /// Enablement status of the zone hold.
    pub hold: pulumi_wasm_rust::Output<bool>,
    /// The RFC3339 compatible timestamp when to automatically re-enable the zone hold.
    pub hold_after: pulumi_wasm_rust::Output<String>,
    /// Whether to extend to block any subdomain of the given zone.
    pub include_subdomains: pulumi_wasm_rust::Output<Option<bool>>,
    /// The zone identifier to target for the resource.
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: ZoneHoldArgs) -> ZoneHoldResult {

    let result = crate::bindings::pulumi::cloudflare::zone_hold::invoke(name, &crate::bindings::pulumi::cloudflare::zone_hold::Args {
        hold: &args.hold.get_inner(),
        hold_after: &args.hold_after.get_inner(),
        include_subdomains: &args.include_subdomains.get_inner(),
        zone_id: &args.zone_id.get_inner(),
    });

    ZoneHoldResult {
        hold: crate::into_domain(result.hold),
        hold_after: crate::into_domain(result.hold_after),
        include_subdomains: crate::into_domain(result.include_subdomains),
        zone_id: crate::into_domain(result.zone_id),
    }
}
