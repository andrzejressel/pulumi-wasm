#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct WebAnalyticsSiteArgs {
    /// The account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// Whether Cloudflare will automatically inject the JavaScript snippet for orange-clouded sites. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub auto_install: pulumi_wasm_rust::Output<bool>,
    /// The hostname to use for gray-clouded sites. Must provide only one of `zone_tag`. **Modifying this attribute will force creation of a new resource.**
    #[builder(into, default)]
    pub host: pulumi_wasm_rust::Output<Option<String>>,
    /// The zone identifier for orange-clouded sites. Must provide only one of `host`. **Modifying this attribute will force creation of a new resource.**
    #[builder(into, default)]
    pub zone_tag: pulumi_wasm_rust::Output<Option<String>>,
}
pub struct WebAnalyticsSiteResult {
    /// The account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// Whether Cloudflare will automatically inject the JavaScript snippet for orange-clouded sites. **Modifying this attribute will force creation of a new resource.**
    pub auto_install: pulumi_wasm_rust::Output<bool>,
    /// The hostname to use for gray-clouded sites. Must provide only one of `zone_tag`. **Modifying this attribute will force creation of a new resource.**
    pub host: pulumi_wasm_rust::Output<Option<String>>,
    /// The ID for the ruleset associated to this Web Analytics Site.
    pub ruleset_id: pulumi_wasm_rust::Output<String>,
    /// The Web Analytics site tag.
    pub site_tag: pulumi_wasm_rust::Output<String>,
    /// The token for the Web Analytics site.
    pub site_token: pulumi_wasm_rust::Output<String>,
    /// The encoded JS snippet to add to your site's HTML page if auto_install is false.
    pub snippet: pulumi_wasm_rust::Output<String>,
    /// The zone identifier for orange-clouded sites. Must provide only one of `host`. **Modifying this attribute will force creation of a new resource.**
    pub zone_tag: pulumi_wasm_rust::Output<Option<String>>,
}
///
/// Registers a new resource with the given unique name and arguments
///
#[allow(non_snake_case, unused_imports)]
pub fn create(name: &str, args: WebAnalyticsSiteArgs) -> WebAnalyticsSiteResult {
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
    use pulumi_wasm_rust::Output;
    use std::collections::HashMap;
    let account_id_binding = args.account_id.get_inner();
    let auto_install_binding = args.auto_install.get_inner();
    let host_binding = args.host.get_inner();
    let zone_tag_binding = args.zone_tag.get_inner();
    let request = register_interface::RegisterResourceRequest {
        type_: "cloudflare:index/webAnalyticsSite:WebAnalyticsSite".into(),
        name: name.to_string(),
        object: Vec::from([
            register_interface::ObjectField {
                name: "accountId".into(),
                value: &account_id_binding,
            },
            register_interface::ObjectField {
                name: "autoInstall".into(),
                value: &auto_install_binding,
            },
            register_interface::ObjectField {
                name: "host".into(),
                value: &host_binding,
            },
            register_interface::ObjectField {
                name: "zoneTag".into(),
                value: &zone_tag_binding,
            },
        ]),
        results: vec![
            register_interface::ResultField { name : "accountId".into() },
            register_interface::ResultField { name : "autoInstall".into() },
            register_interface::ResultField { name : "host".into() },
            register_interface::ResultField { name : "rulesetId".into() },
            register_interface::ResultField { name : "siteTag".into() },
            register_interface::ResultField { name : "siteToken".into() },
            register_interface::ResultField { name : "snippet".into() },
            register_interface::ResultField { name : "zoneTag".into() },
        ],
    };
    fn into_domain<F: serde::Serialize>(output: WitOutput) -> Output<F> {
        unsafe { Output::<F>::new_from_handle(output) }
    }
    let o = register_interface::register(&request);
    let mut hashmap: HashMap<String, _> = o
        .fields
        .into_iter()
        .map(|f| (f.name, f.output))
        .collect();
    WebAnalyticsSiteResult {
        account_id: into_domain(hashmap.remove("accountId").unwrap()),
        auto_install: into_domain(hashmap.remove("autoInstall").unwrap()),
        host: into_domain(hashmap.remove("host").unwrap()),
        ruleset_id: into_domain(hashmap.remove("rulesetId").unwrap()),
        site_tag: into_domain(hashmap.remove("siteTag").unwrap()),
        site_token: into_domain(hashmap.remove("siteToken").unwrap()),
        snippet: into_domain(hashmap.remove("snippet").unwrap()),
        zone_tag: into_domain(hashmap.remove("zoneTag").unwrap()),
    }
}