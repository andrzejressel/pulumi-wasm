#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct ApiShieldSchemaValidationSettingsArgs {
    /// The default mitigation action used when there is no mitigation action defined on the operation.
    #[builder(into)]
    pub validation_default_mitigation_action: pulumi_wasm_rust::Output<String>,
    /// When set, this overrides both zone level and operation level mitigation actions.
    #[builder(into, default)]
    pub validation_override_mitigation_action: pulumi_wasm_rust::Output<Option<String>>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub zone_id: pulumi_wasm_rust::Output<String>,
}
pub struct ApiShieldSchemaValidationSettingsResult {
    /// The default mitigation action used when there is no mitigation action defined on the operation.
    pub validation_default_mitigation_action: pulumi_wasm_rust::Output<String>,
    /// When set, this overrides both zone level and operation level mitigation actions.
    pub validation_override_mitigation_action: pulumi_wasm_rust::Output<Option<String>>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub zone_id: pulumi_wasm_rust::Output<String>,
}
///
/// Registers a new resource with the given unique name and arguments
///
#[allow(non_snake_case, unused_imports)]
pub fn create(
    name: &str,
    args: ApiShieldSchemaValidationSettingsArgs,
) -> ApiShieldSchemaValidationSettingsResult {
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
    use pulumi_wasm_rust::Output;
    use std::collections::HashMap;
    let validation_default_mitigation_action_binding = args
        .validation_default_mitigation_action
        .get_inner();
    let validation_override_mitigation_action_binding = args
        .validation_override_mitigation_action
        .get_inner();
    let zone_id_binding = args.zone_id.get_inner();
    let request = register_interface::RegisterResourceRequest {
        type_: "cloudflare:index/apiShieldSchemaValidationSettings:ApiShieldSchemaValidationSettings"
            .into(),
        name: name.to_string(),
        object: Vec::from([
            register_interface::ObjectField {
                name: "validationDefaultMitigationAction".into(),
                value: &validation_default_mitigation_action_binding,
            },
            register_interface::ObjectField {
                name: "validationOverrideMitigationAction".into(),
                value: &validation_override_mitigation_action_binding,
            },
            register_interface::ObjectField {
                name: "zoneId".into(),
                value: &zone_id_binding,
            },
        ]),
        results: vec![
            register_interface::ResultField { name : "validationDefaultMitigationAction"
            .into() }, register_interface::ResultField { name :
            "validationOverrideMitigationAction".into() },
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
    ApiShieldSchemaValidationSettingsResult {
        validation_default_mitigation_action: into_domain(
            hashmap.remove("validationDefaultMitigationAction").unwrap(),
        ),
        validation_override_mitigation_action: into_domain(
            hashmap.remove("validationOverrideMitigationAction").unwrap(),
        ),
        zone_id: into_domain(hashmap.remove("zoneId").unwrap()),
    }
}
