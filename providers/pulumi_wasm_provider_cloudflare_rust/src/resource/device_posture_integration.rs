pub struct DevicePostureIntegrationArgs {
    pub account_id: pulumi_wasm_rust::Output<String>,
    pub configs:
        pulumi_wasm_rust::Output<Option<Vec<crate::types::DevicePostureIntegrationConfig>>>,
    pub identifier: pulumi_wasm_rust::Output<Option<String>>,
    pub interval: pulumi_wasm_rust::Output<Option<String>>,
    pub name: pulumi_wasm_rust::Output<String>,
    pub type_: pulumi_wasm_rust::Output<String>,
}

pub struct DevicePostureIntegrationResult {
    pub account_id: pulumi_wasm_rust::Output<String>,
    pub configs:
        pulumi_wasm_rust::Output<Option<Vec<crate::types::DevicePostureIntegrationConfig>>>,
    pub identifier: pulumi_wasm_rust::Output<Option<String>>,
    pub interval: pulumi_wasm_rust::Output<Option<String>>,
    pub name: pulumi_wasm_rust::Output<String>,
    pub type_: pulumi_wasm_rust::Output<String>,
}

pub fn create(name: &str, args: DevicePostureIntegrationArgs) -> DevicePostureIntegrationResult {
    let result = crate::bindings::pulumi::cloudflare::device_posture_integration::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::device_posture_integration::Args {
            account_id: args.account_id.get_inner(),
            configs: args.configs.get_inner(),
            identifier: args.identifier.get_inner(),
            interval: args.interval.get_inner(),
            name: args.name.get_inner(),
            type_: args.type_.get_inner(),
        },
    );

    DevicePostureIntegrationResult {
        account_id: crate::into_domain(result.account_id),
        configs: crate::into_domain(result.configs),
        identifier: crate::into_domain(result.identifier),
        interval: crate::into_domain(result.interval),
        name: crate::into_domain(result.name),
        type_: crate::into_domain(result.type_),
    }
}
