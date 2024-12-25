#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct ApiShieldArgs {
    /// Characteristics define properties across which auth-ids can be computed in a privacy-preserving manner.
    #[builder(into, default)]
    pub auth_id_characteristics: pulumi_wasm_rust::Output<
        Option<Vec<super::types::ApiShieldAuthIdCharacteristic>>,
    >,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub zone_id: pulumi_wasm_rust::Output<String>,
}
pub struct ApiShieldResult {
    /// Characteristics define properties across which auth-ids can be computed in a privacy-preserving manner.
    pub auth_id_characteristics: pulumi_wasm_rust::Output<
        Option<Vec<super::types::ApiShieldAuthIdCharacteristic>>,
    >,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub zone_id: pulumi_wasm_rust::Output<String>,
}
///
/// Registers a new resource with the given unique name and arguments
///
#[allow(non_snake_case, unused_imports)]
pub fn create(name: &str, args: ApiShieldArgs) -> ApiShieldResult {
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
    use pulumi_wasm_rust::Output;
    use std::collections::HashMap;
    let auth_id_characteristics_binding = args.auth_id_characteristics.get_inner();
    let zone_id_binding = args.zone_id.get_inner();
    let request = register_interface::RegisterResourceRequest {
        type_: "cloudflare:index/apiShield:ApiShield".into(),
        name: name.to_string(),
        object: Vec::from([
            register_interface::ObjectField {
                name: "authIdCharacteristics".into(),
                value: &auth_id_characteristics_binding,
            },
            register_interface::ObjectField {
                name: "zoneId".into(),
                value: &zone_id_binding,
            },
        ]),
        results: vec![
            register_interface::ResultField { name : "authIdCharacteristics".into() },
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
    ApiShieldResult {
        auth_id_characteristics: into_domain(
            hashmap.remove("authIdCharacteristics").unwrap(),
        ),
        zone_id: into_domain(hashmap.remove("zoneId").unwrap()),
    }
}
