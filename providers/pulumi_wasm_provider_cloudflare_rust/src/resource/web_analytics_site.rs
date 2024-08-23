pub struct WebAnalyticsSiteArgs {
    pub account_id: pulumi_wasm_rust::Output<String>,
    pub auto_install: pulumi_wasm_rust::Output<bool>,
    pub host: pulumi_wasm_rust::Output<Option<String>>,
    pub zone_tag: pulumi_wasm_rust::Output<Option<String>>,
}

pub struct WebAnalyticsSiteResult {
    pub account_id: pulumi_wasm_rust::Output<String>,
    pub auto_install: pulumi_wasm_rust::Output<bool>,
    pub host: pulumi_wasm_rust::Output<Option<String>>,
    pub ruleset_id: pulumi_wasm_rust::Output<String>,
    pub site_tag: pulumi_wasm_rust::Output<String>,
    pub site_token: pulumi_wasm_rust::Output<String>,
    pub snippet: pulumi_wasm_rust::Output<String>,
    pub zone_tag: pulumi_wasm_rust::Output<Option<String>>,
}

pub fn create(name: &str, args: WebAnalyticsSiteArgs) -> WebAnalyticsSiteResult {
    let result = crate::bindings::pulumi::cloudflare::web_analytics_site::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::web_analytics_site::Args {
            account_id: args.account_id.get_inner(),
            auto_install: args.auto_install.get_inner(),
            host: args.host.get_inner(),
            zone_tag: args.zone_tag.get_inner(),
        },
    );

    WebAnalyticsSiteResult {
        account_id: crate::into_domain(result.account_id),
        auto_install: crate::into_domain(result.auto_install),
        host: crate::into_domain(result.host),
        ruleset_id: crate::into_domain(result.ruleset_id),
        site_tag: crate::into_domain(result.site_tag),
        site_token: crate::into_domain(result.site_token),
        snippet: crate::into_domain(result.snippet),
        zone_tag: crate::into_domain(result.zone_tag),
    }
}
