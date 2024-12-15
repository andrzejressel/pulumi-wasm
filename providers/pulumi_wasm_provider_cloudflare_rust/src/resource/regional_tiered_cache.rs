//! Instructs Cloudflare to check a regional hub data center on the way to your upper tier.
//! This can help improve performance for smart and custom tiered cache topologies.
//! 
//! ## Example Usage
//! 
//! ```ignore
//! use pulumi_wasm_rust::Output;
//! use pulumi_wasm_rust::{add_export, pulumi_main};
//! #[pulumi_main]
//! fn test_main() -> Result<(), Error> {
//!     let example = regional_tiered_cache::create(
//!         "example",
//!         RegionalTieredCacheArgs::builder()
//!             .value("on")
//!             .zone_id("0da42c8d2132a9ddaf714f9e7c920711")
//!             .build_struct(),
//!     );
//! }
//! ```
//! 
//! ## Import
//! 
//! ```sh
//! $ pulumi import cloudflare:index/regionalTieredCache:RegionalTieredCache example <zone_id>
//! ```
//! 

#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct RegionalTieredCacheArgs {
    /// Value of the Regional Tiered Cache zone setting.
    #[builder(into)]
    pub value: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource.
    #[builder(into)]
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct RegionalTieredCacheResult {
    /// Value of the Regional Tiered Cache zone setting.
    pub value: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource.
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: RegionalTieredCacheArgs) -> RegionalTieredCacheResult {

    let result = crate::bindings::pulumi::cloudflare::regional_tiered_cache::invoke(name, &crate::bindings::pulumi::cloudflare::regional_tiered_cache::Args {
        value: &args.value.get_inner(),
        zone_id: &args.zone_id.get_inner(),
    });

    RegionalTieredCacheResult {
        value: crate::into_domain(result.value),
        zone_id: crate::into_domain(result.zone_id),
    }
}
