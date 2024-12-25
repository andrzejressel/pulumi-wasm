#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct TeamsProxyEndpointArgs {
    /// The account identifier to target for the resource.
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The networks CIDRs that will be allowed to initiate proxy connections.
    #[builder(into)]
    pub ips: pulumi_wasm_rust::Output<Vec<String>>,
    /// Name of the teams proxy endpoint.
    #[builder(into)]
    pub name: pulumi_wasm_rust::Output<String>,
}
pub struct TeamsProxyEndpointResult {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The networks CIDRs that will be allowed to initiate proxy connections.
    pub ips: pulumi_wasm_rust::Output<Vec<String>>,
    /// Name of the teams proxy endpoint.
    pub name: pulumi_wasm_rust::Output<String>,
    /// The FQDN that proxy clients should be pointed at.
    pub subdomain: pulumi_wasm_rust::Output<String>,
}
///
/// Registers a new resource with the given unique name and arguments
///
#[allow(non_snake_case, unused_imports)]
pub fn create(name: &str, args: TeamsProxyEndpointArgs) -> TeamsProxyEndpointResult {
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
    use pulumi_wasm_rust::Output;
    use std::collections::HashMap;
    let account_id_binding = args.account_id.get_inner();
    let ips_binding = args.ips.get_inner();
    let name_binding = args.name.get_inner();
    let request = register_interface::RegisterResourceRequest {
        type_: "cloudflare:index/teamsProxyEndpoint:TeamsProxyEndpoint".into(),
        name: name.to_string(),
        object: Vec::from([
            register_interface::ObjectField {
                name: "accountId".into(),
                value: &account_id_binding,
            },
            register_interface::ObjectField {
                name: "ips".into(),
                value: &ips_binding,
            },
            register_interface::ObjectField {
                name: "name".into(),
                value: &name_binding,
            },
        ]),
        results: vec![
            register_interface::ResultField { name : "accountId".into() },
            register_interface::ResultField { name : "ips".into() },
            register_interface::ResultField { name : "name".into() },
            register_interface::ResultField { name : "subdomain".into() },
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
    TeamsProxyEndpointResult {
        account_id: into_domain(hashmap.remove("accountId").unwrap()),
        ips: into_domain(hashmap.remove("ips").unwrap()),
        name: into_domain(hashmap.remove("name").unwrap()),
        subdomain: into_domain(hashmap.remove("subdomain").unwrap()),
    }
}
