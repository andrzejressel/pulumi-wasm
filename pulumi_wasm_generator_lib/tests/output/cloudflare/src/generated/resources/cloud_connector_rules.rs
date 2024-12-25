#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct CloudConnectorRulesArgs {
    /// List of Cloud Connector Rules
    #[builder(into, default)]
    pub rules: pulumi_wasm_rust::Output<
        Option<Vec<super::types::CloudConnectorRulesRule>>,
    >,
    /// The zone identifier to target for the resource.
    #[builder(into)]
    pub zone_id: pulumi_wasm_rust::Output<String>,
}
pub struct CloudConnectorRulesResult {
    /// List of Cloud Connector Rules
    pub rules: pulumi_wasm_rust::Output<
        Option<Vec<super::types::CloudConnectorRulesRule>>,
    >,
    /// The zone identifier to target for the resource.
    pub zone_id: pulumi_wasm_rust::Output<String>,
}
///
/// Registers a new resource with the given unique name and arguments
///
#[allow(non_snake_case, unused_imports)]
pub fn create(name: &str, args: CloudConnectorRulesArgs) -> CloudConnectorRulesResult {
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
    use pulumi_wasm_rust::Output;
    use std::collections::HashMap;
    let rules_binding = args.rules.get_inner();
    let zone_id_binding = args.zone_id.get_inner();
    let request = register_interface::RegisterResourceRequest {
        type_: "cloudflare:index/cloudConnectorRules:CloudConnectorRules".into(),
        name: name.to_string(),
        object: Vec::from([
            register_interface::ObjectField {
                name: "rules".into(),
                value: &rules_binding,
            },
            register_interface::ObjectField {
                name: "zoneId".into(),
                value: &zone_id_binding,
            },
        ]),
        results: vec![
            register_interface::ResultField { name : "rules".into() },
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
    CloudConnectorRulesResult {
        rules: into_domain(hashmap.remove("rules").unwrap()),
        zone_id: into_domain(hashmap.remove("zoneId").unwrap()),
    }
}
