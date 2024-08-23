pub struct BotManagementArgs {
    pub auto_update_model: pulumi_wasm_rust::Output<Option<bool>>,
    pub enable_js: pulumi_wasm_rust::Output<Option<bool>>,
    pub fight_mode: pulumi_wasm_rust::Output<Option<bool>>,
    pub optimize_wordpress: pulumi_wasm_rust::Output<Option<bool>>,
    pub sbfm_definitely_automated: pulumi_wasm_rust::Output<Option<String>>,
    pub sbfm_likely_automated: pulumi_wasm_rust::Output<Option<String>>,
    pub sbfm_static_resource_protection: pulumi_wasm_rust::Output<Option<bool>>,
    pub sbfm_verified_bots: pulumi_wasm_rust::Output<Option<String>>,
    pub suppress_session_score: pulumi_wasm_rust::Output<Option<bool>>,
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct BotManagementResult {
    pub auto_update_model: pulumi_wasm_rust::Output<Option<bool>>,
    pub enable_js: pulumi_wasm_rust::Output<Option<bool>>,
    pub fight_mode: pulumi_wasm_rust::Output<Option<bool>>,
    pub optimize_wordpress: pulumi_wasm_rust::Output<Option<bool>>,
    pub sbfm_definitely_automated: pulumi_wasm_rust::Output<Option<String>>,
    pub sbfm_likely_automated: pulumi_wasm_rust::Output<Option<String>>,
    pub sbfm_static_resource_protection: pulumi_wasm_rust::Output<Option<bool>>,
    pub sbfm_verified_bots: pulumi_wasm_rust::Output<Option<String>>,
    pub suppress_session_score: pulumi_wasm_rust::Output<Option<bool>>,
    pub using_latest_model: pulumi_wasm_rust::Output<bool>,
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub fn create(name: &str, args: BotManagementArgs) -> BotManagementResult {
    let result = crate::bindings::pulumi::cloudflare::bot_management::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::bot_management::Args {
            auto_update_model: args.auto_update_model.get_inner(),
            enable_js: args.enable_js.get_inner(),
            fight_mode: args.fight_mode.get_inner(),
            optimize_wordpress: args.optimize_wordpress.get_inner(),
            sbfm_definitely_automated: args.sbfm_definitely_automated.get_inner(),
            sbfm_likely_automated: args.sbfm_likely_automated.get_inner(),
            sbfm_static_resource_protection: args.sbfm_static_resource_protection.get_inner(),
            sbfm_verified_bots: args.sbfm_verified_bots.get_inner(),
            suppress_session_score: args.suppress_session_score.get_inner(),
            zone_id: args.zone_id.get_inner(),
        },
    );

    BotManagementResult {
        auto_update_model: crate::into_domain(result.auto_update_model),
        enable_js: crate::into_domain(result.enable_js),
        fight_mode: crate::into_domain(result.fight_mode),
        optimize_wordpress: crate::into_domain(result.optimize_wordpress),
        sbfm_definitely_automated: crate::into_domain(result.sbfm_definitely_automated),
        sbfm_likely_automated: crate::into_domain(result.sbfm_likely_automated),
        sbfm_static_resource_protection: crate::into_domain(result.sbfm_static_resource_protection),
        sbfm_verified_bots: crate::into_domain(result.sbfm_verified_bots),
        suppress_session_score: crate::into_domain(result.suppress_session_score),
        using_latest_model: crate::into_domain(result.using_latest_model),
        zone_id: crate::into_domain(result.zone_id),
    }
}
