pub struct AccessIdentityProviderArgs {
    pub account_id: pulumi_wasm_rust::Output<Option<String>>,
    pub configs: pulumi_wasm_rust::Output<Option<Vec<crate::types::AccessIdentityProviderConfig>>>,
    pub name: pulumi_wasm_rust::Output<String>,
    pub scim_configs:
        pulumi_wasm_rust::Output<Option<Vec<crate::types::AccessIdentityProviderScimConfig>>>,
    pub type_: pulumi_wasm_rust::Output<String>,
    pub zone_id: pulumi_wasm_rust::Output<Option<String>>,
}

pub struct AccessIdentityProviderResult {
    pub account_id: pulumi_wasm_rust::Output<Option<String>>,
    pub configs: pulumi_wasm_rust::Output<Vec<crate::types::AccessIdentityProviderConfig>>,
    pub name: pulumi_wasm_rust::Output<String>,
    pub scim_configs: pulumi_wasm_rust::Output<Vec<crate::types::AccessIdentityProviderScimConfig>>,
    pub type_: pulumi_wasm_rust::Output<String>,
    pub zone_id: pulumi_wasm_rust::Output<Option<String>>,
}

pub fn create(name: &str, args: AccessIdentityProviderArgs) -> AccessIdentityProviderResult {
    let result = crate::bindings::pulumi::cloudflare::access_identity_provider::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::access_identity_provider::Args {
            account_id: args.account_id.get_inner(),
            configs: args.configs.get_inner(),
            name: args.name.get_inner(),
            scim_configs: args.scim_configs.get_inner(),
            type_: args.type_.get_inner(),
            zone_id: args.zone_id.get_inner(),
        },
    );

    AccessIdentityProviderResult {
        account_id: crate::into_domain(result.account_id),
        configs: crate::into_domain(result.configs),
        name: crate::into_domain(result.name),
        scim_configs: crate::into_domain(result.scim_configs),
        type_: crate::into_domain(result.type_),
        zone_id: crate::into_domain(result.zone_id),
    }
}
