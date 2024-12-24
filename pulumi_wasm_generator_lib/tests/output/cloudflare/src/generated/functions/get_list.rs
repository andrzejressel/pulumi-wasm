#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct GetListArgs {
    /// The account identifier to target for the resource.
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The list name to target for the resource.
    #[builder(into)]
    pub name: pulumi_wasm_rust::Output<String>,
}
pub struct GetListResult {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// List description.
    pub description: pulumi_wasm_rust::Output<String>,
    /// The provider-assigned unique ID for this managed resource.
    pub id: pulumi_wasm_rust::Output<String>,
    /// List kind.
    pub kind: pulumi_wasm_rust::Output<String>,
    /// The list name to target for the resource.
    pub name: pulumi_wasm_rust::Output<String>,
    /// Number of items in list.
    pub numitems: pulumi_wasm_rust::Output<i32>,
}
///
/// Registers a new resource with the given unique name and arguments
///
pub fn invoke(args: GetListArgs) -> GetListResult {
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
    use pulumi_wasm_rust::Output;
    use std::collections::HashMap;
    let account_id_binding = args.account_id.get_inner();
    let name_binding = args.name.get_inner();
    let request = register_interface::ResourceInvokeRequest {
        token: "cloudflare:index/getList:getList".into(),
        object: Vec::from([
            register_interface::ObjectField {
                name: "accountId".into(),
                value: &account_id_binding,
            },
            register_interface::ObjectField {
                name: "name".into(),
                value: &name_binding,
            },
        ]),
        results: vec![
            register_interface::ResultField { name : "accountId".into() },
            register_interface::ResultField { name : "description".into() },
            register_interface::ResultField { name : "id".into() },
            register_interface::ResultField { name : "kind".into() },
            register_interface::ResultField { name : "name".into() },
            register_interface::ResultField { name : "numitems".into() },
        ],
    };
    fn into_domain<F: serde::Serialize>(output: WitOutput) -> Output<F> {
        unsafe { Output::<F>::new_from_handle(output) }
    }
    let o = register_interface::invoke(&request);
    let mut hashmap: HashMap<String, _> = o
        .fields
        .into_iter()
        .map(|f| (f.name, f.output))
        .collect();
    GetListResult {
        account_id: into_domain(hashmap.remove("accountId").unwrap()),
        description: into_domain(hashmap.remove("description").unwrap()),
        id: into_domain(hashmap.remove("id").unwrap()),
        kind: into_domain(hashmap.remove("kind").unwrap()),
        name: into_domain(hashmap.remove("name").unwrap()),
        numitems: into_domain(hashmap.remove("numitems").unwrap()),
    }
}
