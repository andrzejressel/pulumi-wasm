pub struct DeviceDexTestArgs {
    pub account_id: pulumi_wasm_rust::Output<String>,
    pub data: pulumi_wasm_rust::Output<crate::types::DeviceDexTestData>,
    pub description: pulumi_wasm_rust::Output<String>,
    pub enabled: pulumi_wasm_rust::Output<bool>,
    pub interval: pulumi_wasm_rust::Output<String>,
    pub name: pulumi_wasm_rust::Output<String>,
}

pub struct DeviceDexTestResult {
    pub account_id: pulumi_wasm_rust::Output<String>,
    pub created: pulumi_wasm_rust::Output<String>,
    pub data: pulumi_wasm_rust::Output<crate::types::DeviceDexTestData>,
    pub description: pulumi_wasm_rust::Output<String>,
    pub enabled: pulumi_wasm_rust::Output<bool>,
    pub interval: pulumi_wasm_rust::Output<String>,
    pub name: pulumi_wasm_rust::Output<String>,
    pub updated: pulumi_wasm_rust::Output<String>,
}

pub fn create(name: &str, args: DeviceDexTestArgs) -> DeviceDexTestResult {
    let result = crate::bindings::pulumi::cloudflare::device_dex_test::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::device_dex_test::Args {
            account_id: args.account_id.get_inner(),
            data: args.data.get_inner(),
            description: args.description.get_inner(),
            enabled: args.enabled.get_inner(),
            interval: args.interval.get_inner(),
            name: args.name.get_inner(),
        },
    );

    DeviceDexTestResult {
        account_id: crate::into_domain(result.account_id),
        created: crate::into_domain(result.created),
        data: crate::into_domain(result.data),
        description: crate::into_domain(result.description),
        enabled: crate::into_domain(result.enabled),
        interval: crate::into_domain(result.interval),
        name: crate::into_domain(result.name),
        updated: crate::into_domain(result.updated),
    }
}
