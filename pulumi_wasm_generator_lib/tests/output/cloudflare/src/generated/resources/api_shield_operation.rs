#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct ApiShieldOperationArgs {
    /// The endpoint which can contain path parameter templates in curly braces, each will be replaced from left to right with `{varN}`, starting with `{var1}`. This will then be [Cloudflare-normalized](https://developers.cloudflare.com/rules/normalization/how-it-works/). **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub endpoint: pulumi_wasm_rust::Output<String>,
    /// RFC3986-compliant host. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub host: pulumi_wasm_rust::Output<String>,
    /// The HTTP method used to access the endpoint. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub method: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub zone_id: pulumi_wasm_rust::Output<String>,
}
pub struct ApiShieldOperationResult {
    /// The endpoint which can contain path parameter templates in curly braces, each will be replaced from left to right with `{varN}`, starting with `{var1}`. This will then be [Cloudflare-normalized](https://developers.cloudflare.com/rules/normalization/how-it-works/). **Modifying this attribute will force creation of a new resource.**
    pub endpoint: pulumi_wasm_rust::Output<String>,
    /// RFC3986-compliant host. **Modifying this attribute will force creation of a new resource.**
    pub host: pulumi_wasm_rust::Output<String>,
    /// The HTTP method used to access the endpoint. **Modifying this attribute will force creation of a new resource.**
    pub method: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub zone_id: pulumi_wasm_rust::Output<String>,
}
///
/// Registers a new resource with the given unique name and arguments
///
#[allow(non_snake_case, unused_imports)]
pub fn create(name: &str, args: ApiShieldOperationArgs) -> ApiShieldOperationResult {
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
    use pulumi_wasm_rust::Output;
    use std::collections::HashMap;
    let endpoint_binding = args.endpoint.get_inner();
    let host_binding = args.host.get_inner();
    let method_binding = args.method.get_inner();
    let zone_id_binding = args.zone_id.get_inner();
    let request = register_interface::RegisterResourceRequest {
        type_: "cloudflare:index/apiShieldOperation:ApiShieldOperation".into(),
        name: name.to_string(),
        object: Vec::from([
            register_interface::ObjectField {
                name: "endpoint".into(),
                value: &endpoint_binding,
            },
            register_interface::ObjectField {
                name: "host".into(),
                value: &host_binding,
            },
            register_interface::ObjectField {
                name: "method".into(),
                value: &method_binding,
            },
            register_interface::ObjectField {
                name: "zoneId".into(),
                value: &zone_id_binding,
            },
        ]),
        results: vec![
            register_interface::ResultField { name : "endpoint".into() },
            register_interface::ResultField { name : "host".into() },
            register_interface::ResultField { name : "method".into() },
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
    ApiShieldOperationResult {
        endpoint: into_domain(hashmap.remove("endpoint").unwrap()),
        host: into_domain(hashmap.remove("host").unwrap()),
        method: into_domain(hashmap.remove("method").unwrap()),
        zone_id: into_domain(hashmap.remove("zoneId").unwrap()),
    }
}