#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct RegionalTieredCacheArgs {
    /// Value of the Regional Tiered Cache zone setting.
    #[builder(into)]
    pub value: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource.
    #[builder(into)]
    pub zone_id: pulumi_wasm_rust::Output<String>,
}
pub struct RegionalTieredCacheResult {
    /// Value of the Regional Tiered Cache zone setting.
    pub value: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource.
    pub zone_id: pulumi_wasm_rust::Output<String>,
}
///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: RegionalTieredCacheArgs) -> RegionalTieredCacheResult {
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
    use pulumi_wasm_rust::Output;
    use std::collections::HashMap;
    let value_binding = args.value.get_inner();
    let zone_id_binding = args.zone_id.get_inner();
    let request = register_interface::RegisterResourceRequest {
        type_: "cloudflare:index/regionalTieredCache:RegionalTieredCache".into(),
        name: name.to_string(),
        object: Vec::from([
            register_interface::ObjectField {
                name: "value".into(),
                value: &value_binding,
            },
            register_interface::ObjectField {
                name: "zoneId".into(),
                value: &zone_id_binding,
            },
        ]),
        results: vec![
            register_interface::ResultField { name : "value".into() },
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
    RegionalTieredCacheResult {
        value: into_domain(hashmap.remove("value").unwrap()),
        zone_id: into_domain(hashmap.remove("zoneId").unwrap()),
    }
}
