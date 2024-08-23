pub struct UrlNormalizationSettingsArgs {
    pub scope: pulumi_wasm_rust::Output<String>,
    pub type_: pulumi_wasm_rust::Output<String>,
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct UrlNormalizationSettingsResult {
    pub scope: pulumi_wasm_rust::Output<String>,
    pub type_: pulumi_wasm_rust::Output<String>,
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub fn create(name: &str, args: UrlNormalizationSettingsArgs) -> UrlNormalizationSettingsResult {
    let result = crate::bindings::pulumi::cloudflare::url_normalization_settings::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::url_normalization_settings::Args {
            scope: args.scope.get_inner(),
            type_: args.type_.get_inner(),
            zone_id: args.zone_id.get_inner(),
        },
    );

    UrlNormalizationSettingsResult {
        scope: crate::into_domain(result.scope),
        type_: crate::into_domain(result.type_),
        zone_id: crate::into_domain(result.zone_id),
    }
}
