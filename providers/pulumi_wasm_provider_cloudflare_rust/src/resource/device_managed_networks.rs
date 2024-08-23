pub struct DeviceManagedNetworksArgs {
    pub account_id: pulumi_wasm_rust::Output<String>,
    pub config: pulumi_wasm_rust::Output<crate::types::DeviceManagedNetworksConfig>,
    pub name: pulumi_wasm_rust::Output<String>,
    pub type_: pulumi_wasm_rust::Output<String>,
}

pub struct DeviceManagedNetworksResult {
    pub account_id: pulumi_wasm_rust::Output<String>,
    pub config: pulumi_wasm_rust::Output<crate::types::DeviceManagedNetworksConfig>,
    pub name: pulumi_wasm_rust::Output<String>,
    pub type_: pulumi_wasm_rust::Output<String>,
}

pub fn create(name: &str, args: DeviceManagedNetworksArgs) -> DeviceManagedNetworksResult {
    let result = crate::bindings::pulumi::cloudflare::device_managed_networks::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::device_managed_networks::Args {
            account_id: args.account_id.get_inner(),
            config: args.config.get_inner(),
            name: args.name.get_inner(),
            type_: args.type_.get_inner(),
        },
    );

    DeviceManagedNetworksResult {
        account_id: crate::into_domain(result.account_id),
        config: crate::into_domain(result.config),
        name: crate::into_domain(result.name),
        type_: crate::into_domain(result.type_),
    }
}
