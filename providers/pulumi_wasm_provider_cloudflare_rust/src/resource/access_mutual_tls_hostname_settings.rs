pub struct AccessMutualTlsHostnameSettingsArgs {
    pub account_id: pulumi_wasm_rust::Output<Option<String>>,
    pub settings:
        pulumi_wasm_rust::Output<Option<Vec<crate::types::AccessMutualTlsHostnameSettingsSetting>>>,
    pub zone_id: pulumi_wasm_rust::Output<Option<String>>,
}

pub struct AccessMutualTlsHostnameSettingsResult {
    pub account_id: pulumi_wasm_rust::Output<Option<String>>,
    pub settings:
        pulumi_wasm_rust::Output<Option<Vec<crate::types::AccessMutualTlsHostnameSettingsSetting>>>,
    pub zone_id: pulumi_wasm_rust::Output<Option<String>>,
}

pub fn create(
    name: &str,
    args: AccessMutualTlsHostnameSettingsArgs,
) -> AccessMutualTlsHostnameSettingsResult {
    let result = crate::bindings::pulumi::cloudflare::access_mutual_tls_hostname_settings::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::access_mutual_tls_hostname_settings::Args {
            account_id: args.account_id.get_inner(),
            settings: args.settings.get_inner(),
            zone_id: args.zone_id.get_inner(),
        },
    );

    AccessMutualTlsHostnameSettingsResult {
        account_id: crate::into_domain(result.account_id),
        settings: crate::into_domain(result.settings),
        zone_id: crate::into_domain(result.zone_id),
    }
}
