pub struct WaitingRoomSettingsArgs {
    pub search_engine_crawler_bypass: pulumi_wasm_rust::Output<Option<bool>>,
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct WaitingRoomSettingsResult {
    pub search_engine_crawler_bypass: pulumi_wasm_rust::Output<Option<bool>>,
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub fn create(name: &str, args: WaitingRoomSettingsArgs) -> WaitingRoomSettingsResult {
    let result = crate::bindings::pulumi::cloudflare::waiting_room_settings::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::waiting_room_settings::Args {
            search_engine_crawler_bypass: args.search_engine_crawler_bypass.get_inner(),
            zone_id: args.zone_id.get_inner(),
        },
    );

    WaitingRoomSettingsResult {
        search_engine_crawler_bypass: crate::into_domain(result.search_engine_crawler_bypass),
        zone_id: crate::into_domain(result.zone_id),
    }
}
