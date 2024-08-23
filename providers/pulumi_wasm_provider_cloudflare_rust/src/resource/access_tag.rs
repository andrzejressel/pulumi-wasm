pub struct AccessTagArgs {
    pub account_id: pulumi_wasm_rust::Output<Option<String>>,
    pub app_count: pulumi_wasm_rust::Output<Option<i32>>,
    pub name: pulumi_wasm_rust::Output<String>,
    pub zone_id: pulumi_wasm_rust::Output<Option<String>>,
}

pub struct AccessTagResult {
    pub account_id: pulumi_wasm_rust::Output<Option<String>>,
    pub app_count: pulumi_wasm_rust::Output<i32>,
    pub name: pulumi_wasm_rust::Output<String>,
    pub zone_id: pulumi_wasm_rust::Output<Option<String>>,
}

pub fn create(name: &str, args: AccessTagArgs) -> AccessTagResult {
    let result = crate::bindings::pulumi::cloudflare::access_tag::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::access_tag::Args {
            account_id: args.account_id.get_inner(),
            app_count: args.app_count.get_inner(),
            name: args.name.get_inner(),
            zone_id: args.zone_id.get_inner(),
        },
    );

    AccessTagResult {
        account_id: crate::into_domain(result.account_id),
        app_count: crate::into_domain(result.app_count),
        name: crate::into_domain(result.name),
        zone_id: crate::into_domain(result.zone_id),
    }
}
