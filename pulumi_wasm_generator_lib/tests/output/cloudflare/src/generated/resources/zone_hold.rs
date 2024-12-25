#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct ZoneHoldArgs {
    /// Enablement status of the zone hold.
    #[builder(into)]
    pub hold: pulumi_wasm_rust::Output<bool>,
    /// The RFC3339 compatible timestamp when to automatically re-enable the zone hold.
    #[builder(into, default)]
    pub hold_after: pulumi_wasm_rust::Output<Option<String>>,
    /// Whether to extend to block any subdomain of the given zone.
    #[builder(into, default)]
    pub include_subdomains: pulumi_wasm_rust::Output<Option<bool>>,
    /// The zone identifier to target for the resource.
    #[builder(into)]
    pub zone_id: pulumi_wasm_rust::Output<String>,
}
pub struct ZoneHoldResult {
    /// Enablement status of the zone hold.
    pub hold: pulumi_wasm_rust::Output<bool>,
    /// The RFC3339 compatible timestamp when to automatically re-enable the zone hold.
    pub hold_after: pulumi_wasm_rust::Output<String>,
    /// Whether to extend to block any subdomain of the given zone.
    pub include_subdomains: pulumi_wasm_rust::Output<Option<bool>>,
    /// The zone identifier to target for the resource.
    pub zone_id: pulumi_wasm_rust::Output<String>,
}
///
/// Registers a new resource with the given unique name and arguments
///
#[allow(non_snake_case, unused_imports)]
pub fn create(name: &str, args: ZoneHoldArgs) -> ZoneHoldResult {
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
    use pulumi_wasm_rust::Output;
    use std::collections::HashMap;
    let hold_binding = args.hold.get_inner();
    let hold_after_binding = args.hold_after.get_inner();
    let include_subdomains_binding = args.include_subdomains.get_inner();
    let zone_id_binding = args.zone_id.get_inner();
    let request = register_interface::RegisterResourceRequest {
        type_: "cloudflare:index/zoneHold:ZoneHold".into(),
        name: name.to_string(),
        object: Vec::from([
            register_interface::ObjectField {
                name: "hold".into(),
                value: &hold_binding,
            },
            register_interface::ObjectField {
                name: "holdAfter".into(),
                value: &hold_after_binding,
            },
            register_interface::ObjectField {
                name: "includeSubdomains".into(),
                value: &include_subdomains_binding,
            },
            register_interface::ObjectField {
                name: "zoneId".into(),
                value: &zone_id_binding,
            },
        ]),
        results: vec![
            register_interface::ResultField { name : "hold".into() },
            register_interface::ResultField { name : "holdAfter".into() },
            register_interface::ResultField { name : "includeSubdomains".into() },
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
    ZoneHoldResult {
        hold: into_domain(hashmap.remove("hold").unwrap()),
        hold_after: into_domain(hashmap.remove("holdAfter").unwrap()),
        include_subdomains: into_domain(hashmap.remove("includeSubdomains").unwrap()),
        zone_id: into_domain(hashmap.remove("zoneId").unwrap()),
    }
}
