//! Provides a resource, that manages Cloudflare Tiered Cache settings.
//! This allows you to adjust topologies for your zone.
//! 
//! ## Example Usage
//! 
//! ```ignore
//! use pulumi_wasm_rust::Output;
//! use pulumi_wasm_rust::{add_export, pulumi_main};
//! #[pulumi_main]
//! fn test_main() -> Result<(), Error> {
//!     let example = tiered_cache::create(
//!         "example",
//!         TieredCacheArgs::builder()
//!             .cache_type("smart")
//!             .zone_id("0da42c8d2132a9ddaf714f9e7c920711")
//!             .build_struct(),
//!     );
//! }
//! ```

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct TieredCacheArgs {
    /// The typed of tiered cache to utilize on the zone. Available values: `generic`, `smart`, `off`.
    #[builder(into)]
    pub cache_type: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct TieredCacheResult {
    /// The typed of tiered cache to utilize on the zone. Available values: `generic`, `smart`, `off`.
    pub cache_type: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: TieredCacheArgs) -> TieredCacheResult {

    let result = crate::bindings::pulumi::cloudflare::tiered_cache::invoke(name, &crate::bindings::pulumi::cloudflare::tiered_cache::Args {
        cache_type: &args.cache_type.get_inner(),
        zone_id: &args.zone_id.get_inner(),
    });

    TieredCacheResult {
        cache_type: crate::into_domain(result.cache_type),
        zone_id: crate::into_domain(result.zone_id),
    }
}

