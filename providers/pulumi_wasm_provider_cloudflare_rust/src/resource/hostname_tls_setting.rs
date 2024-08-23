pub struct HostnameTlsSettingArgs {
    pub hostname: pulumi_wasm_rust::Output<String>,
    pub setting: pulumi_wasm_rust::Output<String>,
    pub value: pulumi_wasm_rust::Output<String>,
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct HostnameTlsSettingResult {
    pub created_at: pulumi_wasm_rust::Output<String>,
    pub hostname: pulumi_wasm_rust::Output<String>,
    pub setting: pulumi_wasm_rust::Output<String>,
    pub updated_at: pulumi_wasm_rust::Output<String>,
    pub value: pulumi_wasm_rust::Output<String>,
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub fn create(name: &str, args: HostnameTlsSettingArgs) -> HostnameTlsSettingResult {
    let result = crate::bindings::pulumi::cloudflare::hostname_tls_setting::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::hostname_tls_setting::Args {
            hostname: args.hostname.get_inner(),
            setting: args.setting.get_inner(),
            value: args.value.get_inner(),
            zone_id: args.zone_id.get_inner(),
        },
    );

    HostnameTlsSettingResult {
        created_at: crate::into_domain(result.created_at),
        hostname: crate::into_domain(result.hostname),
        setting: crate::into_domain(result.setting),
        updated_at: crate::into_domain(result.updated_at),
        value: crate::into_domain(result.value),
        zone_id: crate::into_domain(result.zone_id),
    }
}
