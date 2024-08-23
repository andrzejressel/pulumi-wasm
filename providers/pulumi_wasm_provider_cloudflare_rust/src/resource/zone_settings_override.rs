pub struct ZoneSettingsOverrideArgs {
    pub settings: pulumi_wasm_rust::Output<Option<crate::types::ZoneSettingsOverrideSettings>>,
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct ZoneSettingsOverrideResult {
    pub initial_settings:
        pulumi_wasm_rust::Output<Vec<crate::types::ZoneSettingsOverrideInitialSetting>>,
    pub initial_settings_read_at: pulumi_wasm_rust::Output<String>,
    pub readonly_settings: pulumi_wasm_rust::Output<Vec<String>>,
    pub settings: pulumi_wasm_rust::Output<crate::types::ZoneSettingsOverrideSettings>,
    pub zone_id: pulumi_wasm_rust::Output<String>,
    pub zone_status: pulumi_wasm_rust::Output<String>,
    pub zone_type: pulumi_wasm_rust::Output<String>,
}

pub fn create(name: &str, args: ZoneSettingsOverrideArgs) -> ZoneSettingsOverrideResult {
    let result = crate::bindings::pulumi::cloudflare::zone_settings_override::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::zone_settings_override::Args {
            settings: args.settings.get_inner(),
            zone_id: args.zone_id.get_inner(),
        },
    );

    ZoneSettingsOverrideResult {
        initial_settings: crate::into_domain(result.initial_settings),
        initial_settings_read_at: crate::into_domain(result.initial_settings_read_at),
        readonly_settings: crate::into_domain(result.readonly_settings),
        settings: crate::into_domain(result.settings),
        zone_id: crate::into_domain(result.zone_id),
        zone_status: crate::into_domain(result.zone_status),
        zone_type: crate::into_domain(result.zone_type),
    }
}
