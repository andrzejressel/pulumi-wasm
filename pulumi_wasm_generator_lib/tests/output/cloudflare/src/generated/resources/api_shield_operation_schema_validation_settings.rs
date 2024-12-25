#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct ApiShieldOperationSchemaValidationSettingsArgs {
    /// The mitigation action to apply to this operation.
    #[builder(into, default)]
    pub mitigation_action: pulumi_wasm_rust::Output<Option<String>>,
    /// Operation ID these settings should apply to. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub operation_id: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub zone_id: pulumi_wasm_rust::Output<String>,
}
pub struct ApiShieldOperationSchemaValidationSettingsResult {
    /// The mitigation action to apply to this operation.
    pub mitigation_action: pulumi_wasm_rust::Output<Option<String>>,
    /// Operation ID these settings should apply to. **Modifying this attribute will force creation of a new resource.**
    pub operation_id: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub zone_id: pulumi_wasm_rust::Output<String>,
}
///
/// Registers a new resource with the given unique name and arguments
///
#[allow(non_snake_case, unused_imports)]
pub fn create(
    name: &str,
    args: ApiShieldOperationSchemaValidationSettingsArgs,
) -> ApiShieldOperationSchemaValidationSettingsResult {
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
    use pulumi_wasm_rust::Output;
    use std::collections::HashMap;
    let mitigation_action_binding = args.mitigation_action.get_inner();
    let operation_id_binding = args.operation_id.get_inner();
    let zone_id_binding = args.zone_id.get_inner();
    let request = register_interface::RegisterResourceRequest {
        type_: "cloudflare:index/apiShieldOperationSchemaValidationSettings:ApiShieldOperationSchemaValidationSettings"
            .into(),
        name: name.to_string(),
        object: Vec::from([
            register_interface::ObjectField {
                name: "mitigationAction".into(),
                value: &mitigation_action_binding,
            },
            register_interface::ObjectField {
                name: "operationId".into(),
                value: &operation_id_binding,
            },
            register_interface::ObjectField {
                name: "zoneId".into(),
                value: &zone_id_binding,
            },
        ]),
        results: vec![
            register_interface::ResultField { name : "mitigationAction".into() },
            register_interface::ResultField { name : "operationId".into() },
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
    ApiShieldOperationSchemaValidationSettingsResult {
        mitigation_action: into_domain(hashmap.remove("mitigationAction").unwrap()),
        operation_id: into_domain(hashmap.remove("operationId").unwrap()),
        zone_id: into_domain(hashmap.remove("zoneId").unwrap()),
    }
}
