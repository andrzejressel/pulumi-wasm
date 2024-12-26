#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct WorkersKvNamespaceArgs {
    /// The account identifier to target for the resource.
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// Title value of the Worker KV Namespace.
    #[builder(into)]
    pub title: pulumi_wasm_rust::Output<String>,
}
pub struct WorkersKvNamespaceResult {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// Title value of the Worker KV Namespace.
    pub title: pulumi_wasm_rust::Output<String>,
}
///
/// Registers a new resource with the given unique name and arguments
///
#[allow(non_snake_case, unused_imports)]
pub fn create(name: &str, args: WorkersKvNamespaceArgs) -> WorkersKvNamespaceResult {
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
    use pulumi_wasm_rust::Output;
    use std::collections::HashMap;
    let account_id_binding = args.account_id.get_inner();
    let title_binding = args.title.get_inner();
    let request = register_interface::RegisterResourceRequest {
        type_: "cloudflare:index/workersKvNamespace:WorkersKvNamespace".into(),
        name: name.to_string(),
        object: Vec::from([
            register_interface::ObjectField {
                name: "accountId".into(),
                value: &account_id_binding,
            },
            register_interface::ObjectField {
                name: "title".into(),
                value: &title_binding,
            },
        ]),
        results: vec![
            register_interface::ResultField { name : "accountId".into() },
            register_interface::ResultField { name : "title".into() },
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
    WorkersKvNamespaceResult {
        account_id: into_domain(hashmap.remove("accountId").unwrap()),
        title: into_domain(hashmap.remove("title").unwrap()),
    }
}
