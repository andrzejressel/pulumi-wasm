#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct UrlNormalizationSettingsArgs {
    /// The scope of the URL normalization.
    #[builder(into)]
    pub scope: pulumi_wasm_rust::Output<String>,
    /// The type of URL normalization performed by Cloudflare.
    #[builder(into)]
    pub type_: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub zone_id: pulumi_wasm_rust::Output<String>,
}
pub struct UrlNormalizationSettingsResult {
    /// The scope of the URL normalization.
    pub scope: pulumi_wasm_rust::Output<String>,
    /// The type of URL normalization performed by Cloudflare.
    pub type_: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub zone_id: pulumi_wasm_rust::Output<String>,
}
///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(
    name: &str,
    args: UrlNormalizationSettingsArgs,
) -> UrlNormalizationSettingsResult {
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
    use pulumi_wasm_rust::Output;
    use std::collections::HashMap;
    let scope_binding = args.scope.get_inner();
    let type__binding = args.type_.get_inner();
    let zone_id_binding = args.zone_id.get_inner();
    let request = register_interface::RegisterResourceRequest {
        type_: "cloudflare:index/urlNormalizationSettings:UrlNormalizationSettings"
            .into(),
        name: name.to_string(),
        object: Vec::from([
            register_interface::ObjectField {
                name: "scope".into(),
                value: &scope_binding,
            },
            register_interface::ObjectField {
                name: "type".into(),
                value: &type__binding,
            },
            register_interface::ObjectField {
                name: "zoneId".into(),
                value: &zone_id_binding,
            },
        ]),
        results: vec![
            register_interface::ResultField { name : "scope".into() },
            register_interface::ResultField { name : "type".into() },
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
    UrlNormalizationSettingsResult {
        scope: into_domain(hashmap.remove("scope").unwrap()),
        type_: into_domain(hashmap.remove("type").unwrap()),
        zone_id: into_domain(hashmap.remove("zoneId").unwrap()),
    }
}
