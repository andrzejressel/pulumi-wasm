#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct ListArgs {
    /// The account identifier to target for the resource.
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// An optional description of the list.
    #[builder(into, default)]
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    /// The items in the list.
    #[builder(into, default)]
    pub items: pulumi_wasm_rust::Output<Option<Vec<super::types::ListItem>>>,
    /// The type of items the list will contain. Must provide only one of: `ip`, `redirect`, `hostname`, `asn`..
    #[builder(into)]
    pub kind: pulumi_wasm_rust::Output<String>,
    /// The name of the list.
    #[builder(into)]
    pub name: pulumi_wasm_rust::Output<String>,
}
pub struct ListResult {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// An optional description of the list.
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    /// The items in the list.
    pub items: pulumi_wasm_rust::Output<Option<Vec<super::types::ListItem>>>,
    /// The type of items the list will contain. Must provide only one of: `ip`, `redirect`, `hostname`, `asn`..
    pub kind: pulumi_wasm_rust::Output<String>,
    /// The name of the list.
    pub name: pulumi_wasm_rust::Output<String>,
}
///
/// Registers a new resource with the given unique name and arguments
///
#[allow(non_snake_case, unused_imports)]
pub fn create(name: &str, args: ListArgs) -> ListResult {
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
    use pulumi_wasm_rust::Output;
    use std::collections::HashMap;
    let account_id_binding = args.account_id.get_inner();
    let description_binding = args.description.get_inner();
    let items_binding = args.items.get_inner();
    let kind_binding = args.kind.get_inner();
    let name_binding = args.name.get_inner();
    let request = register_interface::RegisterResourceRequest {
        type_: "cloudflare:index/list:List".into(),
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
                name: "items".into(),
                value: &items_binding,
            },
            register_interface::ObjectField {
                name: "kind".into(),
                value: &kind_binding,
            },
            register_interface::ObjectField {
                name: "name".into(),
                value: &name_binding,
            },
        ]),
        results: vec![
            register_interface::ResultField { name : "accountId".into() },
            register_interface::ResultField { name : "description".into() },
            register_interface::ResultField { name : "items".into() },
            register_interface::ResultField { name : "kind".into() },
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
    ListResult {
        account_id: into_domain(hashmap.remove("accountId").unwrap()),
        description: into_domain(hashmap.remove("description").unwrap()),
        items: into_domain(hashmap.remove("items").unwrap()),
        kind: into_domain(hashmap.remove("kind").unwrap()),
        name: into_domain(hashmap.remove("name").unwrap()),
    }
}