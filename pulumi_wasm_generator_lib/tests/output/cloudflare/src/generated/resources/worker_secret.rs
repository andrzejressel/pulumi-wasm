#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct WorkerSecretArgs {
    /// The account identifier to target for the resource.
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The name of the Worker secret. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub name: pulumi_wasm_rust::Output<String>,
    /// The name of the Worker script to associate the secret with. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub script_name: pulumi_wasm_rust::Output<String>,
    /// The text of the Worker secret. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub secret_text: pulumi_wasm_rust::Output<String>,
}
pub struct WorkerSecretResult {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The name of the Worker secret. **Modifying this attribute will force creation of a new resource.**
    pub name: pulumi_wasm_rust::Output<String>,
    /// The name of the Worker script to associate the secret with. **Modifying this attribute will force creation of a new resource.**
    pub script_name: pulumi_wasm_rust::Output<String>,
    /// The text of the Worker secret. **Modifying this attribute will force creation of a new resource.**
    pub secret_text: pulumi_wasm_rust::Output<String>,
}
///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: WorkerSecretArgs) -> WorkerSecretResult {
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
    use pulumi_wasm_rust::Output;
    use std::collections::HashMap;
    let account_id_binding = args.account_id.get_inner();
    let name_binding = args.name.get_inner();
    let script_name_binding = args.script_name.get_inner();
    let secret_text_binding = args.secret_text.get_inner();
    let request = register_interface::RegisterResourceRequest {
        type_: "cloudflare:index/workerSecret:WorkerSecret".into(),
        name: name.to_string(),
        object: Vec::from([
            register_interface::ObjectField {
                name: "accountId".into(),
                value: &account_id_binding,
            },
            register_interface::ObjectField {
                name: "name".into(),
                value: &name_binding,
            },
            register_interface::ObjectField {
                name: "scriptName".into(),
                value: &script_name_binding,
            },
            register_interface::ObjectField {
                name: "secretText".into(),
                value: &secret_text_binding,
            },
        ]),
        results: vec![
            register_interface::ResultField { name : "accountId".into() },
            register_interface::ResultField { name : "name".into() },
            register_interface::ResultField { name : "scriptName".into() },
            register_interface::ResultField { name : "secretText".into() },
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
    WorkerSecretResult {
        account_id: into_domain(hashmap.remove("accountId").unwrap()),
        name: into_domain(hashmap.remove("name").unwrap()),
        script_name: into_domain(hashmap.remove("scriptName").unwrap()),
        secret_text: into_domain(hashmap.remove("secretText").unwrap()),
    }
}
