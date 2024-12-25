#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct FilterArgs {
    /// A note that you can use to describe the purpose of the filter.
    #[builder(into, default)]
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    /// The filter expression to be used.
    #[builder(into)]
    pub expression: pulumi_wasm_rust::Output<String>,
    /// Whether this filter is currently paused.
    #[builder(into, default)]
    pub paused: pulumi_wasm_rust::Output<Option<bool>>,
    /// Short reference tag to quickly select related rules.
    #[builder(into, default)]
    pub ref_: pulumi_wasm_rust::Output<Option<String>>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub zone_id: pulumi_wasm_rust::Output<String>,
}
pub struct FilterResult {
    /// A note that you can use to describe the purpose of the filter.
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    /// The filter expression to be used.
    pub expression: pulumi_wasm_rust::Output<String>,
    /// Whether this filter is currently paused.
    pub paused: pulumi_wasm_rust::Output<Option<bool>>,
    /// Short reference tag to quickly select related rules.
    pub ref_: pulumi_wasm_rust::Output<Option<String>>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub zone_id: pulumi_wasm_rust::Output<String>,
}
///
/// Registers a new resource with the given unique name and arguments
///
#[allow(non_snake_case, unused_imports)]
pub fn create(name: &str, args: FilterArgs) -> FilterResult {
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
    use pulumi_wasm_rust::Output;
    use std::collections::HashMap;
    let description_binding = args.description.get_inner();
    let expression_binding = args.expression.get_inner();
    let paused_binding = args.paused.get_inner();
    let ref__binding = args.ref_.get_inner();
    let zone_id_binding = args.zone_id.get_inner();
    let request = register_interface::RegisterResourceRequest {
        type_: "cloudflare:index/filter:Filter".into(),
        name: name.to_string(),
        object: Vec::from([
            register_interface::ObjectField {
                name: "description".into(),
                value: &description_binding,
            },
            register_interface::ObjectField {
                name: "expression".into(),
                value: &expression_binding,
            },
            register_interface::ObjectField {
                name: "paused".into(),
                value: &paused_binding,
            },
            register_interface::ObjectField {
                name: "ref".into(),
                value: &ref__binding,
            },
            register_interface::ObjectField {
                name: "zoneId".into(),
                value: &zone_id_binding,
            },
        ]),
        results: vec![
            register_interface::ResultField { name : "description".into() },
            register_interface::ResultField { name : "expression".into() },
            register_interface::ResultField { name : "paused".into() },
            register_interface::ResultField { name : "ref".into() },
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
    FilterResult {
        description: into_domain(hashmap.remove("description").unwrap()),
        expression: into_domain(hashmap.remove("expression").unwrap()),
        paused: into_domain(hashmap.remove("paused").unwrap()),
        ref_: into_domain(hashmap.remove("ref").unwrap()),
        zone_id: into_domain(hashmap.remove("zoneId").unwrap()),
    }
}
