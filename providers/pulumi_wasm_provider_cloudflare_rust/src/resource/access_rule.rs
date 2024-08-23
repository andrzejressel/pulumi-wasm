pub struct AccessRuleArgs {
    pub account_id: pulumi_wasm_rust::Output<Option<String>>,
    pub configuration: pulumi_wasm_rust::Output<crate::types::AccessRuleConfiguration>,
    pub mode: pulumi_wasm_rust::Output<String>,
    pub notes: pulumi_wasm_rust::Output<Option<String>>,
    pub zone_id: pulumi_wasm_rust::Output<Option<String>>,
}

pub struct AccessRuleResult {
    pub account_id: pulumi_wasm_rust::Output<String>,
    pub configuration: pulumi_wasm_rust::Output<crate::types::AccessRuleConfiguration>,
    pub mode: pulumi_wasm_rust::Output<String>,
    pub notes: pulumi_wasm_rust::Output<Option<String>>,
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub fn create(name: &str, args: AccessRuleArgs) -> AccessRuleResult {
    let result = crate::bindings::pulumi::cloudflare::access_rule::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::access_rule::Args {
            account_id: args.account_id.get_inner(),
            configuration: args.configuration.get_inner(),
            mode: args.mode.get_inner(),
            notes: args.notes.get_inner(),
            zone_id: args.zone_id.get_inner(),
        },
    );

    AccessRuleResult {
        account_id: crate::into_domain(result.account_id),
        configuration: crate::into_domain(result.configuration),
        mode: crate::into_domain(result.mode),
        notes: crate::into_domain(result.notes),
        zone_id: crate::into_domain(result.zone_id),
    }
}
