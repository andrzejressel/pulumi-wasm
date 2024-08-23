pub struct AccessKeysConfigurationArgs {
    pub account_id: pulumi_wasm_rust::Output<String>,
    pub key_rotation_interval_days: pulumi_wasm_rust::Output<Option<i32>>,
}

pub struct AccessKeysConfigurationResult {
    pub account_id: pulumi_wasm_rust::Output<String>,
    pub key_rotation_interval_days: pulumi_wasm_rust::Output<i32>,
}

pub fn create(name: &str, args: AccessKeysConfigurationArgs) -> AccessKeysConfigurationResult {
    let result = crate::bindings::pulumi::cloudflare::access_keys_configuration::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::access_keys_configuration::Args {
            account_id: args.account_id.get_inner(),
            key_rotation_interval_days: args.key_rotation_interval_days.get_inner(),
        },
    );

    AccessKeysConfigurationResult {
        account_id: crate::into_domain(result.account_id),
        key_rotation_interval_days: crate::into_domain(result.key_rotation_interval_days),
    }
}
