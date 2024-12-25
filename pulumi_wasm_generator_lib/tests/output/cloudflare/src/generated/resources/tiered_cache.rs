#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct TieredCacheArgs {
    /// The typed of tiered cache to utilize on the zone. Available values: `generic`, `smart`, `off`.
    #[builder(into)]
    pub cache_type: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub zone_id: pulumi_wasm_rust::Output<String>,
}
pub struct TieredCacheResult {
    /// The typed of tiered cache to utilize on the zone. Available values: `generic`, `smart`, `off`.
    pub cache_type: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub zone_id: pulumi_wasm_rust::Output<String>,
}
///
/// Registers a new resource with the given unique name and arguments
///
#[allow(non_snake_case, unused_imports)]
pub fn create(name: &str, args: TieredCacheArgs) -> TieredCacheResult {
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
    use pulumi_wasm_rust::Output;
    use std::collections::HashMap;
    let cache_type_binding = args.cache_type.get_inner();
    let zone_id_binding = args.zone_id.get_inner();
    let request = register_interface::RegisterResourceRequest {
        type_: "cloudflare:index/tieredCache:TieredCache".into(),
        name: name.to_string(),
        object: Vec::from([
            register_interface::ObjectField {
                name: "cacheType".into(),
                value: &cache_type_binding,
            },
            register_interface::ObjectField {
                name: "zoneId".into(),
                value: &zone_id_binding,
            },
        ]),
        results: vec![
            register_interface::ResultField { name : "cacheType".into() },
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
    TieredCacheResult {
        cache_type: into_domain(hashmap.remove("cacheType").unwrap()),
        zone_id: into_domain(hashmap.remove("zoneId").unwrap()),
    }
}
