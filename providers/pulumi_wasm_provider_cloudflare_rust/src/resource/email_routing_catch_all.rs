pub struct EmailRoutingCatchAllArgs {
    pub actions: pulumi_wasm_rust::Output<Vec<crate::types::EmailRoutingCatchAllAction>>,
    pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
    pub matchers: pulumi_wasm_rust::Output<Vec<crate::types::EmailRoutingCatchAllMatcher>>,
    pub name: pulumi_wasm_rust::Output<String>,
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct EmailRoutingCatchAllResult {
    pub actions: pulumi_wasm_rust::Output<Vec<crate::types::EmailRoutingCatchAllAction>>,
    pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
    pub matchers: pulumi_wasm_rust::Output<Vec<crate::types::EmailRoutingCatchAllMatcher>>,
    pub name: pulumi_wasm_rust::Output<String>,
    pub tag: pulumi_wasm_rust::Output<String>,
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub fn create(name: &str, args: EmailRoutingCatchAllArgs) -> EmailRoutingCatchAllResult {
    let result = crate::bindings::pulumi::cloudflare::email_routing_catch_all::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::email_routing_catch_all::Args {
            actions: args.actions.get_inner(),
            enabled: args.enabled.get_inner(),
            matchers: args.matchers.get_inner(),
            name: args.name.get_inner(),
            zone_id: args.zone_id.get_inner(),
        },
    );

    EmailRoutingCatchAllResult {
        actions: crate::into_domain(result.actions),
        enabled: crate::into_domain(result.enabled),
        matchers: crate::into_domain(result.matchers),
        name: crate::into_domain(result.name),
        tag: crate::into_domain(result.tag),
        zone_id: crate::into_domain(result.zone_id),
    }
}
