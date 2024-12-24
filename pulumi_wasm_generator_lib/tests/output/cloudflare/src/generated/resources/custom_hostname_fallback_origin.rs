#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct CustomHostnameFallbackOriginArgs {
    /// Hostname you intend to fallback requests to. Origin must be a proxied A/AAAA/CNAME DNS record within Clouldflare.
    #[builder(into)]
    pub origin: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub zone_id: pulumi_wasm_rust::Output<String>,
}
pub struct CustomHostnameFallbackOriginResult {
    /// Hostname you intend to fallback requests to. Origin must be a proxied A/AAAA/CNAME DNS record within Clouldflare.
    pub origin: pulumi_wasm_rust::Output<String>,
    /// Status of the fallback origin's activation.
    pub status: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub zone_id: pulumi_wasm_rust::Output<String>,
}
///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(
    name: &str,
    args: CustomHostnameFallbackOriginArgs,
) -> CustomHostnameFallbackOriginResult {
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
    use pulumi_wasm_rust::Output;
    use std::collections::HashMap;
    let origin_binding = args.origin.get_inner();
    let zone_id_binding = args.zone_id.get_inner();
    let request = register_interface::RegisterResourceRequest {
        type_: "cloudflare:index/customHostnameFallbackOrigin:CustomHostnameFallbackOrigin"
            .into(),
        name: name.to_string(),
        object: Vec::from([
            register_interface::ObjectField {
                name: "origin".into(),
                value: &origin_binding,
            },
            register_interface::ObjectField {
                name: "zoneId".into(),
                value: &zone_id_binding,
            },
        ]),
        results: vec![
            register_interface::ResultField { name : "origin".into() },
            register_interface::ResultField { name : "status".into() },
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
    CustomHostnameFallbackOriginResult {
        origin: into_domain(hashmap.remove("origin").unwrap()),
        status: into_domain(hashmap.remove("status").unwrap()),
        zone_id: into_domain(hashmap.remove("zoneId").unwrap()),
    }
}
