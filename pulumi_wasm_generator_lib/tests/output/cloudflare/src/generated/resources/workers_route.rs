#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct WorkersRouteArgs {
    /// The [route pattern](https://developers.cloudflare.com/workers/about/routes/) to associate the Worker with.
    #[builder(into)]
    pub pattern: pulumi_wasm_rust::Output<String>,
    /// Worker script name to invoke for requests that match the route pattern.
    #[builder(into, default)]
    pub script_name: pulumi_wasm_rust::Output<Option<String>>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub zone_id: pulumi_wasm_rust::Output<String>,
}
pub struct WorkersRouteResult {
    /// The [route pattern](https://developers.cloudflare.com/workers/about/routes/) to associate the Worker with.
    pub pattern: pulumi_wasm_rust::Output<String>,
    /// Worker script name to invoke for requests that match the route pattern.
    pub script_name: pulumi_wasm_rust::Output<Option<String>>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub zone_id: pulumi_wasm_rust::Output<String>,
}
///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: WorkersRouteArgs) -> WorkersRouteResult {
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
    use pulumi_wasm_rust::Output;
    use std::collections::HashMap;
    let pattern_binding = args.pattern.get_inner();
    let script_name_binding = args.script_name.get_inner();
    let zone_id_binding = args.zone_id.get_inner();
    let request = register_interface::RegisterResourceRequest {
        type_: "cloudflare:index/workersRoute:WorkersRoute".into(),
        name: name.to_string(),
        object: Vec::from([
            register_interface::ObjectField {
                name: "pattern".into(),
                value: &pattern_binding,
            },
            register_interface::ObjectField {
                name: "scriptName".into(),
                value: &script_name_binding,
            },
            register_interface::ObjectField {
                name: "zoneId".into(),
                value: &zone_id_binding,
            },
        ]),
        results: vec![
            register_interface::ResultField { name : "pattern".into() },
            register_interface::ResultField { name : "scriptName".into() },
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
    WorkersRouteResult {
        pattern: into_domain(hashmap.remove("pattern").unwrap()),
        script_name: into_domain(hashmap.remove("scriptName").unwrap()),
        zone_id: into_domain(hashmap.remove("zoneId").unwrap()),
    }
}
