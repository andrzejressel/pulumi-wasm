#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct GetAccountRolesArgs {
    /// The account identifier to target for the resource.
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
}
pub struct GetAccountRolesResult {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The provider-assigned unique ID for this managed resource.
    pub id: pulumi_wasm_rust::Output<String>,
    /// A list of roles object.
    pub roles: pulumi_wasm_rust::Output<Vec<super::super::types::GetAccountRolesRole>>,
}
///
/// Registers a new resource with the given unique name and arguments
///
#[allow(non_snake_case, unused_imports)]
pub fn invoke(args: GetAccountRolesArgs) -> GetAccountRolesResult {
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
    use pulumi_wasm_rust::Output;
    use std::collections::HashMap;
    let account_id_binding = args.account_id.get_inner();
    let request = register_interface::ResourceInvokeRequest {
        token: "cloudflare:index/getAccountRoles:getAccountRoles".into(),
        object: Vec::from([
            register_interface::ObjectField {
                name: "accountId".into(),
                value: &account_id_binding,
            },
        ]),
        results: vec![
            register_interface::ResultField { name : "accountId".into() },
            register_interface::ResultField { name : "id".into() },
            register_interface::ResultField { name : "roles".into() },
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
    GetAccountRolesResult {
        account_id: into_domain(hashmap.remove("accountId").unwrap()),
        id: into_domain(hashmap.remove("id").unwrap()),
        roles: into_domain(hashmap.remove("roles").unwrap()),
    }
}