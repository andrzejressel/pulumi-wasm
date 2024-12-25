#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct PageRuleArgs {
    /// The actions taken by the page rule, options given below.
    #[builder(into)]
    pub actions: pulumi_wasm_rust::Output<super::types::PageRuleActions>,
    /// The priority of the page rule among others for this target, the higher the number the higher the priority as per [API documentation](https://api.cloudflare.com/#page-rules-for-a-zone-create-page-rule).
    #[builder(into, default)]
    pub priority: pulumi_wasm_rust::Output<Option<i32>>,
    /// Whether the page rule is active or disabled.
    #[builder(into, default)]
    pub status: pulumi_wasm_rust::Output<Option<String>>,
    /// The URL pattern to target with the page rule.
    #[builder(into)]
    pub target: pulumi_wasm_rust::Output<String>,
    /// The DNS zone ID to which the page rule should be added.
    #[builder(into)]
    pub zone_id: pulumi_wasm_rust::Output<String>,
}
pub struct PageRuleResult {
    /// The actions taken by the page rule, options given below.
    pub actions: pulumi_wasm_rust::Output<super::types::PageRuleActions>,
    /// The priority of the page rule among others for this target, the higher the number the higher the priority as per [API documentation](https://api.cloudflare.com/#page-rules-for-a-zone-create-page-rule).
    pub priority: pulumi_wasm_rust::Output<Option<i32>>,
    /// Whether the page rule is active or disabled.
    pub status: pulumi_wasm_rust::Output<Option<String>>,
    /// The URL pattern to target with the page rule.
    pub target: pulumi_wasm_rust::Output<String>,
    /// The DNS zone ID to which the page rule should be added.
    pub zone_id: pulumi_wasm_rust::Output<String>,
}
///
/// Registers a new resource with the given unique name and arguments
///
#[allow(non_snake_case, unused_imports)]
pub fn create(name: &str, args: PageRuleArgs) -> PageRuleResult {
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
    use pulumi_wasm_rust::Output;
    use std::collections::HashMap;
    let actions_binding = args.actions.get_inner();
    let priority_binding = args.priority.get_inner();
    let status_binding = args.status.get_inner();
    let target_binding = args.target.get_inner();
    let zone_id_binding = args.zone_id.get_inner();
    let request = register_interface::RegisterResourceRequest {
        type_: "cloudflare:index/pageRule:PageRule".into(),
        name: name.to_string(),
        object: Vec::from([
            register_interface::ObjectField {
                name: "actions".into(),
                value: &actions_binding,
            },
            register_interface::ObjectField {
                name: "priority".into(),
                value: &priority_binding,
            },
            register_interface::ObjectField {
                name: "status".into(),
                value: &status_binding,
            },
            register_interface::ObjectField {
                name: "target".into(),
                value: &target_binding,
            },
            register_interface::ObjectField {
                name: "zoneId".into(),
                value: &zone_id_binding,
            },
        ]),
        results: vec![
            register_interface::ResultField { name : "actions".into() },
            register_interface::ResultField { name : "priority".into() },
            register_interface::ResultField { name : "status".into() },
            register_interface::ResultField { name : "target".into() },
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
    PageRuleResult {
        actions: into_domain(hashmap.remove("actions").unwrap()),
        priority: into_domain(hashmap.remove("priority").unwrap()),
        status: into_domain(hashmap.remove("status").unwrap()),
        target: into_domain(hashmap.remove("target").unwrap()),
        zone_id: into_domain(hashmap.remove("zoneId").unwrap()),
    }
}
