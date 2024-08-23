pub struct AccessCustomPageArgs {
    pub account_id: pulumi_wasm_rust::Output<Option<String>>,
    pub app_count: pulumi_wasm_rust::Output<Option<i32>>,
    pub custom_html: pulumi_wasm_rust::Output<Option<String>>,
    pub name: pulumi_wasm_rust::Output<String>,
    pub type_: pulumi_wasm_rust::Output<String>,
    pub zone_id: pulumi_wasm_rust::Output<Option<String>>,
}

pub struct AccessCustomPageResult {
    pub account_id: pulumi_wasm_rust::Output<Option<String>>,
    pub app_count: pulumi_wasm_rust::Output<Option<i32>>,
    pub custom_html: pulumi_wasm_rust::Output<Option<String>>,
    pub name: pulumi_wasm_rust::Output<String>,
    pub type_: pulumi_wasm_rust::Output<String>,
    pub zone_id: pulumi_wasm_rust::Output<Option<String>>,
}

pub fn create(name: &str, args: AccessCustomPageArgs) -> AccessCustomPageResult {
    let result = crate::bindings::pulumi::cloudflare::access_custom_page::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::access_custom_page::Args {
            account_id: args.account_id.get_inner(),
            app_count: args.app_count.get_inner(),
            custom_html: args.custom_html.get_inner(),
            name: args.name.get_inner(),
            type_: args.type_.get_inner(),
            zone_id: args.zone_id.get_inner(),
        },
    );

    AccessCustomPageResult {
        account_id: crate::into_domain(result.account_id),
        app_count: crate::into_domain(result.app_count),
        custom_html: crate::into_domain(result.custom_html),
        name: crate::into_domain(result.name),
        type_: crate::into_domain(result.type_),
        zone_id: crate::into_domain(result.zone_id),
    }
}
