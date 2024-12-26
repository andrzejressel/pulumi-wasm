#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct AccountMemberArgs {
    /// Account ID to create the account member in.
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The email address of the user who you wish to manage. Following creation, this field becomes read only via the API and cannot be updated.
    #[builder(into)]
    pub email_address: pulumi_wasm_rust::Output<String>,
    /// List of account role IDs that you want to assign to a member.
    #[builder(into)]
    pub role_ids: pulumi_wasm_rust::Output<Vec<String>>,
    /// A member's status in the account. Available values: `accepted`, `pending`.
    #[builder(into, default)]
    pub status: pulumi_wasm_rust::Output<Option<String>>,
}
pub struct AccountMemberResult {
    /// Account ID to create the account member in.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The email address of the user who you wish to manage. Following creation, this field becomes read only via the API and cannot be updated.
    pub email_address: pulumi_wasm_rust::Output<String>,
    /// List of account role IDs that you want to assign to a member.
    pub role_ids: pulumi_wasm_rust::Output<Vec<String>>,
    /// A member's status in the account. Available values: `accepted`, `pending`.
    pub status: pulumi_wasm_rust::Output<String>,
}
///
/// Registers a new resource with the given unique name and arguments
///
#[allow(non_snake_case, unused_imports)]
pub fn create(name: &str, args: AccountMemberArgs) -> AccountMemberResult {
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
    use pulumi_wasm_rust::Output;
    use std::collections::HashMap;
    let account_id_binding = args.account_id.get_inner();
    let email_address_binding = args.email_address.get_inner();
    let role_ids_binding = args.role_ids.get_inner();
    let status_binding = args.status.get_inner();
    let request = register_interface::RegisterResourceRequest {
        type_: "cloudflare:index/accountMember:AccountMember".into(),
        name: name.to_string(),
        object: Vec::from([
            register_interface::ObjectField {
                name: "accountId".into(),
                value: &account_id_binding,
            },
            register_interface::ObjectField {
                name: "emailAddress".into(),
                value: &email_address_binding,
            },
            register_interface::ObjectField {
                name: "roleIds".into(),
                value: &role_ids_binding,
            },
            register_interface::ObjectField {
                name: "status".into(),
                value: &status_binding,
            },
        ]),
        results: vec![
            register_interface::ResultField { name : "accountId".into() },
            register_interface::ResultField { name : "emailAddress".into() },
            register_interface::ResultField { name : "roleIds".into() },
            register_interface::ResultField { name : "status".into() },
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
    AccountMemberResult {
        account_id: into_domain(hashmap.remove("accountId").unwrap()),
        email_address: into_domain(hashmap.remove("emailAddress").unwrap()),
        role_ids: into_domain(hashmap.remove("roleIds").unwrap()),
        status: into_domain(hashmap.remove("status").unwrap()),
    }
}
