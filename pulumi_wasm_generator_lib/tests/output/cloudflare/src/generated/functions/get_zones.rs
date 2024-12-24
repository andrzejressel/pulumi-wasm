#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct GetZonesArgs {
    /// One or more values used to look up zone records. If more than one value is given all values must match in order to be included.
    #[builder(into)]
    pub filter: pulumi_wasm_rust::Output<super::super::types::GetZonesFilter>,
}
pub struct GetZonesResult {
    /// One or more values used to look up zone records. If more than one value is given all values must match in order to be included.
    pub filter: pulumi_wasm_rust::Output<super::super::types::GetZonesFilter>,
    /// The provider-assigned unique ID for this managed resource.
    pub id: pulumi_wasm_rust::Output<String>,
    /// A list of zone objects.
    pub zones: pulumi_wasm_rust::Output<Vec<super::super::types::GetZonesZone>>,
}
///
/// Registers a new resource with the given unique name and arguments
///
pub fn invoke(args: GetZonesArgs) -> GetZonesResult {
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
    use pulumi_wasm_rust::Output;
    use std::collections::HashMap;
    let filter_binding = args.filter.get_inner();
    let request = register_interface::ResourceInvokeRequest {
        token: "cloudflare:index/getZones:getZones".into(),
        object: Vec::from([
            register_interface::ObjectField {
                name: "filter".into(),
                value: &filter_binding,
            },
        ]),
        results: vec![
            register_interface::ResultField { name : "filter".into() },
            register_interface::ResultField { name : "id".into() },
            register_interface::ResultField { name : "zones".into() },
        ],
    };
    fn into_domain<F: serde::Serialize>(output: WitOutput) -> Output<F> {
        unsafe { Output::<F>::new_from_handle(output) }
    }
    let o = register_interface::invoke(&request);
    let mut hashmap: HashMap<String, _> = o
        .fields
        .into_iter()
        .map(|f| (f.name, f.output))
        .collect();
    GetZonesResult {
        filter: into_domain(hashmap.remove("filter").unwrap()),
        id: into_domain(hashmap.remove("id").unwrap()),
        zones: into_domain(hashmap.remove("zones").unwrap()),
    }
}
