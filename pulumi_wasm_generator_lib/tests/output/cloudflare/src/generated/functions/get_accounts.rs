#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct GetAccountsArgs {
    /// The account name to target for the resource.
    #[builder(into, default)]
    pub name: pulumi_wasm_rust::Output<Option<String>>,
}
pub struct GetAccountsResult {
    pub accounts: pulumi_wasm_rust::Output<Vec<super::super::types::GetAccountsAccount>>,
    /// The provider-assigned unique ID for this managed resource.
    pub id: pulumi_wasm_rust::Output<String>,
    /// The account name to target for the resource.
    pub name: pulumi_wasm_rust::Output<Option<String>>,
}
///
/// Registers a new resource with the given unique name and arguments
///
pub fn invoke(args: GetAccountsArgs) -> GetAccountsResult {
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
    use pulumi_wasm_rust::Output;
    use std::collections::HashMap;
    let name_binding = args.name.get_inner();
    let request = register_interface::ResourceInvokeRequest {
        token: "cloudflare:index/getAccounts:getAccounts".into(),
        object: Vec::from([
            register_interface::ObjectField {
                name: "name".into(),
                value: &name_binding,
            },
        ]),
        results: vec![
            register_interface::ResultField { name : "accounts".into() },
            register_interface::ResultField { name : "id".into() },
            register_interface::ResultField { name : "name".into() },
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
    GetAccountsResult {
        accounts: into_domain(hashmap.remove("accounts").unwrap()),
        id: into_domain(hashmap.remove("id").unwrap()),
        name: into_domain(hashmap.remove("name").unwrap()),
    }
}
