#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct PagesDomainArgs {
    /// The account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// Custom domain. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub domain: pulumi_wasm_rust::Output<String>,
    /// Name of the Pages Project. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub project_name: pulumi_wasm_rust::Output<String>,
}
pub struct PagesDomainResult {
    /// The account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// Custom domain. **Modifying this attribute will force creation of a new resource.**
    pub domain: pulumi_wasm_rust::Output<String>,
    /// Name of the Pages Project. **Modifying this attribute will force creation of a new resource.**
    pub project_name: pulumi_wasm_rust::Output<String>,
    /// Status of the custom domain.
    pub status: pulumi_wasm_rust::Output<String>,
}
///
/// Registers a new resource with the given unique name and arguments
///
#[allow(non_snake_case, unused_imports)]
pub fn create(name: &str, args: PagesDomainArgs) -> PagesDomainResult {
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
    use pulumi_wasm_rust::Output;
    use std::collections::HashMap;
    let account_id_binding = args.account_id.get_inner();
    let domain_binding = args.domain.get_inner();
    let project_name_binding = args.project_name.get_inner();
    let request = register_interface::RegisterResourceRequest {
        type_: "cloudflare:index/pagesDomain:PagesDomain".into(),
        name: name.to_string(),
        object: Vec::from([
            register_interface::ObjectField {
                name: "accountId".into(),
                value: &account_id_binding,
            },
            register_interface::ObjectField {
                name: "domain".into(),
                value: &domain_binding,
            },
            register_interface::ObjectField {
                name: "projectName".into(),
                value: &project_name_binding,
            },
        ]),
        results: vec![
            register_interface::ResultField { name : "accountId".into() },
            register_interface::ResultField { name : "domain".into() },
            register_interface::ResultField { name : "projectName".into() },
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
    PagesDomainResult {
        account_id: into_domain(hashmap.remove("accountId").unwrap()),
        domain: into_domain(hashmap.remove("domain").unwrap()),
        project_name: into_domain(hashmap.remove("projectName").unwrap()),
        status: into_domain(hashmap.remove("status").unwrap()),
    }
}
