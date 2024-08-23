pub struct EmailRoutingSettingsArgs {
    pub enabled: pulumi_wasm_rust::Output<bool>,
    pub skip_wizard: pulumi_wasm_rust::Output<Option<bool>>,
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct EmailRoutingSettingsResult {
    pub created: pulumi_wasm_rust::Output<String>,
    pub enabled: pulumi_wasm_rust::Output<bool>,
    pub modified: pulumi_wasm_rust::Output<String>,
    pub name: pulumi_wasm_rust::Output<String>,
    pub skip_wizard: pulumi_wasm_rust::Output<bool>,
    pub status: pulumi_wasm_rust::Output<String>,
    pub tag: pulumi_wasm_rust::Output<String>,
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub fn create(name: &str, args: EmailRoutingSettingsArgs) -> EmailRoutingSettingsResult {
    let result = crate::bindings::pulumi::cloudflare::email_routing_settings::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::email_routing_settings::Args {
            enabled: args.enabled.get_inner(),
            skip_wizard: args.skip_wizard.get_inner(),
            zone_id: args.zone_id.get_inner(),
        },
    );

    EmailRoutingSettingsResult {
        created: crate::into_domain(result.created),
        enabled: crate::into_domain(result.enabled),
        modified: crate::into_domain(result.modified),
        name: crate::into_domain(result.name),
        skip_wizard: crate::into_domain(result.skip_wizard),
        status: crate::into_domain(result.status),
        tag: crate::into_domain(result.tag),
        zone_id: crate::into_domain(result.zone_id),
    }
}
