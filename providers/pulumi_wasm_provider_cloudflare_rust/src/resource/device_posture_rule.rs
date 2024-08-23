pub struct DevicePostureRuleArgs {
    pub account_id: pulumi_wasm_rust::Output<String>,
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    pub expiration: pulumi_wasm_rust::Output<Option<String>>,
    pub inputs: pulumi_wasm_rust::Output<Option<Vec<crate::types::DevicePostureRuleInput>>>,
    pub matches: pulumi_wasm_rust::Output<Option<Vec<crate::types::DevicePostureRuleMatch>>>,
    pub name: pulumi_wasm_rust::Output<Option<String>>,
    pub schedule: pulumi_wasm_rust::Output<Option<String>>,
    pub type_: pulumi_wasm_rust::Output<String>,
}

pub struct DevicePostureRuleResult {
    pub account_id: pulumi_wasm_rust::Output<String>,
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    pub expiration: pulumi_wasm_rust::Output<Option<String>>,
    pub inputs: pulumi_wasm_rust::Output<Vec<crate::types::DevicePostureRuleInput>>,
    pub matches: pulumi_wasm_rust::Output<Option<Vec<crate::types::DevicePostureRuleMatch>>>,
    pub name: pulumi_wasm_rust::Output<Option<String>>,
    pub schedule: pulumi_wasm_rust::Output<Option<String>>,
    pub type_: pulumi_wasm_rust::Output<String>,
}

pub fn create(name: &str, args: DevicePostureRuleArgs) -> DevicePostureRuleResult {
    let result = crate::bindings::pulumi::cloudflare::device_posture_rule::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::device_posture_rule::Args {
            account_id: args.account_id.get_inner(),
            description: args.description.get_inner(),
            expiration: args.expiration.get_inner(),
            inputs: args.inputs.get_inner(),
            matches: args.matches.get_inner(),
            name: args.name.get_inner(),
            schedule: args.schedule.get_inner(),
            type_: args.type_.get_inner(),
        },
    );

    DevicePostureRuleResult {
        account_id: crate::into_domain(result.account_id),
        description: crate::into_domain(result.description),
        expiration: crate::into_domain(result.expiration),
        inputs: crate::into_domain(result.inputs),
        matches: crate::into_domain(result.matches),
        name: crate::into_domain(result.name),
        schedule: crate::into_domain(result.schedule),
        type_: crate::into_domain(result.type_),
    }
}
