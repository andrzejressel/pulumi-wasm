#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct EmailRoutingAddressArgs {
    /// The account identifier to target for the resource.
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The contact email address of the user.
    #[builder(into)]
    pub email: pulumi_wasm_rust::Output<String>,
}
pub struct EmailRoutingAddressResult {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The date and time the destination address has been created.
    pub created: pulumi_wasm_rust::Output<String>,
    /// The contact email address of the user.
    pub email: pulumi_wasm_rust::Output<String>,
    /// The date and time the destination address has been modified.
    pub modified: pulumi_wasm_rust::Output<String>,
    /// Destination address identifier.
    pub tag: pulumi_wasm_rust::Output<String>,
    /// The date and time the destination address has been verified. Null means not verified yet.
    pub verified: pulumi_wasm_rust::Output<String>,
}
///
/// Registers a new resource with the given unique name and arguments
///
#[allow(non_snake_case, unused_imports)]
pub fn create(name: &str, args: EmailRoutingAddressArgs) -> EmailRoutingAddressResult {
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
    use pulumi_wasm_rust::Output;
    use std::collections::HashMap;
    let account_id_binding = args.account_id.get_inner();
    let email_binding = args.email.get_inner();
    let request = register_interface::RegisterResourceRequest {
        type_: "cloudflare:index/emailRoutingAddress:EmailRoutingAddress".into(),
        name: name.to_string(),
        object: Vec::from([
            register_interface::ObjectField {
                name: "accountId".into(),
                value: &account_id_binding,
            },
            register_interface::ObjectField {
                name: "email".into(),
                value: &email_binding,
            },
        ]),
        results: vec![
            register_interface::ResultField { name : "accountId".into() },
            register_interface::ResultField { name : "created".into() },
            register_interface::ResultField { name : "email".into() },
            register_interface::ResultField { name : "modified".into() },
            register_interface::ResultField { name : "tag".into() },
            register_interface::ResultField { name : "verified".into() },
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
    EmailRoutingAddressResult {
        account_id: into_domain(hashmap.remove("accountId").unwrap()),
        created: into_domain(hashmap.remove("created").unwrap()),
        email: into_domain(hashmap.remove("email").unwrap()),
        modified: into_domain(hashmap.remove("modified").unwrap()),
        tag: into_domain(hashmap.remove("tag").unwrap()),
        verified: into_domain(hashmap.remove("verified").unwrap()),
    }
}
