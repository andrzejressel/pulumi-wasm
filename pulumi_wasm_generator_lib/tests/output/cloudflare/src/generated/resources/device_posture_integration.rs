#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct DevicePostureIntegrationArgs {
    /// The account identifier to target for the resource.
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The device posture integration's connection authorization parameters.
    #[builder(into, default)]
    pub configs: pulumi_wasm_rust::Output<
        Option<Vec<super::types::DevicePostureIntegrationConfig>>,
    >,
    #[builder(into, default)]
    pub identifier: pulumi_wasm_rust::Output<Option<String>>,
    /// Indicates the frequency with which to poll the third-party API. Must be in the format `1h` or `30m`.
    #[builder(into, default)]
    pub interval: pulumi_wasm_rust::Output<Option<String>>,
    /// Name of the device posture integration.
    #[builder(into)]
    pub name: pulumi_wasm_rust::Output<String>,
    /// The device posture integration type. Available values: `workspace_one`, `uptycs`, `crowdstrike_s2s`, `intune`, `kolide`, `sentinelone_s2s`, `tanium_s2s`, `custom_s2s`.
    #[builder(into)]
    pub type_: pulumi_wasm_rust::Output<String>,
}
pub struct DevicePostureIntegrationResult {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The device posture integration's connection authorization parameters.
    pub configs: pulumi_wasm_rust::Output<
        Option<Vec<super::types::DevicePostureIntegrationConfig>>,
    >,
    pub identifier: pulumi_wasm_rust::Output<Option<String>>,
    /// Indicates the frequency with which to poll the third-party API. Must be in the format `1h` or `30m`.
    pub interval: pulumi_wasm_rust::Output<Option<String>>,
    /// Name of the device posture integration.
    pub name: pulumi_wasm_rust::Output<String>,
    /// The device posture integration type. Available values: `workspace_one`, `uptycs`, `crowdstrike_s2s`, `intune`, `kolide`, `sentinelone_s2s`, `tanium_s2s`, `custom_s2s`.
    pub type_: pulumi_wasm_rust::Output<String>,
}
///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(
    name: &str,
    args: DevicePostureIntegrationArgs,
) -> DevicePostureIntegrationResult {
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
    use pulumi_wasm_rust::Output;
    use std::collections::HashMap;
    let account_id_binding = args.account_id.get_inner();
    let configs_binding = args.configs.get_inner();
    let identifier_binding = args.identifier.get_inner();
    let interval_binding = args.interval.get_inner();
    let name_binding = args.name.get_inner();
    let type__binding = args.type_.get_inner();
    let request = register_interface::RegisterResourceRequest {
        type_: "cloudflare:index/devicePostureIntegration:DevicePostureIntegration"
            .into(),
        name: name.to_string(),
        object: Vec::from([
            register_interface::ObjectField {
                name: "accountId".into(),
                value: &account_id_binding,
            },
            register_interface::ObjectField {
                name: "configs".into(),
                value: &configs_binding,
            },
            register_interface::ObjectField {
                name: "identifier".into(),
                value: &identifier_binding,
            },
            register_interface::ObjectField {
                name: "interval".into(),
                value: &interval_binding,
            },
            register_interface::ObjectField {
                name: "name".into(),
                value: &name_binding,
            },
            register_interface::ObjectField {
                name: "type".into(),
                value: &type__binding,
            },
        ]),
        results: vec![
            register_interface::ResultField { name : "accountId".into() },
            register_interface::ResultField { name : "configs".into() },
            register_interface::ResultField { name : "identifier".into() },
            register_interface::ResultField { name : "interval".into() },
            register_interface::ResultField { name : "name".into() },
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
    DevicePostureIntegrationResult {
        account_id: into_domain(hashmap.remove("accountId").unwrap()),
        configs: into_domain(hashmap.remove("configs").unwrap()),
        identifier: into_domain(hashmap.remove("identifier").unwrap()),
        interval: into_domain(hashmap.remove("interval").unwrap()),
        name: into_domain(hashmap.remove("name").unwrap()),
        type_: into_domain(hashmap.remove("type").unwrap()),
    }
}
