#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct AuthenticatedOriginPullsArgs {
    /// The ID of an uploaded Authenticated Origin Pulls certificate. If no hostname is provided, this certificate will be used zone wide as Per-Zone Authenticated Origin Pulls.
    #[builder(into, default)]
    pub authenticated_origin_pulls_certificate: pulumi_wasm_rust::Output<Option<String>>,
    /// Whether to enable Authenticated Origin Pulls on the given zone or hostname.
    #[builder(into)]
    pub enabled: pulumi_wasm_rust::Output<bool>,
    /// Specify a hostname to enable Per-Hostname Authenticated Origin Pulls on, using the provided certificate.
    #[builder(into, default)]
    pub hostname: pulumi_wasm_rust::Output<Option<String>>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub zone_id: pulumi_wasm_rust::Output<String>,
}
pub struct AuthenticatedOriginPullsResult {
    /// The ID of an uploaded Authenticated Origin Pulls certificate. If no hostname is provided, this certificate will be used zone wide as Per-Zone Authenticated Origin Pulls.
    pub authenticated_origin_pulls_certificate: pulumi_wasm_rust::Output<Option<String>>,
    /// Whether to enable Authenticated Origin Pulls on the given zone or hostname.
    pub enabled: pulumi_wasm_rust::Output<bool>,
    /// Specify a hostname to enable Per-Hostname Authenticated Origin Pulls on, using the provided certificate.
    pub hostname: pulumi_wasm_rust::Output<Option<String>>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub zone_id: pulumi_wasm_rust::Output<String>,
}
///
/// Registers a new resource with the given unique name and arguments
///
#[allow(non_snake_case, unused_imports)]
pub fn create(
    name: &str,
    args: AuthenticatedOriginPullsArgs,
) -> AuthenticatedOriginPullsResult {
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
    use pulumi_wasm_rust::Output;
    use std::collections::HashMap;
    let authenticated_origin_pulls_certificate_binding = args
        .authenticated_origin_pulls_certificate
        .get_inner();
    let enabled_binding = args.enabled.get_inner();
    let hostname_binding = args.hostname.get_inner();
    let zone_id_binding = args.zone_id.get_inner();
    let request = register_interface::RegisterResourceRequest {
        type_: "cloudflare:index/authenticatedOriginPulls:AuthenticatedOriginPulls"
            .into(),
        name: name.to_string(),
        object: Vec::from([
            register_interface::ObjectField {
                name: "authenticatedOriginPullsCertificate".into(),
                value: &authenticated_origin_pulls_certificate_binding,
            },
            register_interface::ObjectField {
                name: "enabled".into(),
                value: &enabled_binding,
            },
            register_interface::ObjectField {
                name: "hostname".into(),
                value: &hostname_binding,
            },
            register_interface::ObjectField {
                name: "zoneId".into(),
                value: &zone_id_binding,
            },
        ]),
        results: vec![
            register_interface::ResultField { name :
            "authenticatedOriginPullsCertificate".into() },
            register_interface::ResultField { name : "enabled".into() },
            register_interface::ResultField { name : "hostname".into() },
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
    AuthenticatedOriginPullsResult {
        authenticated_origin_pulls_certificate: into_domain(
            hashmap.remove("authenticatedOriginPullsCertificate").unwrap(),
        ),
        enabled: into_domain(hashmap.remove("enabled").unwrap()),
        hostname: into_domain(hashmap.remove("hostname").unwrap()),
        zone_id: into_domain(hashmap.remove("zoneId").unwrap()),
    }
}
