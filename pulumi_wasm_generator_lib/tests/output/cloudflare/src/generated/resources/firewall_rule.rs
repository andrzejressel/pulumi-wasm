#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct FirewallRuleArgs {
    /// The action to apply to a matched request. Available values: `block`, `challenge`, `allow`, `js_challenge`, `managed_challenge`, `log`, `bypass`.
    #[builder(into)]
    pub action: pulumi_wasm_rust::Output<String>,
    /// A description of the rule to help identify it.
    #[builder(into, default)]
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    /// The identifier of the Filter to use for determining if the Firewall Rule should be triggered.
    #[builder(into)]
    pub filter_id: pulumi_wasm_rust::Output<String>,
    /// Whether this filter based firewall rule is currently paused.
    #[builder(into, default)]
    pub paused: pulumi_wasm_rust::Output<Option<bool>>,
    /// The priority of the rule to allow control of processing order. A lower number indicates high priority. If not provided, any rules with a priority will be sequenced before those without.
    #[builder(into, default)]
    pub priority: pulumi_wasm_rust::Output<Option<i32>>,
    /// List of products to bypass for a request when the bypass action is used. Available values: `zoneLockdown`, `uaBlock`, `bic`, `hot`, `securityLevel`, `rateLimit`, `waf`.
    #[builder(into, default)]
    pub products: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub zone_id: pulumi_wasm_rust::Output<String>,
}
pub struct FirewallRuleResult {
    /// The action to apply to a matched request. Available values: `block`, `challenge`, `allow`, `js_challenge`, `managed_challenge`, `log`, `bypass`.
    pub action: pulumi_wasm_rust::Output<String>,
    /// A description of the rule to help identify it.
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    /// The identifier of the Filter to use for determining if the Firewall Rule should be triggered.
    pub filter_id: pulumi_wasm_rust::Output<String>,
    /// Whether this filter based firewall rule is currently paused.
    pub paused: pulumi_wasm_rust::Output<Option<bool>>,
    /// The priority of the rule to allow control of processing order. A lower number indicates high priority. If not provided, any rules with a priority will be sequenced before those without.
    pub priority: pulumi_wasm_rust::Output<Option<i32>>,
    /// List of products to bypass for a request when the bypass action is used. Available values: `zoneLockdown`, `uaBlock`, `bic`, `hot`, `securityLevel`, `rateLimit`, `waf`.
    pub products: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub zone_id: pulumi_wasm_rust::Output<String>,
}
///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: FirewallRuleArgs) -> FirewallRuleResult {
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
    use pulumi_wasm_rust::Output;
    use std::collections::HashMap;
    let action_binding = args.action.get_inner();
    let description_binding = args.description.get_inner();
    let filter_id_binding = args.filter_id.get_inner();
    let paused_binding = args.paused.get_inner();
    let priority_binding = args.priority.get_inner();
    let products_binding = args.products.get_inner();
    let zone_id_binding = args.zone_id.get_inner();
    let request = register_interface::RegisterResourceRequest {
        type_: "cloudflare:index/firewallRule:FirewallRule".into(),
        name: name.to_string(),
        object: Vec::from([
            register_interface::ObjectField {
                name: "action".into(),
                value: &action_binding,
            },
            register_interface::ObjectField {
                name: "description".into(),
                value: &description_binding,
            },
            register_interface::ObjectField {
                name: "filterId".into(),
                value: &filter_id_binding,
            },
            register_interface::ObjectField {
                name: "paused".into(),
                value: &paused_binding,
            },
            register_interface::ObjectField {
                name: "priority".into(),
                value: &priority_binding,
            },
            register_interface::ObjectField {
                name: "products".into(),
                value: &products_binding,
            },
            register_interface::ObjectField {
                name: "zoneId".into(),
                value: &zone_id_binding,
            },
        ]),
        results: vec![
            register_interface::ResultField { name : "action".into() },
            register_interface::ResultField { name : "description".into() },
            register_interface::ResultField { name : "filterId".into() },
            register_interface::ResultField { name : "paused".into() },
            register_interface::ResultField { name : "priority".into() },
            register_interface::ResultField { name : "products".into() },
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
    FirewallRuleResult {
        action: into_domain(hashmap.remove("action").unwrap()),
        description: into_domain(hashmap.remove("description").unwrap()),
        filter_id: into_domain(hashmap.remove("filterId").unwrap()),
        paused: into_domain(hashmap.remove("paused").unwrap()),
        priority: into_domain(hashmap.remove("priority").unwrap()),
        products: into_domain(hashmap.remove("products").unwrap()),
        zone_id: into_domain(hashmap.remove("zoneId").unwrap()),
    }
}
