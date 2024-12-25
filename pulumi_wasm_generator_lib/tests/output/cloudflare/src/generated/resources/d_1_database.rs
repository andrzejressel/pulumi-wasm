#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct D1DatabaseArgs {
    /// The account identifier to target for the resource.
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The name of the D1 Database.
    #[builder(into)]
    pub name: pulumi_wasm_rust::Output<String>,
}
pub struct D1DatabaseResult {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The name of the D1 Database.
    pub name: pulumi_wasm_rust::Output<String>,
    /// The backend version of the database.
    pub version: pulumi_wasm_rust::Output<String>,
}
///
/// Registers a new resource with the given unique name and arguments
///
#[allow(non_snake_case, unused_imports)]
pub fn create(name: &str, args: D1DatabaseArgs) -> D1DatabaseResult {
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
    use pulumi_wasm_rust::Output;
    use std::collections::HashMap;
    let account_id_binding = args.account_id.get_inner();
    let name_binding = args.name.get_inner();
    let request = register_interface::RegisterResourceRequest {
        type_: "cloudflare:index/d1Database:D1Database".into(),
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
        ]),
        results: vec![
            register_interface::ResultField { name : "accountId".into() },
            register_interface::ResultField { name : "name".into() },
            register_interface::ResultField { name : "version".into() },
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
    D1DatabaseResult {
        account_id: into_domain(hashmap.remove("accountId").unwrap()),
        name: into_domain(hashmap.remove("name").unwrap()),
        version: into_domain(hashmap.remove("version").unwrap()),
    }
}
