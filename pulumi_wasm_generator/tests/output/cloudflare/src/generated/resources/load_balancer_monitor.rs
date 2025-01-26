/// If Cloudflare's Load Balancing to load-balance across multiple
/// origin servers or data centers, you configure one of these Monitors
/// to actively check the availability of those servers over HTTP(S) or
/// TCP.
///
/// ## Import
///
/// ```sh
/// $ pulumi import cloudflare:index/loadBalancerMonitor:LoadBalancerMonitor example <account_id>/<load_balancer_monitor_id>
/// ```
///
pub mod load_balancer_monitor {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LoadBalancerMonitorArgs {
        /// The account identifier to target for the resource.
        #[builder(into)]
        pub account_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// Do not validate the certificate when monitor use HTTPS.  Only valid if `type` is "http" or "https".
        #[builder(into, default)]
        pub allow_insecure: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// To be marked unhealthy the monitored origin must fail this healthcheck N consecutive times. Defaults to `0`.
        #[builder(into, default)]
        pub consecutive_down: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// To be marked healthy the monitored origin must pass this healthcheck N consecutive times. Defaults to `0`.
        #[builder(into, default)]
        pub consecutive_up: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// Free text description.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A case-insensitive sub-string to look for in the response body. If this string is not found, the origin will be marked as unhealthy. Only valid if `type` is "http" or "https".
        #[builder(into, default)]
        pub expected_body: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The expected HTTP response code or code range of the health check. Eg `2xx`. Only valid and required if `type` is "http" or "https".
        #[builder(into, default)]
        pub expected_codes: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Follow redirects if returned by the origin. Only valid if `type` is "http" or "https".
        #[builder(into, default)]
        pub follow_redirects: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// The HTTP request headers to send in the health check. It is recommended you set a Host header by default. The User-Agent header cannot be overridden.
        #[builder(into, default)]
        pub headers: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::types::LoadBalancerMonitorHeader>>,
        >,
        /// The interval between each health check. Shorter intervals may improve failover time, but will increase load on the origins as we check from multiple locations. Defaults to `60`.
        #[builder(into, default)]
        pub interval: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// The method to use for the health check.
        #[builder(into, default)]
        pub method: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The endpoint path to health check against.
        #[builder(into, default)]
        pub path: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The port number to use for the healthcheck, required when creating a TCP monitor.
        #[builder(into, default)]
        pub port: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// Assign this monitor to emulate the specified zone while probing. Only valid if `type` is "http" or "https".
        #[builder(into, default)]
        pub probe_zone: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The number of retries to attempt in case of a timeout before marking the origin as unhealthy. Retries are attempted immediately. Defaults to `2`.
        #[builder(into, default)]
        pub retries: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// The timeout (in seconds) before marking the health check as failed. Defaults to `5`.
        #[builder(into, default)]
        pub timeout: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// The protocol to use for the healthcheck. Available values: `http`, `https`, `tcp`, `udp_icmp`, `icmp_ping`, `smtp`. Defaults to `http`.
        #[builder(into, default)]
        pub type_: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct LoadBalancerMonitorResult {
        /// The account identifier to target for the resource.
        pub account_id: pulumi_wasm_rust::Output<String>,
        /// Do not validate the certificate when monitor use HTTPS.  Only valid if `type` is "http" or "https".
        pub allow_insecure: pulumi_wasm_rust::Output<Option<bool>>,
        /// To be marked unhealthy the monitored origin must fail this healthcheck N consecutive times. Defaults to `0`.
        pub consecutive_down: pulumi_wasm_rust::Output<Option<i32>>,
        /// To be marked healthy the monitored origin must pass this healthcheck N consecutive times. Defaults to `0`.
        pub consecutive_up: pulumi_wasm_rust::Output<Option<i32>>,
        /// The RFC3339 timestamp of when the load balancer monitor was created.
        pub created_on: pulumi_wasm_rust::Output<String>,
        /// Free text description.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// A case-insensitive sub-string to look for in the response body. If this string is not found, the origin will be marked as unhealthy. Only valid if `type` is "http" or "https".
        pub expected_body: pulumi_wasm_rust::Output<Option<String>>,
        /// The expected HTTP response code or code range of the health check. Eg `2xx`. Only valid and required if `type` is "http" or "https".
        pub expected_codes: pulumi_wasm_rust::Output<Option<String>>,
        /// Follow redirects if returned by the origin. Only valid if `type` is "http" or "https".
        pub follow_redirects: pulumi_wasm_rust::Output<Option<bool>>,
        /// The HTTP request headers to send in the health check. It is recommended you set a Host header by default. The User-Agent header cannot be overridden.
        pub headers: pulumi_wasm_rust::Output<
            Option<Vec<super::types::LoadBalancerMonitorHeader>>,
        >,
        /// The interval between each health check. Shorter intervals may improve failover time, but will increase load on the origins as we check from multiple locations. Defaults to `60`.
        pub interval: pulumi_wasm_rust::Output<Option<i32>>,
        /// The method to use for the health check.
        pub method: pulumi_wasm_rust::Output<String>,
        /// The RFC3339 timestamp of when the load balancer monitor was last modified.
        pub modified_on: pulumi_wasm_rust::Output<String>,
        /// The endpoint path to health check against.
        pub path: pulumi_wasm_rust::Output<String>,
        /// The port number to use for the healthcheck, required when creating a TCP monitor.
        pub port: pulumi_wasm_rust::Output<Option<i32>>,
        /// Assign this monitor to emulate the specified zone while probing. Only valid if `type` is "http" or "https".
        pub probe_zone: pulumi_wasm_rust::Output<Option<String>>,
        /// The number of retries to attempt in case of a timeout before marking the origin as unhealthy. Retries are attempted immediately. Defaults to `2`.
        pub retries: pulumi_wasm_rust::Output<Option<i32>>,
        /// The timeout (in seconds) before marking the health check as failed. Defaults to `5`.
        pub timeout: pulumi_wasm_rust::Output<Option<i32>>,
        /// The protocol to use for the healthcheck. Available values: `http`, `https`, `tcp`, `udp_icmp`, `icmp_ping`, `smtp`. Defaults to `http`.
        pub type_: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: LoadBalancerMonitorArgs,
    ) -> LoadBalancerMonitorResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let account_id_binding = args.account_id.get_output(context).get_inner();
        let allow_insecure_binding = args.allow_insecure.get_output(context).get_inner();
        let consecutive_down_binding = args
            .consecutive_down
            .get_output(context)
            .get_inner();
        let consecutive_up_binding = args.consecutive_up.get_output(context).get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let expected_body_binding = args.expected_body.get_output(context).get_inner();
        let expected_codes_binding = args.expected_codes.get_output(context).get_inner();
        let follow_redirects_binding = args
            .follow_redirects
            .get_output(context)
            .get_inner();
        let headers_binding = args.headers.get_output(context).get_inner();
        let interval_binding = args.interval.get_output(context).get_inner();
        let method_binding = args.method.get_output(context).get_inner();
        let path_binding = args.path.get_output(context).get_inner();
        let port_binding = args.port.get_output(context).get_inner();
        let probe_zone_binding = args.probe_zone.get_output(context).get_inner();
        let retries_binding = args.retries.get_output(context).get_inner();
        let timeout_binding = args.timeout.get_output(context).get_inner();
        let type__binding = args.type_.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "cloudflare:index/loadBalancerMonitor:LoadBalancerMonitor".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding,
                },
                register_interface::ObjectField {
                    name: "allowInsecure".into(),
                    value: &allow_insecure_binding,
                },
                register_interface::ObjectField {
                    name: "consecutiveDown".into(),
                    value: &consecutive_down_binding,
                },
                register_interface::ObjectField {
                    name: "consecutiveUp".into(),
                    value: &consecutive_up_binding,
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
                    name: "path".into(),
                    value: &path_binding,
                },
                register_interface::ObjectField {
                    name: "port".into(),
                    value: &port_binding,
                },
                register_interface::ObjectField {
                    name: "probeZone".into(),
                    value: &probe_zone_binding,
                },
                register_interface::ObjectField {
                    name: "retries".into(),
                    value: &retries_binding,
                },
                register_interface::ObjectField {
                    name: "timeout".into(),
                    value: &timeout_binding,
                },
                register_interface::ObjectField {
                    name: "type".into(),
                    value: &type__binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "accountId".into(),
                },
                register_interface::ResultField {
                    name: "allowInsecure".into(),
                },
                register_interface::ResultField {
                    name: "consecutiveDown".into(),
                },
                register_interface::ResultField {
                    name: "consecutiveUp".into(),
                },
                register_interface::ResultField {
                    name: "createdOn".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "expectedBody".into(),
                },
                register_interface::ResultField {
                    name: "expectedCodes".into(),
                },
                register_interface::ResultField {
                    name: "followRedirects".into(),
                },
                register_interface::ResultField {
                    name: "headers".into(),
                },
                register_interface::ResultField {
                    name: "interval".into(),
                },
                register_interface::ResultField {
                    name: "method".into(),
                },
                register_interface::ResultField {
                    name: "modifiedOn".into(),
                },
                register_interface::ResultField {
                    name: "path".into(),
                },
                register_interface::ResultField {
                    name: "port".into(),
                },
                register_interface::ResultField {
                    name: "probeZone".into(),
                },
                register_interface::ResultField {
                    name: "retries".into(),
                },
                register_interface::ResultField {
                    name: "timeout".into(),
                },
                register_interface::ResultField {
                    name: "type".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        LoadBalancerMonitorResult {
            account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accountId").unwrap(),
            ),
            allow_insecure: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("allowInsecure").unwrap(),
            ),
            consecutive_down: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("consecutiveDown").unwrap(),
            ),
            consecutive_up: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("consecutiveUp").unwrap(),
            ),
            created_on: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createdOn").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            expected_body: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("expectedBody").unwrap(),
            ),
            expected_codes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("expectedCodes").unwrap(),
            ),
            follow_redirects: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("followRedirects").unwrap(),
            ),
            headers: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("headers").unwrap(),
            ),
            interval: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("interval").unwrap(),
            ),
            method: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("method").unwrap(),
            ),
            modified_on: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("modifiedOn").unwrap(),
            ),
            path: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("path").unwrap(),
            ),
            port: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("port").unwrap(),
            ),
            probe_zone: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("probeZone").unwrap(),
            ),
            retries: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("retries").unwrap(),
            ),
            timeout: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("timeout").unwrap(),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("type").unwrap(),
            ),
        }
    }
}
