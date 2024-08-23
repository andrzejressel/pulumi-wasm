pub struct AddressMapArgs {
    pub account_id: pulumi_wasm_rust::Output<String>,
    pub default_sni: pulumi_wasm_rust::Output<Option<String>>,
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    pub enabled: pulumi_wasm_rust::Output<bool>,
    pub ips: pulumi_wasm_rust::Output<Option<Vec<crate::types::AddressMapIp>>>,
    pub memberships: pulumi_wasm_rust::Output<Option<Vec<crate::types::AddressMapMembership>>>,
}

pub struct AddressMapResult {
    pub account_id: pulumi_wasm_rust::Output<String>,
    pub can_delete: pulumi_wasm_rust::Output<bool>,
    pub can_modify_ips: pulumi_wasm_rust::Output<bool>,
    pub default_sni: pulumi_wasm_rust::Output<Option<String>>,
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    pub enabled: pulumi_wasm_rust::Output<bool>,
    pub ips: pulumi_wasm_rust::Output<Option<Vec<crate::types::AddressMapIp>>>,
    pub memberships: pulumi_wasm_rust::Output<Option<Vec<crate::types::AddressMapMembership>>>,
}

pub fn create(name: &str, args: AddressMapArgs) -> AddressMapResult {
    let result = crate::bindings::pulumi::cloudflare::address_map::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::address_map::Args {
            account_id: args.account_id.get_inner(),
            default_sni: args.default_sni.get_inner(),
            description: args.description.get_inner(),
            enabled: args.enabled.get_inner(),
            ips: args.ips.get_inner(),
            memberships: args.memberships.get_inner(),
        },
    );

    AddressMapResult {
        account_id: crate::into_domain(result.account_id),
        can_delete: crate::into_domain(result.can_delete),
        can_modify_ips: crate::into_domain(result.can_modify_ips),
        default_sni: crate::into_domain(result.default_sni),
        description: crate::into_domain(result.description),
        enabled: crate::into_domain(result.enabled),
        ips: crate::into_domain(result.ips),
        memberships: crate::into_domain(result.memberships),
    }
}
