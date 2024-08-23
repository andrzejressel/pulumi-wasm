pub struct ZoneArgs {
    pub account_id: pulumi_wasm_rust::Output<String>,
    pub jump_start: pulumi_wasm_rust::Output<Option<bool>>,
    pub paused: pulumi_wasm_rust::Output<Option<bool>>,
    pub plan: pulumi_wasm_rust::Output<Option<String>>,
    pub type_: pulumi_wasm_rust::Output<Option<String>>,
    pub zone: pulumi_wasm_rust::Output<String>,
}

pub struct ZoneResult {
    pub account_id: pulumi_wasm_rust::Output<String>,
    pub jump_start: pulumi_wasm_rust::Output<Option<bool>>,
    pub meta: pulumi_wasm_rust::Output<std::collections::HashMap<String, bool>>,
    pub name_servers: pulumi_wasm_rust::Output<Vec<String>>,
    pub paused: pulumi_wasm_rust::Output<Option<bool>>,
    pub plan: pulumi_wasm_rust::Output<String>,
    pub status: pulumi_wasm_rust::Output<String>,
    pub type_: pulumi_wasm_rust::Output<Option<String>>,
    pub vanity_name_servers: pulumi_wasm_rust::Output<Vec<String>>,
    pub verification_key: pulumi_wasm_rust::Output<String>,
    pub zone: pulumi_wasm_rust::Output<String>,
}

pub fn create(name: &str, args: ZoneArgs) -> ZoneResult {
    let result = crate::bindings::pulumi::cloudflare::zone::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::zone::Args {
            account_id: args.account_id.get_inner(),
            jump_start: args.jump_start.get_inner(),
            paused: args.paused.get_inner(),
            plan: args.plan.get_inner(),
            type_: args.type_.get_inner(),
            zone: args.zone.get_inner(),
        },
    );

    ZoneResult {
        account_id: crate::into_domain(result.account_id),
        jump_start: crate::into_domain(result.jump_start),
        meta: crate::into_domain(result.meta),
        name_servers: crate::into_domain(result.name_servers),
        paused: crate::into_domain(result.paused),
        plan: crate::into_domain(result.plan),
        status: crate::into_domain(result.status),
        type_: crate::into_domain(result.type_),
        vanity_name_servers: crate::into_domain(result.vanity_name_servers),
        verification_key: crate::into_domain(result.verification_key),
        zone: crate::into_domain(result.zone),
    }
}
