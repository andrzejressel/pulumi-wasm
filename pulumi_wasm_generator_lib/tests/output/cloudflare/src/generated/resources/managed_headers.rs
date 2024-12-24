#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct ManagedHeadersArgs {
    /// The list of managed request headers.
    #[builder(into, default)]
    pub managed_request_headers: pulumi_wasm_rust::Output<
        Option<Vec<super::types::ManagedHeadersManagedRequestHeader>>,
    >,
    /// The list of managed response headers.
    #[builder(into, default)]
    pub managed_response_headers: pulumi_wasm_rust::Output<
        Option<Vec<super::types::ManagedHeadersManagedResponseHeader>>,
    >,
    /// The zone identifier to target for the resource.
    #[builder(into)]
    pub zone_id: pulumi_wasm_rust::Output<String>,
}
pub struct ManagedHeadersResult {
    /// The list of managed request headers.
    pub managed_request_headers: pulumi_wasm_rust::Output<
        Option<Vec<super::types::ManagedHeadersManagedRequestHeader>>,
    >,
    /// The list of managed response headers.
    pub managed_response_headers: pulumi_wasm_rust::Output<
        Option<Vec<super::types::ManagedHeadersManagedResponseHeader>>,
    >,
    /// The zone identifier to target for the resource.
    pub zone_id: pulumi_wasm_rust::Output<String>,
}
///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: ManagedHeadersArgs) -> ManagedHeadersResult {
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
    use pulumi_wasm_rust::Output;
    use std::collections::HashMap;
    let managed_request_headers_binding = args.managed_request_headers.get_inner();
    let managed_response_headers_binding = args.managed_response_headers.get_inner();
    let zone_id_binding = args.zone_id.get_inner();
    let request = register_interface::RegisterResourceRequest {
        type_: "cloudflare:index/managedHeaders:ManagedHeaders".into(),
        name: name.to_string(),
        object: Vec::from([
            register_interface::ObjectField {
                name: "managedRequestHeaders".into(),
                value: &managed_request_headers_binding,
            },
            register_interface::ObjectField {
                name: "managedResponseHeaders".into(),
                value: &managed_response_headers_binding,
            },
            register_interface::ObjectField {
                name: "zoneId".into(),
                value: &zone_id_binding,
            },
        ]),
        results: vec![
            register_interface::ResultField { name : "managedRequestHeaders".into() },
            register_interface::ResultField { name : "managedResponseHeaders".into() },
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
    ManagedHeadersResult {
        managed_request_headers: into_domain(
            hashmap.remove("managedRequestHeaders").unwrap(),
        ),
        managed_response_headers: into_domain(
            hashmap.remove("managedResponseHeaders").unwrap(),
        ),
        zone_id: into_domain(hashmap.remove("zoneId").unwrap()),
    }
}
