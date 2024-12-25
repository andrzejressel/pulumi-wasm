#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct CustomPagesArgs {
    /// The account identifier to target for the resource. Conflicts with `zone_id`.
    #[builder(into, default)]
    pub account_id: pulumi_wasm_rust::Output<Option<String>>,
    /// Managed state of the custom page. Available values: `default`, `customized`.
    #[builder(into, default)]
    pub state: pulumi_wasm_rust::Output<Option<String>>,
    /// The type of custom page you wish to update. Available values: `basic_challenge`, `waf_challenge`, `waf_block`, `ratelimit_block`, `country_challenge`, `ip_block`, `under_attack`, `500_errors`, `1000_errors`, `managed_challenge`.
    #[builder(into)]
    pub type_: pulumi_wasm_rust::Output<String>,
    /// URL of where the custom page source is located.
    #[builder(into)]
    pub url: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource. Conflicts with `account_id`.
    #[builder(into, default)]
    pub zone_id: pulumi_wasm_rust::Output<Option<String>>,
}
pub struct CustomPagesResult {
    /// The account identifier to target for the resource. Conflicts with `zone_id`.
    pub account_id: pulumi_wasm_rust::Output<Option<String>>,
    /// Managed state of the custom page. Available values: `default`, `customized`.
    pub state: pulumi_wasm_rust::Output<Option<String>>,
    /// The type of custom page you wish to update. Available values: `basic_challenge`, `waf_challenge`, `waf_block`, `ratelimit_block`, `country_challenge`, `ip_block`, `under_attack`, `500_errors`, `1000_errors`, `managed_challenge`.
    pub type_: pulumi_wasm_rust::Output<String>,
    /// URL of where the custom page source is located.
    pub url: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource. Conflicts with `account_id`.
    pub zone_id: pulumi_wasm_rust::Output<Option<String>>,
}
///
/// Registers a new resource with the given unique name and arguments
///
#[allow(non_snake_case, unused_imports)]
pub fn create(name: &str, args: CustomPagesArgs) -> CustomPagesResult {
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
    use pulumi_wasm_rust::Output;
    use std::collections::HashMap;
    let account_id_binding = args.account_id.get_inner();
    let state_binding = args.state.get_inner();
    let type__binding = args.type_.get_inner();
    let url_binding = args.url.get_inner();
    let zone_id_binding = args.zone_id.get_inner();
    let request = register_interface::RegisterResourceRequest {
        type_: "cloudflare:index/customPages:CustomPages".into(),
        name: name.to_string(),
        object: Vec::from([
            register_interface::ObjectField {
                name: "accountId".into(),
                value: &account_id_binding,
            },
            register_interface::ObjectField {
                name: "state".into(),
                value: &state_binding,
            },
            register_interface::ObjectField {
                name: "type".into(),
                value: &type__binding,
            },
            register_interface::ObjectField {
                name: "url".into(),
                value: &url_binding,
            },
            register_interface::ObjectField {
                name: "zoneId".into(),
                value: &zone_id_binding,
            },
        ]),
        results: vec![
            register_interface::ResultField { name : "accountId".into() },
            register_interface::ResultField { name : "state".into() },
            register_interface::ResultField { name : "type".into() },
            register_interface::ResultField { name : "url".into() },
            register_interface::ResultField { name : "zoneId".into() },
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
    CustomPagesResult {
        account_id: into_domain(hashmap.remove("accountId").unwrap()),
        state: into_domain(hashmap.remove("state").unwrap()),
        type_: into_domain(hashmap.remove("type").unwrap()),
        url: into_domain(hashmap.remove("url").unwrap()),
        zone_id: into_domain(hashmap.remove("zoneId").unwrap()),
    }
}
