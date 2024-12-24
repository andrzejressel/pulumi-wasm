#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct KeylessCertificateArgs {
    /// A ubiquitous bundle has the highest probability of being verified everywhere, even by clients using outdated or unusual trust stores. An optimal bundle uses the shortest chain and newest intermediates. And the force bundle verifies the chain, but does not otherwise modify it. Available values: `ubiquitous`, `optimal`, `force`. Defaults to `ubiquitous`. **Modifying this attribute will force creation of a new resource.**
    #[builder(into, default)]
    pub bundle_method: pulumi_wasm_rust::Output<Option<String>>,
    /// The zone's SSL certificate or SSL certificate and intermediate(s). **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub certificate: pulumi_wasm_rust::Output<String>,
    /// Whether the KeyLess SSL is on.
    #[builder(into, default)]
    pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
    /// The KeyLess SSL host.
    #[builder(into)]
    pub host: pulumi_wasm_rust::Output<String>,
    /// The KeyLess SSL name.
    #[builder(into, default)]
    pub name: pulumi_wasm_rust::Output<Option<String>>,
    /// The KeyLess SSL port used to communicate between Cloudflare and the client's KeyLess SSL server. Defaults to `24008`.
    #[builder(into, default)]
    pub port: pulumi_wasm_rust::Output<Option<i32>>,
    /// The zone identifier to target for the resource.
    #[builder(into)]
    pub zone_id: pulumi_wasm_rust::Output<String>,
}
pub struct KeylessCertificateResult {
    /// A ubiquitous bundle has the highest probability of being verified everywhere, even by clients using outdated or unusual trust stores. An optimal bundle uses the shortest chain and newest intermediates. And the force bundle verifies the chain, but does not otherwise modify it. Available values: `ubiquitous`, `optimal`, `force`. Defaults to `ubiquitous`. **Modifying this attribute will force creation of a new resource.**
    pub bundle_method: pulumi_wasm_rust::Output<Option<String>>,
    /// The zone's SSL certificate or SSL certificate and intermediate(s). **Modifying this attribute will force creation of a new resource.**
    pub certificate: pulumi_wasm_rust::Output<String>,
    /// Whether the KeyLess SSL is on.
    pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
    /// The KeyLess SSL host.
    pub host: pulumi_wasm_rust::Output<String>,
    /// The KeyLess SSL name.
    pub name: pulumi_wasm_rust::Output<Option<String>>,
    /// The KeyLess SSL port used to communicate between Cloudflare and the client's KeyLess SSL server. Defaults to `24008`.
    pub port: pulumi_wasm_rust::Output<Option<i32>>,
    /// Status of the KeyLess SSL.
    pub status: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource.
    pub zone_id: pulumi_wasm_rust::Output<String>,
}
///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: KeylessCertificateArgs) -> KeylessCertificateResult {
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
    use pulumi_wasm_rust::Output;
    use std::collections::HashMap;
    let bundle_method_binding = args.bundle_method.get_inner();
    let certificate_binding = args.certificate.get_inner();
    let enabled_binding = args.enabled.get_inner();
    let host_binding = args.host.get_inner();
    let name_binding = args.name.get_inner();
    let port_binding = args.port.get_inner();
    let zone_id_binding = args.zone_id.get_inner();
    let request = register_interface::RegisterResourceRequest {
        type_: "cloudflare:index/keylessCertificate:KeylessCertificate".into(),
        name: name.to_string(),
        object: Vec::from([
            register_interface::ObjectField {
                name: "bundleMethod".into(),
                value: &bundle_method_binding,
            },
            register_interface::ObjectField {
                name: "certificate".into(),
                value: &certificate_binding,
            },
            register_interface::ObjectField {
                name: "enabled".into(),
                value: &enabled_binding,
            },
            register_interface::ObjectField {
                name: "host".into(),
                value: &host_binding,
            },
            register_interface::ObjectField {
                name: "name".into(),
                value: &name_binding,
            },
            register_interface::ObjectField {
                name: "port".into(),
                value: &port_binding,
            },
            register_interface::ObjectField {
                name: "zoneId".into(),
                value: &zone_id_binding,
            },
        ]),
        results: vec![
            register_interface::ResultField { name : "bundleMethod".into() },
            register_interface::ResultField { name : "certificate".into() },
            register_interface::ResultField { name : "enabled".into() },
            register_interface::ResultField { name : "host".into() },
            register_interface::ResultField { name : "name".into() },
            register_interface::ResultField { name : "port".into() },
            register_interface::ResultField { name : "status".into() },
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
    KeylessCertificateResult {
        bundle_method: into_domain(hashmap.remove("bundleMethod").unwrap()),
        certificate: into_domain(hashmap.remove("certificate").unwrap()),
        enabled: into_domain(hashmap.remove("enabled").unwrap()),
        host: into_domain(hashmap.remove("host").unwrap()),
        name: into_domain(hashmap.remove("name").unwrap()),
        port: into_domain(hashmap.remove("port").unwrap()),
        status: into_domain(hashmap.remove("status").unwrap()),
        zone_id: into_domain(hashmap.remove("zoneId").unwrap()),
    }
}
