#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct AccountArgs {
    /// Whether 2FA is enforced on the account. Defaults to `false`.
    #[builder(into, default)]
    pub enforce_twofactor: pulumi_wasm_rust::Output<Option<bool>>,
    /// The name of the account that is displayed in the Cloudflare dashboard.
    #[builder(into)]
    pub name: pulumi_wasm_rust::Output<String>,
    /// Account type. Available values: `enterprise`, `standard`. Defaults to `standard`. **Modifying this attribute will force creation of a new resource.**
    #[builder(into, default)]
    pub type_: pulumi_wasm_rust::Output<Option<String>>,
}
pub struct AccountResult {
    /// Whether 2FA is enforced on the account. Defaults to `false`.
    pub enforce_twofactor: pulumi_wasm_rust::Output<Option<bool>>,
    /// The name of the account that is displayed in the Cloudflare dashboard.
    pub name: pulumi_wasm_rust::Output<String>,
    /// Account type. Available values: `enterprise`, `standard`. Defaults to `standard`. **Modifying this attribute will force creation of a new resource.**
    pub type_: pulumi_wasm_rust::Output<Option<String>>,
}
///
/// Registers a new resource with the given unique name and arguments
///
#[allow(non_snake_case, unused_imports)]
pub fn create(name: &str, args: AccountArgs) -> AccountResult {
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
    use pulumi_wasm_rust::Output;
    use std::collections::HashMap;
    let enforce_twofactor_binding = args.enforce_twofactor.get_inner();
    let name_binding = args.name.get_inner();
    let type__binding = args.type_.get_inner();
    let request = register_interface::RegisterResourceRequest {
        type_: "cloudflare:index/account:Account".into(),
        name: name.to_string(),
        object: Vec::from([
            register_interface::ObjectField {
                name: "enforceTwofactor".into(),
                value: &enforce_twofactor_binding,
            },
            register_interface::ObjectField {
                name: "name".into(),
                value: &name_binding,
            },
            register_interface::ObjectField {
                name: "type".into(),
                value: &type__binding,
            },
        ]),
        results: vec![
            register_interface::ResultField { name : "enforceTwofactor".into() },
            register_interface::ResultField { name : "name".into() },
            register_interface::ResultField { name : "type".into() },
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
    AccountResult {
        enforce_twofactor: into_domain(hashmap.remove("enforceTwofactor").unwrap()),
        name: into_domain(hashmap.remove("name").unwrap()),
        type_: into_domain(hashmap.remove("type").unwrap()),
    }
}
