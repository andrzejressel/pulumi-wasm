#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct WorkersKvArgs {
    /// The account identifier to target for the resource.
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// Name of the KV pair. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub key: pulumi_wasm_rust::Output<String>,
    /// The ID of the Workers KV namespace in which you want to create the KV pair. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub namespace_id: pulumi_wasm_rust::Output<String>,
    /// Value of the KV pair.
    #[builder(into)]
    pub value: pulumi_wasm_rust::Output<String>,
}
pub struct WorkersKvResult {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// Name of the KV pair. **Modifying this attribute will force creation of a new resource.**
    pub key: pulumi_wasm_rust::Output<String>,
    /// The ID of the Workers KV namespace in which you want to create the KV pair. **Modifying this attribute will force creation of a new resource.**
    pub namespace_id: pulumi_wasm_rust::Output<String>,
    /// Value of the KV pair.
    pub value: pulumi_wasm_rust::Output<String>,
}
///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: WorkersKvArgs) -> WorkersKvResult {
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
    use pulumi_wasm_rust::Output;
    use std::collections::HashMap;
    let account_id_binding = args.account_id.get_inner();
    let key_binding = args.key.get_inner();
    let namespace_id_binding = args.namespace_id.get_inner();
    let value_binding = args.value.get_inner();
    let request = register_interface::RegisterResourceRequest {
        type_: "cloudflare:index/workersKv:WorkersKv".into(),
        name: name.to_string(),
        object: Vec::from([
            register_interface::ObjectField {
                name: "accountId".into(),
                value: &account_id_binding,
            },
            register_interface::ObjectField {
                name: "key".into(),
                value: &key_binding,
            },
            register_interface::ObjectField {
                name: "namespaceId".into(),
                value: &namespace_id_binding,
            },
            register_interface::ObjectField {
                name: "value".into(),
                value: &value_binding,
            },
        ]),
        results: vec![
            register_interface::ResultField { name : "accountId".into() },
            register_interface::ResultField { name : "key".into() },
            register_interface::ResultField { name : "namespaceId".into() },
            register_interface::ResultField { name : "value".into() },
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
    WorkersKvResult {
        account_id: into_domain(hashmap.remove("accountId").unwrap()),
        key: into_domain(hashmap.remove("key").unwrap()),
        namespace_id: into_domain(hashmap.remove("namespaceId").unwrap()),
        value: into_domain(hashmap.remove("value").unwrap()),
    }
}
