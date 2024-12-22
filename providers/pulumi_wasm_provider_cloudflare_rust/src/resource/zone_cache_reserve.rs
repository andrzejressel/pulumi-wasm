//! Provides a Cloudflare Cache Reserve resource. Cache Reserve can
//! increase cache lifetimes by automatically storing all cacheable
//! files in Cloudflare's persistent object storage buckets.
//! 
//! Note: Using Cache Reserve without Tiered Cache is not recommended.
//! 
//! ## Example Usage
//! 
//! ```ignore
//! use pulumi_wasm_rust::Output;
//! use pulumi_wasm_rust::{add_export, pulumi_main};
//! #[pulumi_main]
//! fn test_main() -> Result<(), Error> {
//!     let example = zone_cache_reserve::create(
//!         "example",
//!         ZoneCacheReserveArgs::builder()
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
//! $ pulumi import cloudflare:index/zoneCacheReserve:ZoneCacheReserve example <zone_id>
//! ```
//! 

#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct ZoneCacheReserveArgs {
    /// Whether to enable or disable Cache Reserve support for a given zone.
    #[builder(into)]
    pub enabled: pulumi_wasm_rust::Output<bool>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct ZoneCacheReserveResult {
    /// Whether to enable or disable Cache Reserve support for a given zone.
    pub enabled: pulumi_wasm_rust::Output<bool>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(
    name: &str,
    args: ZoneCacheReserveArgs
) -> ZoneCacheReserveResult {

    let result = crate::bindings::pulumi::cloudflare::zone_cache_reserve::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::zone_cache_reserve::Args {
                enabled: &args.enabled.get_inner(),
                zone_id: &args.zone_id.get_inner(),
        }
    );

    ZoneCacheReserveResult {
        enabled: crate::into_domain(result.enabled),
        zone_id: crate::into_domain(result.zone_id),
    }
}
