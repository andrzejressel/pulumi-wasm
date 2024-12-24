#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct SecretArgs {
    /// Base64-url-safe-encoded secret data
    #[builder(into)]
    pub data: pulumi_wasm_rust::Output<String>,
    /// User-defined key/value metadata
    #[builder(into, default)]
    pub labels: pulumi_wasm_rust::Output<Option<Vec<super::types::SecretLabel>>>,
    /// User-defined name of the secret
    #[builder(into, default)]
    pub name: pulumi_wasm_rust::Output<Option<String>>,
}
pub struct SecretResult {
    /// Base64-url-safe-encoded secret data
    pub data: pulumi_wasm_rust::Output<String>,
    /// User-defined key/value metadata
    pub labels: pulumi_wasm_rust::Output<Option<Vec<super::types::SecretLabel>>>,
    /// User-defined name of the secret
    pub name: pulumi_wasm_rust::Output<String>,
}
///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: SecretArgs) -> SecretResult {
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
    use pulumi_wasm_rust::Output;
    use std::collections::HashMap;
    let data_binding = args.data.get_inner();
    let labels_binding = args.labels.get_inner();
    let name_binding = args.name.get_inner();
    let request = register_interface::RegisterResourceRequest {
        type_: "docker:index/secret:Secret".into(),
        name: name.to_string(),
        object: Vec::from([
            register_interface::ObjectField {
                name: "data".into(),
                value: &data_binding,
            },
            register_interface::ObjectField {
                name: "labels".into(),
                value: &labels_binding,
            },
            register_interface::ObjectField {
                name: "name".into(),
                value: &name_binding,
            },
        ]),
        results: vec![
            register_interface::ResultField { name : "data".into() },
            register_interface::ResultField { name : "labels".into() },
            register_interface::ResultField { name : "name".into() },
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
    SecretResult {
        data: into_domain(hashmap.remove("data").unwrap()),
        labels: into_domain(hashmap.remove("labels").unwrap()),
        name: into_domain(hashmap.remove("name").unwrap()),
    }
}
