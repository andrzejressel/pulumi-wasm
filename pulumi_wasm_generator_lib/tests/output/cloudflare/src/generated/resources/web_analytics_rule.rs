#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct WebAnalyticsRuleArgs {
    /// The account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The host to apply the rule to.
    #[builder(into)]
    pub host: pulumi_wasm_rust::Output<String>,
    /// Whether the rule includes or excludes the matched traffic from being measured in Web Analytics.
    #[builder(into)]
    pub inclusive: pulumi_wasm_rust::Output<bool>,
    /// Whether the rule is paused or not.
    #[builder(into)]
    pub is_paused: pulumi_wasm_rust::Output<bool>,
    /// A list of paths to apply the rule to.
    #[builder(into)]
    pub paths: pulumi_wasm_rust::Output<Vec<String>>,
    /// The Web Analytics ruleset id. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub ruleset_id: pulumi_wasm_rust::Output<String>,
}
pub struct WebAnalyticsRuleResult {
    /// The account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The host to apply the rule to.
    pub host: pulumi_wasm_rust::Output<String>,
    /// Whether the rule includes or excludes the matched traffic from being measured in Web Analytics.
    pub inclusive: pulumi_wasm_rust::Output<bool>,
    /// Whether the rule is paused or not.
    pub is_paused: pulumi_wasm_rust::Output<bool>,
    /// A list of paths to apply the rule to.
    pub paths: pulumi_wasm_rust::Output<Vec<String>>,
    /// The Web Analytics ruleset id. **Modifying this attribute will force creation of a new resource.**
    pub ruleset_id: pulumi_wasm_rust::Output<String>,
}
///
/// Registers a new resource with the given unique name and arguments
///
#[allow(non_snake_case, unused_imports)]
pub fn create(name: &str, args: WebAnalyticsRuleArgs) -> WebAnalyticsRuleResult {
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
    use pulumi_wasm_rust::Output;
    use std::collections::HashMap;
    let account_id_binding = args.account_id.get_inner();
    let host_binding = args.host.get_inner();
    let inclusive_binding = args.inclusive.get_inner();
    let is_paused_binding = args.is_paused.get_inner();
    let paths_binding = args.paths.get_inner();
    let ruleset_id_binding = args.ruleset_id.get_inner();
    let request = register_interface::RegisterResourceRequest {
        type_: "cloudflare:index/webAnalyticsRule:WebAnalyticsRule".into(),
        name: name.to_string(),
        object: Vec::from([
            register_interface::ObjectField {
                name: "accountId".into(),
                value: &account_id_binding,
            },
            register_interface::ObjectField {
                name: "host".into(),
                value: &host_binding,
            },
            register_interface::ObjectField {
                name: "inclusive".into(),
                value: &inclusive_binding,
            },
            register_interface::ObjectField {
                name: "isPaused".into(),
                value: &is_paused_binding,
            },
            register_interface::ObjectField {
                name: "paths".into(),
                value: &paths_binding,
            },
            register_interface::ObjectField {
                name: "rulesetId".into(),
                value: &ruleset_id_binding,
            },
        ]),
        results: vec![
            register_interface::ResultField { name : "accountId".into() },
            register_interface::ResultField { name : "host".into() },
            register_interface::ResultField { name : "inclusive".into() },
            register_interface::ResultField { name : "isPaused".into() },
            register_interface::ResultField { name : "paths".into() },
            register_interface::ResultField { name : "rulesetId".into() },
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
    WebAnalyticsRuleResult {
        account_id: into_domain(hashmap.remove("accountId").unwrap()),
        host: into_domain(hashmap.remove("host").unwrap()),
        inclusive: into_domain(hashmap.remove("inclusive").unwrap()),
        is_paused: into_domain(hashmap.remove("isPaused").unwrap()),
        paths: into_domain(hashmap.remove("paths").unwrap()),
        ruleset_id: into_domain(hashmap.remove("rulesetId").unwrap()),
    }
}
