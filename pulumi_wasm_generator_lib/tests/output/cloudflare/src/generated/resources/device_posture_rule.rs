#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct DevicePostureRuleArgs {
    /// The account identifier to target for the resource.
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    #[builder(into, default)]
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    /// Expire posture results after the specified amount of time. Must be in the format `1h` or `30m`. Valid units are `h` and `m`.
    #[builder(into, default)]
    pub expiration: pulumi_wasm_rust::Output<Option<String>>,
    /// Required for all rule types except `warp`, `gateway`, and `tanium`.
    #[builder(into, default)]
    pub inputs: pulumi_wasm_rust::Output<
        Option<Vec<super::types::DevicePostureRuleInput>>,
    >,
    /// The conditions that the client must match to run the rule.
    #[builder(into, default)]
    pub matches: pulumi_wasm_rust::Output<
        Option<Vec<super::types::DevicePostureRuleMatch>>,
    >,
    /// Name of the device posture rule.
    #[builder(into, default)]
    pub name: pulumi_wasm_rust::Output<Option<String>>,
    /// Tells the client when to run the device posture check. Must be in the format `1h` or `30m`. Valid units are `h` and `m`.
    #[builder(into, default)]
    pub schedule: pulumi_wasm_rust::Output<Option<String>>,
    /// The device posture rule type. Available values: `serial_number`, `file`, `application`, `gateway`, `warp`, `domain_joined`, `os_version`, `disk_encryption`, `firewall`, `client_certificate`, `client_certificate_v2`, `workspace_one`, `unique_client_id`, `crowdstrike_s2s`, `sentinelone`, `kolide`, `tanium_s2s`, `intune`, `sentinelone_s2s`, `custom_s2s`.
    #[builder(into)]
    pub type_: pulumi_wasm_rust::Output<String>,
}
pub struct DevicePostureRuleResult {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<String>,
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    /// Expire posture results after the specified amount of time. Must be in the format `1h` or `30m`. Valid units are `h` and `m`.
    pub expiration: pulumi_wasm_rust::Output<Option<String>>,
    /// Required for all rule types except `warp`, `gateway`, and `tanium`.
    pub inputs: pulumi_wasm_rust::Output<Vec<super::types::DevicePostureRuleInput>>,
    /// The conditions that the client must match to run the rule.
    pub matches: pulumi_wasm_rust::Output<
        Option<Vec<super::types::DevicePostureRuleMatch>>,
    >,
    /// Name of the device posture rule.
    pub name: pulumi_wasm_rust::Output<Option<String>>,
    /// Tells the client when to run the device posture check. Must be in the format `1h` or `30m`. Valid units are `h` and `m`.
    pub schedule: pulumi_wasm_rust::Output<Option<String>>,
    /// The device posture rule type. Available values: `serial_number`, `file`, `application`, `gateway`, `warp`, `domain_joined`, `os_version`, `disk_encryption`, `firewall`, `client_certificate`, `client_certificate_v2`, `workspace_one`, `unique_client_id`, `crowdstrike_s2s`, `sentinelone`, `kolide`, `tanium_s2s`, `intune`, `sentinelone_s2s`, `custom_s2s`.
    pub type_: pulumi_wasm_rust::Output<String>,
}
///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: DevicePostureRuleArgs) -> DevicePostureRuleResult {
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
    use pulumi_wasm_rust::Output;
    use std::collections::HashMap;
    let account_id_binding = args.account_id.get_inner();
    let description_binding = args.description.get_inner();
    let expiration_binding = args.expiration.get_inner();
    let inputs_binding = args.inputs.get_inner();
    let matches_binding = args.matches.get_inner();
    let name_binding = args.name.get_inner();
    let schedule_binding = args.schedule.get_inner();
    let type__binding = args.type_.get_inner();
    let request = register_interface::RegisterResourceRequest {
        type_: "cloudflare:index/devicePostureRule:DevicePostureRule".into(),
        name: name.to_string(),
        object: Vec::from([
            register_interface::ObjectField {
                name: "accountId".into(),
                value: &account_id_binding,
            },
            register_interface::ObjectField {
                name: "description".into(),
                value: &description_binding,
            },
            register_interface::ObjectField {
                name: "expiration".into(),
                value: &expiration_binding,
            },
            register_interface::ObjectField {
                name: "inputs".into(),
                value: &inputs_binding,
            },
            register_interface::ObjectField {
                name: "matches".into(),
                value: &matches_binding,
            },
            register_interface::ObjectField {
                name: "name".into(),
                value: &name_binding,
            },
            register_interface::ObjectField {
                name: "schedule".into(),
                value: &schedule_binding,
            },
            register_interface::ObjectField {
                name: "type".into(),
                value: &type__binding,
            },
        ]),
        results: vec![
            register_interface::ResultField { name : "accountId".into() },
            register_interface::ResultField { name : "description".into() },
            register_interface::ResultField { name : "expiration".into() },
            register_interface::ResultField { name : "inputs".into() },
            register_interface::ResultField { name : "matches".into() },
            register_interface::ResultField { name : "name".into() },
            register_interface::ResultField { name : "schedule".into() },
            register_interface::ResultField { name : "type".into() },
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
    DevicePostureRuleResult {
        account_id: into_domain(hashmap.remove("accountId").unwrap()),
        description: into_domain(hashmap.remove("description").unwrap()),
        expiration: into_domain(hashmap.remove("expiration").unwrap()),
        inputs: into_domain(hashmap.remove("inputs").unwrap()),
        matches: into_domain(hashmap.remove("matches").unwrap()),
        name: into_domain(hashmap.remove("name").unwrap()),
        schedule: into_domain(hashmap.remove("schedule").unwrap()),
        type_: into_domain(hashmap.remove("type").unwrap()),
    }
}
