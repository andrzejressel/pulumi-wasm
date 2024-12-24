#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct WaitingRoomSettingsArgs {
    /// Whether to allow verified search engine crawlers to bypass all waiting rooms on this zone. Defaults to `false`.
    #[builder(into, default)]
    pub search_engine_crawler_bypass: pulumi_wasm_rust::Output<Option<bool>>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub zone_id: pulumi_wasm_rust::Output<String>,
}
pub struct WaitingRoomSettingsResult {
    /// Whether to allow verified search engine crawlers to bypass all waiting rooms on this zone. Defaults to `false`.
    pub search_engine_crawler_bypass: pulumi_wasm_rust::Output<Option<bool>>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub zone_id: pulumi_wasm_rust::Output<String>,
}
///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: WaitingRoomSettingsArgs) -> WaitingRoomSettingsResult {
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
    use pulumi_wasm_rust::Output;
    use std::collections::HashMap;
    let search_engine_crawler_bypass_binding = args
        .search_engine_crawler_bypass
        .get_inner();
    let zone_id_binding = args.zone_id.get_inner();
    let request = register_interface::RegisterResourceRequest {
        type_: "cloudflare:index/waitingRoomSettings:WaitingRoomSettings".into(),
        name: name.to_string(),
        object: Vec::from([
            register_interface::ObjectField {
                name: "searchEngineCrawlerBypass".into(),
                value: &search_engine_crawler_bypass_binding,
            },
            register_interface::ObjectField {
                name: "zoneId".into(),
                value: &zone_id_binding,
            },
        ]),
        results: vec![
            register_interface::ResultField { name : "searchEngineCrawlerBypass".into()
            }, register_interface::ResultField { name : "zoneId".into() },
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
    WaitingRoomSettingsResult {
        search_engine_crawler_bypass: into_domain(
            hashmap.remove("searchEngineCrawlerBypass").unwrap(),
        ),
        zone_id: into_domain(hashmap.remove("zoneId").unwrap()),
    }
}
