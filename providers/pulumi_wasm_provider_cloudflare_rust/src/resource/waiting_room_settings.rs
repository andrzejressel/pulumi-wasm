//! Configure zone-wide settings for Cloudflare waiting rooms.
//! 
//! ## Example Usage
//! 
//! ```ignore
//! use pulumi_wasm_rust::Output;
//! use pulumi_wasm_rust::{add_export, pulumi_main};
//! #[pulumi_main]
//! fn test_main() -> Result<(), Error> {
//!     let example = waiting_room_settings::create(
//!         "example",
//!         WaitingRoomSettingsArgs::builder()
//!             .search_engine_crawler_bypass(true)
//!             .zone_id("0da42c8d2132a9ddaf714f9e7c920711")
//!             .build_struct(),
//!     );
//! }
//! ```
//! 
//! ## Import
//! 
//! ```sh
//! $ pulumi import cloudflare:index/waitingRoomSettings:WaitingRoomSettings example <zone_id>
//! ```
//! 

#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct WaitingRoomSettingsArgs {
    /// Whether to allow verified search engine crawlers to bypass all waiting rooms on this zone. Defaults to `false`.
    #[builder(into, default)]
    pub search_engine_crawler_bypass: pulumi_wasm_rust::Output<Option<bool>>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct WaitingRoomSettingsResult {
    /// Whether to allow verified search engine crawlers to bypass all waiting rooms on this zone. Defaults to `false`.
    pub search_engine_crawler_bypass: pulumi_wasm_rust::Output<Option<bool>>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(
    name: &str,
    args: WaitingRoomSettingsArgs
) -> WaitingRoomSettingsResult {

    let result = crate::bindings::pulumi::cloudflare::waiting_room_settings::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::waiting_room_settings::Args {
                search_engine_crawler_bypass: &args.search_engine_crawler_bypass.get_inner(),
                zone_id: &args.zone_id.get_inner(),
        }
    );

    WaitingRoomSettingsResult {
        search_engine_crawler_bypass: crate::into_domain(result.search_engine_crawler_bypass),
        zone_id: crate::into_domain(result.zone_id),
    }
}
