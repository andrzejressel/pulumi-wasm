pub struct AccessGroupArgs {
    pub account_id: pulumi_wasm_rust::Output<Option<String>>,
    pub excludes: pulumi_wasm_rust::Output<Option<Vec<crate::types::AccessGroupExclude>>>,
    pub includes: pulumi_wasm_rust::Output<Vec<crate::types::AccessGroupInclude>>,
    pub name: pulumi_wasm_rust::Output<String>,
    pub requires: pulumi_wasm_rust::Output<Option<Vec<crate::types::AccessGroupRequire>>>,
    pub zone_id: pulumi_wasm_rust::Output<Option<String>>,
}

pub struct AccessGroupResult {
    pub account_id: pulumi_wasm_rust::Output<Option<String>>,
    pub excludes: pulumi_wasm_rust::Output<Option<Vec<crate::types::AccessGroupExclude>>>,
    pub includes: pulumi_wasm_rust::Output<Vec<crate::types::AccessGroupInclude>>,
    pub name: pulumi_wasm_rust::Output<String>,
    pub requires: pulumi_wasm_rust::Output<Option<Vec<crate::types::AccessGroupRequire>>>,
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub fn create(name: &str, args: AccessGroupArgs) -> AccessGroupResult {
    let result = crate::bindings::pulumi::cloudflare::access_group::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::access_group::Args {
            account_id: args.account_id.get_inner(),
            excludes: args.excludes.get_inner(),
            includes: args.includes.get_inner(),
            name: args.name.get_inner(),
            requires: args.requires.get_inner(),
            zone_id: args.zone_id.get_inner(),
        },
    );

    AccessGroupResult {
        account_id: crate::into_domain(result.account_id),
        excludes: crate::into_domain(result.excludes),
        includes: crate::into_domain(result.includes),
        name: crate::into_domain(result.name),
        requires: crate::into_domain(result.requires),
        zone_id: crate::into_domain(result.zone_id),
    }
}
