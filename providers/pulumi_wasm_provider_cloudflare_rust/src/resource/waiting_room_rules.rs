pub struct WaitingRoomRulesArgs {
    pub rules: pulumi_wasm_rust::Output<Option<Vec<crate::types::WaitingRoomRulesRule>>>,
    pub waiting_room_id: pulumi_wasm_rust::Output<String>,
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct WaitingRoomRulesResult {
    pub rules: pulumi_wasm_rust::Output<Option<Vec<crate::types::WaitingRoomRulesRule>>>,
    pub waiting_room_id: pulumi_wasm_rust::Output<String>,
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub fn create(name: &str, args: WaitingRoomRulesArgs) -> WaitingRoomRulesResult {
    let result = crate::bindings::pulumi::cloudflare::waiting_room_rules::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::waiting_room_rules::Args {
            rules: args.rules.get_inner(),
            waiting_room_id: args.waiting_room_id.get_inner(),
            zone_id: args.zone_id.get_inner(),
        },
    );

    WaitingRoomRulesResult {
        rules: crate::into_domain(result.rules),
        waiting_room_id: crate::into_domain(result.waiting_room_id),
        zone_id: crate::into_domain(result.zone_id),
    }
}
