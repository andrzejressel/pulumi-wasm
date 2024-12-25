#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct HealthcheckArgs {
    /// The hostname or IP address of the origin server to run health checks on.
    #[builder(into)]
    pub address: pulumi_wasm_rust::Output<String>,
    /// Do not validate the certificate when the health check uses HTTPS. Defaults to `false`.
    #[builder(into, default)]
    pub allow_insecure: pulumi_wasm_rust::Output<Option<bool>>,
    /// A list of regions from which to run health checks. If not set, Cloudflare will pick a default region. Available values: `WNAM`, `ENAM`, `WEU`, `EEU`, `NSAM`, `SSAM`, `OC`, `ME`, `NAF`, `SAF`, `IN`, `SEAS`, `NEAS`, `ALL_REGIONS`.
    #[builder(into, default)]
    pub check_regions: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    /// The number of consecutive fails required from a health check before changing the health to unhealthy. Defaults to `1`.
    #[builder(into, default)]
    pub consecutive_fails: pulumi_wasm_rust::Output<Option<i32>>,
    /// The number of consecutive successes required from a health check before changing the health to healthy. Defaults to `1`.
    #[builder(into, default)]
    pub consecutive_successes: pulumi_wasm_rust::Output<Option<i32>>,
    /// A human-readable description of the health check.
    #[builder(into, default)]
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    /// A case-insensitive sub-string to look for in the response body. If this string is not found the origin will be marked as unhealthy.
    #[builder(into, default)]
    pub expected_body: pulumi_wasm_rust::Output<Option<String>>,
    /// The expected HTTP response codes (e.g. '200') or code ranges (e.g. '2xx' for all codes starting with 2) of the health check.
    #[builder(into, default)]
    pub expected_codes: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    /// Follow redirects if the origin returns a 3xx status code. Defaults to `false`.
    #[builder(into, default)]
    pub follow_redirects: pulumi_wasm_rust::Output<Option<bool>>,
    /// The HTTP request headers to send in the health check. It is recommended you set a Host header by default. The User-Agent header cannot be overridden.
    #[builder(into, default)]
    pub headers: pulumi_wasm_rust::Output<Option<Vec<super::types::HealthcheckHeader>>>,
    /// The interval between each health check. Shorter intervals may give quicker notifications if the origin status changes, but will increase the load on the origin as we check from multiple locations. Defaults to `60`.
    #[builder(into, default)]
    pub interval: pulumi_wasm_rust::Output<Option<i32>>,
    /// The HTTP method to use for the health check. Available values: `connection_established`, `GET`, `HEAD`.
    #[builder(into, default)]
    pub method: pulumi_wasm_rust::Output<Option<String>>,
    /// A short name to identify the health check. Only alphanumeric characters, hyphens, and underscores are allowed.
    #[builder(into)]
    pub name: pulumi_wasm_rust::Output<String>,
    /// The endpoint path to health check against. Defaults to `/`.
    #[builder(into, default)]
    pub path: pulumi_wasm_rust::Output<Option<String>>,
    /// Port number to connect to for the health check. Defaults to `80`.
    #[builder(into, default)]
    pub port: pulumi_wasm_rust::Output<Option<i32>>,
    /// The number of retries to attempt in case of a timeout before marking the origin as unhealthy. Retries are attempted immediately. Defaults to `2`.
    #[builder(into, default)]
    pub retries: pulumi_wasm_rust::Output<Option<i32>>,
    /// If suspended, no health checks are sent to the origin. Defaults to `false`.
    #[builder(into, default)]
    pub suspended: pulumi_wasm_rust::Output<Option<bool>>,
    /// The timeout (in seconds) before marking the health check as failed. Defaults to `5`.
    #[builder(into, default)]
    pub timeout: pulumi_wasm_rust::Output<Option<i32>>,
    /// The protocol to use for the health check. Available values: `TCP`, `HTTP`, `HTTPS`.
    #[builder(into)]
    pub type_: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub zone_id: pulumi_wasm_rust::Output<String>,
}
pub struct HealthcheckResult {
    /// The hostname or IP address of the origin server to run health checks on.
    pub address: pulumi_wasm_rust::Output<String>,
    /// Do not validate the certificate when the health check uses HTTPS. Defaults to `false`.
    pub allow_insecure: pulumi_wasm_rust::Output<Option<bool>>,
    /// A list of regions from which to run health checks. If not set, Cloudflare will pick a default region. Available values: `WNAM`, `ENAM`, `WEU`, `EEU`, `NSAM`, `SSAM`, `OC`, `ME`, `NAF`, `SAF`, `IN`, `SEAS`, `NEAS`, `ALL_REGIONS`.
    pub check_regions: pulumi_wasm_rust::Output<Vec<String>>,
    /// The number of consecutive fails required from a health check before changing the health to unhealthy. Defaults to `1`.
    pub consecutive_fails: pulumi_wasm_rust::Output<Option<i32>>,
    /// The number of consecutive successes required from a health check before changing the health to healthy. Defaults to `1`.
    pub consecutive_successes: pulumi_wasm_rust::Output<Option<i32>>,
    /// Creation time.
    pub created_on: pulumi_wasm_rust::Output<String>,
    /// A human-readable description of the health check.
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    /// A case-insensitive sub-string to look for in the response body. If this string is not found the origin will be marked as unhealthy.
    pub expected_body: pulumi_wasm_rust::Output<Option<String>>,
    /// The expected HTTP response codes (e.g. '200') or code ranges (e.g. '2xx' for all codes starting with 2) of the health check.
    pub expected_codes: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    /// Follow redirects if the origin returns a 3xx status code. Defaults to `false`.
    pub follow_redirects: pulumi_wasm_rust::Output<Option<bool>>,
    /// The HTTP request headers to send in the health check. It is recommended you set a Host header by default. The User-Agent header cannot be overridden.
    pub headers: pulumi_wasm_rust::Output<Option<Vec<super::types::HealthcheckHeader>>>,
    /// The interval between each health check. Shorter intervals may give quicker notifications if the origin status changes, but will increase the load on the origin as we check from multiple locations. Defaults to `60`.
    pub interval: pulumi_wasm_rust::Output<Option<i32>>,
    /// The HTTP method to use for the health check. Available values: `connection_established`, `GET`, `HEAD`.
    pub method: pulumi_wasm_rust::Output<String>,
    /// Last modified time.
    pub modified_on: pulumi_wasm_rust::Output<String>,
    /// A short name to identify the health check. Only alphanumeric characters, hyphens, and underscores are allowed.
    pub name: pulumi_wasm_rust::Output<String>,
    /// The endpoint path to health check against. Defaults to `/`.
    pub path: pulumi_wasm_rust::Output<Option<String>>,
    /// Port number to connect to for the health check. Defaults to `80`.
    pub port: pulumi_wasm_rust::Output<Option<i32>>,
    /// The number of retries to attempt in case of a timeout before marking the origin as unhealthy. Retries are attempted immediately. Defaults to `2`.
    pub retries: pulumi_wasm_rust::Output<Option<i32>>,
    /// If suspended, no health checks are sent to the origin. Defaults to `false`.
    pub suspended: pulumi_wasm_rust::Output<Option<bool>>,
    /// The timeout (in seconds) before marking the health check as failed. Defaults to `5`.
    pub timeout: pulumi_wasm_rust::Output<Option<i32>>,
    /// The protocol to use for the health check. Available values: `TCP`, `HTTP`, `HTTPS`.
    pub type_: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
    pub zone_id: pulumi_wasm_rust::Output<String>,
}
///
/// Registers a new resource with the given unique name and arguments
///
#[allow(non_snake_case, unused_imports)]
pub fn create(name: &str, args: HealthcheckArgs) -> HealthcheckResult {
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
    use pulumi_wasm_rust::Output;
    use std::collections::HashMap;
    let address_binding = args.address.get_inner();
    let allow_insecure_binding = args.allow_insecure.get_inner();
    let check_regions_binding = args.check_regions.get_inner();
    let consecutive_fails_binding = args.consecutive_fails.get_inner();
    let consecutive_successes_binding = args.consecutive_successes.get_inner();
    let description_binding = args.description.get_inner();
    let expected_body_binding = args.expected_body.get_inner();
    let expected_codes_binding = args.expected_codes.get_inner();
    let follow_redirects_binding = args.follow_redirects.get_inner();
    let headers_binding = args.headers.get_inner();
    let interval_binding = args.interval.get_inner();
    let method_binding = args.method.get_inner();
    let name_binding = args.name.get_inner();
    let path_binding = args.path.get_inner();
    let port_binding = args.port.get_inner();
    let retries_binding = args.retries.get_inner();
    let suspended_binding = args.suspended.get_inner();
    let timeout_binding = args.timeout.get_inner();
    let type__binding = args.type_.get_inner();
    let zone_id_binding = args.zone_id.get_inner();
    let request = register_interface::RegisterResourceRequest {
        type_: "cloudflare:index/healthcheck:Healthcheck".into(),
        name: name.to_string(),
        object: Vec::from([
            register_interface::ObjectField {
                name: "address".into(),
                value: &address_binding,
            },
            register_interface::ObjectField {
                name: "allowInsecure".into(),
                value: &allow_insecure_binding,
            },
            register_interface::ObjectField {
                name: "checkRegions".into(),
                value: &check_regions_binding,
            },
            register_interface::ObjectField {
                name: "consecutiveFails".into(),
                value: &consecutive_fails_binding,
            },
            register_interface::ObjectField {
                name: "consecutiveSuccesses".into(),
                value: &consecutive_successes_binding,
            },
            register_interface::ObjectField {
                name: "description".into(),
                value: &description_binding,
            },
            register_interface::ObjectField {
                name: "expectedBody".into(),
                value: &expected_body_binding,
            },
            register_interface::ObjectField {
                name: "expectedCodes".into(),
                value: &expected_codes_binding,
            },
            register_interface::ObjectField {
                name: "followRedirects".into(),
                value: &follow_redirects_binding,
            },
            register_interface::ObjectField {
                name: "headers".into(),
                value: &headers_binding,
            },
            register_interface::ObjectField {
                name: "interval".into(),
                value: &interval_binding,
            },
            register_interface::ObjectField {
                name: "method".into(),
                value: &method_binding,
            },
            register_interface::ObjectField {
                name: "name".into(),
                value: &name_binding,
            },
            register_interface::ObjectField {
                name: "path".into(),
                value: &path_binding,
            },
            register_interface::ObjectField {
                name: "port".into(),
                value: &port_binding,
            },
            register_interface::ObjectField {
                name: "retries".into(),
                value: &retries_binding,
            },
            register_interface::ObjectField {
                name: "suspended".into(),
                value: &suspended_binding,
            },
            register_interface::ObjectField {
                name: "timeout".into(),
                value: &timeout_binding,
            },
            register_interface::ObjectField {
                name: "type".into(),
                value: &type__binding,
            },
            register_interface::ObjectField {
                name: "zoneId".into(),
                value: &zone_id_binding,
            },
        ]),
        results: vec![
            register_interface::ResultField { name : "address".into() },
            register_interface::ResultField { name : "allowInsecure".into() },
            register_interface::ResultField { name : "checkRegions".into() },
            register_interface::ResultField { name : "consecutiveFails".into() },
            register_interface::ResultField { name : "consecutiveSuccesses".into() },
            register_interface::ResultField { name : "createdOn".into() },
            register_interface::ResultField { name : "description".into() },
            register_interface::ResultField { name : "expectedBody".into() },
            register_interface::ResultField { name : "expectedCodes".into() },
            register_interface::ResultField { name : "followRedirects".into() },
            register_interface::ResultField { name : "headers".into() },
            register_interface::ResultField { name : "interval".into() },
            register_interface::ResultField { name : "method".into() },
            register_interface::ResultField { name : "modifiedOn".into() },
            register_interface::ResultField { name : "name".into() },
            register_interface::ResultField { name : "path".into() },
            register_interface::ResultField { name : "port".into() },
            register_interface::ResultField { name : "retries".into() },
            register_interface::ResultField { name : "suspended".into() },
            register_interface::ResultField { name : "timeout".into() },
            register_interface::ResultField { name : "type".into() },
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
    HealthcheckResult {
        address: into_domain(hashmap.remove("address").unwrap()),
        allow_insecure: into_domain(hashmap.remove("allowInsecure").unwrap()),
        check_regions: into_domain(hashmap.remove("checkRegions").unwrap()),
        consecutive_fails: into_domain(hashmap.remove("consecutiveFails").unwrap()),
        consecutive_successes: into_domain(
            hashmap.remove("consecutiveSuccesses").unwrap(),
        ),
        created_on: into_domain(hashmap.remove("createdOn").unwrap()),
        description: into_domain(hashmap.remove("description").unwrap()),
        expected_body: into_domain(hashmap.remove("expectedBody").unwrap()),
        expected_codes: into_domain(hashmap.remove("expectedCodes").unwrap()),
        follow_redirects: into_domain(hashmap.remove("followRedirects").unwrap()),
        headers: into_domain(hashmap.remove("headers").unwrap()),
        interval: into_domain(hashmap.remove("interval").unwrap()),
        method: into_domain(hashmap.remove("method").unwrap()),
        modified_on: into_domain(hashmap.remove("modifiedOn").unwrap()),
        name: into_domain(hashmap.remove("name").unwrap()),
        path: into_domain(hashmap.remove("path").unwrap()),
        port: into_domain(hashmap.remove("port").unwrap()),
        retries: into_domain(hashmap.remove("retries").unwrap()),
        suspended: into_domain(hashmap.remove("suspended").unwrap()),
        timeout: into_domain(hashmap.remove("timeout").unwrap()),
        type_: into_domain(hashmap.remove("type").unwrap()),
        zone_id: into_domain(hashmap.remove("zoneId").unwrap()),
    }
}
