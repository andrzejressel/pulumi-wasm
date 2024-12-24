#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct ArgoArgs {
    /// Whether smart routing is enabled. Available values: `on`, `off`.
    #[builder(into, default)]
    pub smart_routing: pulumi_wasm_rust::Output<Option<String>>,
    /// Whether tiered caching is enabled. Available values: `on`, `off`.
    #[builder(into, default)]
    pub tiered_caching: pulumi_wasm_rust::Output<Option<String>>,
    /// The zone identifier to target for the resource.
    #[builder(into)]
    pub zone_id: pulumi_wasm_rust::Output<String>,
}
pub struct ArgoResult {
    /// Whether smart routing is enabled. Available values: `on`, `off`.
    pub smart_routing: pulumi_wasm_rust::Output<Option<String>>,
    /// Whether tiered caching is enabled. Available values: `on`, `off`.
    pub tiered_caching: pulumi_wasm_rust::Output<Option<String>>,
    /// The zone identifier to target for the resource.
    pub zone_id: pulumi_wasm_rust::Output<String>,
}
///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: ArgoArgs) -> ArgoResult {
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
    use pulumi_wasm_rust::Output;
    use std::collections::HashMap;
    let smart_routing_binding = args.smart_routing.get_inner();
    let tiered_caching_binding = args.tiered_caching.get_inner();
    let zone_id_binding = args.zone_id.get_inner();
    let request = register_interface::RegisterResourceRequest {
        type_: "cloudflare:index/argo:Argo".into(),
        name: name.to_string(),
        object: Vec::from([
            register_interface::ObjectField {
                name: "smartRouting".into(),
                value: &smart_routing_binding,
            },
            register_interface::ObjectField {
                name: "tieredCaching".into(),
                value: &tiered_caching_binding,
            },
            register_interface::ObjectField {
                name: "zoneId".into(),
                value: &zone_id_binding,
            },
        ]),
        results: vec![
            register_interface::ResultField { name : "smartRouting".into() },
            register_interface::ResultField { name : "tieredCaching".into() },
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
    ArgoResult {
        smart_routing: into_domain(hashmap.remove("smartRouting").unwrap()),
        tiered_caching: into_domain(hashmap.remove("tieredCaching").unwrap()),
        zone_id: into_domain(hashmap.remove("zoneId").unwrap()),
    }
}
