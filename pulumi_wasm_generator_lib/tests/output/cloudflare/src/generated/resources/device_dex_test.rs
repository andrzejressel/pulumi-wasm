#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct DeviceDexTestArgs {
    /// The account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The configuration object which contains the details for the WARP client to conduct the test.
    #[builder(into)]
    pub data: pulumi_wasm_rust::Output<super::types::DeviceDexTestData>,
    /// Additional details about the test.
    #[builder(into)]
    pub description: pulumi_wasm_rust::Output<String>,
    /// Determines whether or not the test is active.
    #[builder(into)]
    pub enabled: pulumi_wasm_rust::Output<bool>,
    /// How often the test will run.
    #[builder(into)]
    pub interval: pulumi_wasm_rust::Output<String>,
    /// The name of the Device Dex Test. Must be unique.
    #[builder(into)]
    pub name: pulumi_wasm_rust::Output<String>,
}
pub struct DeviceDexTestResult {
    /// The account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// Timestamp of when the Dex Test was created.
    pub created: pulumi_wasm_rust::Output<String>,
    /// The configuration object which contains the details for the WARP client to conduct the test.
    pub data: pulumi_wasm_rust::Output<super::types::DeviceDexTestData>,
    /// Additional details about the test.
    pub description: pulumi_wasm_rust::Output<String>,
    /// Determines whether or not the test is active.
    pub enabled: pulumi_wasm_rust::Output<bool>,
    /// How often the test will run.
    pub interval: pulumi_wasm_rust::Output<String>,
    /// The name of the Device Dex Test. Must be unique.
    pub name: pulumi_wasm_rust::Output<String>,
    /// Timestamp of when the Dex Test was last updated.
    pub updated: pulumi_wasm_rust::Output<String>,
}
///
/// Registers a new resource with the given unique name and arguments
///
#[allow(non_snake_case, unused_imports)]
pub fn create(name: &str, args: DeviceDexTestArgs) -> DeviceDexTestResult {
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
    use pulumi_wasm_rust::Output;
    use std::collections::HashMap;
    let account_id_binding = args.account_id.get_inner();
    let data_binding = args.data.get_inner();
    let description_binding = args.description.get_inner();
    let enabled_binding = args.enabled.get_inner();
    let interval_binding = args.interval.get_inner();
    let name_binding = args.name.get_inner();
    let request = register_interface::RegisterResourceRequest {
        type_: "cloudflare:index/deviceDexTest:DeviceDexTest".into(),
        name: name.to_string(),
        object: Vec::from([
            register_interface::ObjectField {
                name: "accountId".into(),
                value: &account_id_binding,
            },
            register_interface::ObjectField {
                name: "data".into(),
                value: &data_binding,
            },
            register_interface::ObjectField {
                name: "description".into(),
                value: &description_binding,
            },
            register_interface::ObjectField {
                name: "enabled".into(),
                value: &enabled_binding,
            },
            register_interface::ObjectField {
                name: "interval".into(),
                value: &interval_binding,
            },
            register_interface::ObjectField {
                name: "name".into(),
                value: &name_binding,
            },
        ]),
        results: vec![
            register_interface::ResultField { name : "accountId".into() },
            register_interface::ResultField { name : "created".into() },
            register_interface::ResultField { name : "data".into() },
            register_interface::ResultField { name : "description".into() },
            register_interface::ResultField { name : "enabled".into() },
            register_interface::ResultField { name : "interval".into() },
            register_interface::ResultField { name : "name".into() },
            register_interface::ResultField { name : "updated".into() },
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
    DeviceDexTestResult {
        account_id: into_domain(hashmap.remove("accountId").unwrap()),
        created: into_domain(hashmap.remove("created").unwrap()),
        data: into_domain(hashmap.remove("data").unwrap()),
        description: into_domain(hashmap.remove("description").unwrap()),
        enabled: into_domain(hashmap.remove("enabled").unwrap()),
        interval: into_domain(hashmap.remove("interval").unwrap()),
        name: into_domain(hashmap.remove("name").unwrap()),
        updated: into_domain(hashmap.remove("updated").unwrap()),
    }
}
