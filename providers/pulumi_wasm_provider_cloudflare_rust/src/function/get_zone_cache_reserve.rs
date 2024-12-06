//! Provides a Cloudflare data source to look up Cache Reserve
//! status for a given zone.
//! 
//! ## Example Usage
//! 
//! ```yaml
//! variables:
//!   example:
//!     fn::invoke:
//!       Function: cloudflare:getZoneCacheReserve
//!       Arguments:
//!         zoneId: 0da42c8d2132a9ddaf714f9e7c920711
//! ```

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct GetZoneCacheReserveArgs {
    /// The zone identifier to target for the resource.
    #[builder(into)]
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct GetZoneCacheReserveResult {
    /// The status of Cache Reserve support.
    pub enabled: pulumi_wasm_rust::Output<bool>,
    /// The provider-assigned unique ID for this managed resource.
    pub id: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource.
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn invoke(
    args: GetZoneCacheReserveArgs
) -> GetZoneCacheReserveResult {

    let result = crate::bindings::pulumi::cloudflare::get_zone_cache_reserve::invoke(
        &crate::bindings::pulumi::cloudflare::get_zone_cache_reserve::Args {
                zone_id: &args.zone_id.get_inner(),
        }
    );

    GetZoneCacheReserveResult {
        enabled: crate::into_domain(result.enabled),
        id: crate::into_domain(result.id),
        zone_id: crate::into_domain(result.zone_id),
    }
}
