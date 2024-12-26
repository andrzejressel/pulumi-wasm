#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct InfrastructureAccessTargetArgs {
    /// The account identifier to target for the resource.
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// A non-unique field that refers to a target.
    #[builder(into)]
    pub hostname: pulumi_wasm_rust::Output<String>,
    /// The IPv4/IPv6 address that identifies where to reach a target.
    #[builder(into)]
    pub ip: pulumi_wasm_rust::Output<super::types::InfrastructureAccessTargetIp>,
}
pub struct InfrastructureAccessTargetResult {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The date and time at which the target was created.
    pub created_at: pulumi_wasm_rust::Output<String>,
    /// A non-unique field that refers to a target.
    pub hostname: pulumi_wasm_rust::Output<String>,
    /// The IPv4/IPv6 address that identifies where to reach a target.
    pub ip: pulumi_wasm_rust::Output<super::types::InfrastructureAccessTargetIp>,
    /// The date and time at which the target was last modified.
    pub modified_at: pulumi_wasm_rust::Output<String>,
}
///
/// Registers a new resource with the given unique name and arguments
///
#[allow(non_snake_case, unused_imports)]
pub fn create(
    name: &str,
    args: InfrastructureAccessTargetArgs,
) -> InfrastructureAccessTargetResult {
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
    use pulumi_wasm_rust::Output;
    use std::collections::HashMap;
    let account_id_binding = args.account_id.get_inner();
    let hostname_binding = args.hostname.get_inner();
    let ip_binding = args.ip.get_inner();
    let request = register_interface::RegisterResourceRequest {
        type_: "cloudflare:index/infrastructureAccessTarget:InfrastructureAccessTarget"
            .into(),
        name: name.to_string(),
        object: Vec::from([
            register_interface::ObjectField {
                name: "accountId".into(),
                value: &account_id_binding,
            },
            register_interface::ObjectField {
                name: "hostname".into(),
                value: &hostname_binding,
            },
            register_interface::ObjectField {
                name: "ip".into(),
                value: &ip_binding,
            },
        ]),
        results: vec![
            register_interface::ResultField { name : "accountId".into() },
            register_interface::ResultField { name : "createdAt".into() },
            register_interface::ResultField { name : "hostname".into() },
            register_interface::ResultField { name : "ip".into() },
            register_interface::ResultField { name : "modifiedAt".into() },
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
    InfrastructureAccessTargetResult {
        account_id: into_domain(hashmap.remove("accountId").unwrap()),
        created_at: into_domain(hashmap.remove("createdAt").unwrap()),
        hostname: into_domain(hashmap.remove("hostname").unwrap()),
        ip: into_domain(hashmap.remove("ip").unwrap()),
        modified_at: into_domain(hashmap.remove("modifiedAt").unwrap()),
    }
}
