#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct AccessRuleArgs {
    /// The account identifier to target for the resource. Must provide only one of `account_id`, `zone_id`. **Modifying this attribute will force creation of a new resource.**
    #[builder(into, default)]
    pub account_id: pulumi_wasm_rust::Output<Option<String>>,
    /// Rule configuration to apply to a matched request. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub configuration: pulumi_wasm_rust::Output<super::types::AccessRuleConfiguration>,
    /// The action to apply to a matched request. Available values: `block`, `challenge`, `whitelist`, `js_challenge`, `managed_challenge`.
    #[builder(into)]
    pub mode: pulumi_wasm_rust::Output<String>,
    /// A personal note about the rule. Typically used as a reminder or explanation for the rule.
    #[builder(into, default)]
    pub notes: pulumi_wasm_rust::Output<Option<String>>,
    /// The zone identifier to target for the resource. Must provide only one of `account_id`, `zone_id`. **Modifying this attribute will force creation of a new resource.**
    #[builder(into, default)]
    pub zone_id: pulumi_wasm_rust::Output<Option<String>>,
}
pub struct AccessRuleResult {
    /// The account identifier to target for the resource. Must provide only one of `account_id`, `zone_id`. **Modifying this attribute will force creation of a new resource.**
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// Rule configuration to apply to a matched request. **Modifying this attribute will force creation of a new resource.**
    pub configuration: pulumi_wasm_rust::Output<super::types::AccessRuleConfiguration>,
    /// The action to apply to a matched request. Available values: `block`, `challenge`, `whitelist`, `js_challenge`, `managed_challenge`.
    pub mode: pulumi_wasm_rust::Output<String>,
    /// A personal note about the rule. Typically used as a reminder or explanation for the rule.
    pub notes: pulumi_wasm_rust::Output<Option<String>>,
    /// The zone identifier to target for the resource. Must provide only one of `account_id`, `zone_id`. **Modifying this attribute will force creation of a new resource.**
    pub zone_id: pulumi_wasm_rust::Output<String>,
}
///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: AccessRuleArgs) -> AccessRuleResult {
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
    use pulumi_wasm_rust::Output;
    use std::collections::HashMap;
    let account_id_binding = args.account_id.get_inner();
    let configuration_binding = args.configuration.get_inner();
    let mode_binding = args.mode.get_inner();
    let notes_binding = args.notes.get_inner();
    let zone_id_binding = args.zone_id.get_inner();
    let request = register_interface::RegisterResourceRequest {
        type_: "cloudflare:index/accessRule:AccessRule".into(),
        name: name.to_string(),
        object: Vec::from([
            register_interface::ObjectField {
                name: "accountId".into(),
                value: &account_id_binding,
            },
            register_interface::ObjectField {
                name: "configuration".into(),
                value: &configuration_binding,
            },
            register_interface::ObjectField {
                name: "mode".into(),
                value: &mode_binding,
            },
            register_interface::ObjectField {
                name: "notes".into(),
                value: &notes_binding,
            },
            register_interface::ObjectField {
                name: "zoneId".into(),
                value: &zone_id_binding,
            },
        ]),
        results: vec![
            register_interface::ResultField { name : "accountId".into() },
            register_interface::ResultField { name : "configuration".into() },
            register_interface::ResultField { name : "mode".into() },
            register_interface::ResultField { name : "notes".into() },
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
    AccessRuleResult {
        account_id: into_domain(hashmap.remove("accountId").unwrap()),
        configuration: into_domain(hashmap.remove("configuration").unwrap()),
        mode: into_domain(hashmap.remove("mode").unwrap()),
        notes: into_domain(hashmap.remove("notes").unwrap()),
        zone_id: into_domain(hashmap.remove("zoneId").unwrap()),
    }
}
