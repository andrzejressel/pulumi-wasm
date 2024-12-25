#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct WorkersDomainArgs {
    /// The account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The name of the Worker environment. Defaults to `production`.
    #[builder(into, default)]
    pub environment: pulumi_wasm_rust::Output<Option<String>>,
    /// Hostname of the Worker Domain.
    #[builder(into)]
    pub hostname: pulumi_wasm_rust::Output<String>,
    /// Name of worker script to attach the domain to.
    #[builder(into)]
    pub service: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub zone_id: pulumi_wasm_rust::Output<String>,
}
pub struct WorkersDomainResult {
    /// The account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The name of the Worker environment. Defaults to `production`.
    pub environment: pulumi_wasm_rust::Output<Option<String>>,
    /// Hostname of the Worker Domain.
    pub hostname: pulumi_wasm_rust::Output<String>,
    /// Name of worker script to attach the domain to.
    pub service: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub zone_id: pulumi_wasm_rust::Output<String>,
}
///
/// Registers a new resource with the given unique name and arguments
///
#[allow(non_snake_case, unused_imports)]
pub fn create(name: &str, args: WorkersDomainArgs) -> WorkersDomainResult {
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
    use pulumi_wasm_rust::Output;
    use std::collections::HashMap;
    let account_id_binding = args.account_id.get_inner();
    let environment_binding = args.environment.get_inner();
    let hostname_binding = args.hostname.get_inner();
    let service_binding = args.service.get_inner();
    let zone_id_binding = args.zone_id.get_inner();
    let request = register_interface::RegisterResourceRequest {
        type_: "cloudflare:index/workersDomain:WorkersDomain".into(),
        name: name.to_string(),
        object: Vec::from([
            register_interface::ObjectField {
                name: "accountId".into(),
                value: &account_id_binding,
            },
            register_interface::ObjectField {
                name: "environment".into(),
                value: &environment_binding,
            },
            register_interface::ObjectField {
                name: "hostname".into(),
                value: &hostname_binding,
            },
            register_interface::ObjectField {
                name: "service".into(),
                value: &service_binding,
            },
            register_interface::ObjectField {
                name: "zoneId".into(),
                value: &zone_id_binding,
            },
        ]),
        results: vec![
            register_interface::ResultField { name : "accountId".into() },
            register_interface::ResultField { name : "environment".into() },
            register_interface::ResultField { name : "hostname".into() },
            register_interface::ResultField { name : "service".into() },
            register_interface::ResultField { name : "zoneId".into() },
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
    WorkersDomainResult {
        account_id: into_domain(hashmap.remove("accountId").unwrap()),
        environment: into_domain(hashmap.remove("environment").unwrap()),
        hostname: into_domain(hashmap.remove("hostname").unwrap()),
        service: into_domain(hashmap.remove("service").unwrap()),
        zone_id: into_domain(hashmap.remove("zoneId").unwrap()),
    }
}
