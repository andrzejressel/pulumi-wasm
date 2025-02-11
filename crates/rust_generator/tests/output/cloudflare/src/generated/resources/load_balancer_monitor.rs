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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod load_balancer_monitor {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LoadBalancerMonitorArgs {
        /// The account identifier to target for the resource.
        #[builder(into)]
        pub account_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Do not validate the certificate when monitor use HTTPS.  Only valid if `type` is "http" or "https".
        #[builder(into, default)]
        pub allow_insecure: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// To be marked unhealthy the monitored origin must fail this healthcheck N consecutive times. Defaults to `0`.
        #[builder(into, default)]
        pub consecutive_down: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// To be marked healthy the monitored origin must pass this healthcheck N consecutive times. Defaults to `0`.
        #[builder(into, default)]
        pub consecutive_up: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Free text description.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A case-insensitive sub-string to look for in the response body. If this string is not found, the origin will be marked as unhealthy. Only valid if `type` is "http" or "https".
        #[builder(into, default)]
        pub expected_body: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The expected HTTP response code or code range of the health check. Eg `2xx`. Only valid and required if `type` is "http" or "https".
        #[builder(into, default)]
        pub expected_codes: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Follow redirects if returned by the origin. Only valid if `type` is "http" or "https".
        #[builder(into, default)]
        pub follow_redirects: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The HTTP request headers to send in the health check. It is recommended you set a Host header by default. The User-Agent header cannot be overridden.
        #[builder(into, default)]
        pub headers: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::types::LoadBalancerMonitorHeader>>,
        >,
        /// The interval between each health check. Shorter intervals may improve failover time, but will increase load on the origins as we check from multiple locations. Defaults to `60`.
        #[builder(into, default)]
        pub interval: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The method to use for the health check.
        #[builder(into, default)]
        pub method: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The endpoint path to health check against.
        #[builder(into, default)]
        pub path: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The port number to use for the healthcheck, required when creating a TCP monitor.
        #[builder(into, default)]
        pub port: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Assign this monitor to emulate the specified zone while probing. Only valid if `type` is "http" or "https".
        #[builder(into, default)]
        pub probe_zone: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The number of retries to attempt in case of a timeout before marking the origin as unhealthy. Retries are attempted immediately. Defaults to `2`.
        #[builder(into, default)]
        pub retries: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The timeout (in seconds) before marking the health check as failed. Defaults to `5`.
        #[builder(into, default)]
        pub timeout: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The protocol to use for the healthcheck. Available values: `http`, `https`, `tcp`, `udp_icmp`, `icmp_ping`, `smtp`. Defaults to `http`.
        #[builder(into, default)]
        pub type_: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct LoadBalancerMonitorResult {
        /// The account identifier to target for the resource.
        pub account_id: pulumi_gestalt_rust::Output<String>,
        /// Do not validate the certificate when monitor use HTTPS.  Only valid if `type` is "http" or "https".
        pub allow_insecure: pulumi_gestalt_rust::Output<Option<bool>>,
        /// To be marked unhealthy the monitored origin must fail this healthcheck N consecutive times. Defaults to `0`.
        pub consecutive_down: pulumi_gestalt_rust::Output<Option<i32>>,
        /// To be marked healthy the monitored origin must pass this healthcheck N consecutive times. Defaults to `0`.
        pub consecutive_up: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The RFC3339 timestamp of when the load balancer monitor was created.
        pub created_on: pulumi_gestalt_rust::Output<String>,
        /// Free text description.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// A case-insensitive sub-string to look for in the response body. If this string is not found, the origin will be marked as unhealthy. Only valid if `type` is "http" or "https".
        pub expected_body: pulumi_gestalt_rust::Output<Option<String>>,
        /// The expected HTTP response code or code range of the health check. Eg `2xx`. Only valid and required if `type` is "http" or "https".
        pub expected_codes: pulumi_gestalt_rust::Output<Option<String>>,
        /// Follow redirects if returned by the origin. Only valid if `type` is "http" or "https".
        pub follow_redirects: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The HTTP request headers to send in the health check. It is recommended you set a Host header by default. The User-Agent header cannot be overridden.
        pub headers: pulumi_gestalt_rust::Output<
            Option<Vec<super::types::LoadBalancerMonitorHeader>>,
        >,
        /// The interval between each health check. Shorter intervals may improve failover time, but will increase load on the origins as we check from multiple locations. Defaults to `60`.
        pub interval: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The method to use for the health check.
        pub method: pulumi_gestalt_rust::Output<String>,
        /// The RFC3339 timestamp of when the load balancer monitor was last modified.
        pub modified_on: pulumi_gestalt_rust::Output<String>,
        /// The endpoint path to health check against.
        pub path: pulumi_gestalt_rust::Output<String>,
        /// The port number to use for the healthcheck, required when creating a TCP monitor.
        pub port: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Assign this monitor to emulate the specified zone while probing. Only valid if `type` is "http" or "https".
        pub probe_zone: pulumi_gestalt_rust::Output<Option<String>>,
        /// The number of retries to attempt in case of a timeout before marking the origin as unhealthy. Retries are attempted immediately. Defaults to `2`.
        pub retries: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The timeout (in seconds) before marking the health check as failed. Defaults to `5`.
        pub timeout: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The protocol to use for the healthcheck. Available values: `http`, `https`, `tcp`, `udp_icmp`, `icmp_ping`, `smtp`. Defaults to `http`.
        pub type_: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: LoadBalancerMonitorArgs,
    ) -> LoadBalancerMonitorResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let account_id_binding = args.account_id.get_output(context);
        let allow_insecure_binding = args.allow_insecure.get_output(context);
        let consecutive_down_binding = args.consecutive_down.get_output(context);
        let consecutive_up_binding = args.consecutive_up.get_output(context);
        let description_binding = args.description.get_output(context);
        let expected_body_binding = args.expected_body.get_output(context);
        let expected_codes_binding = args.expected_codes.get_output(context);
        let follow_redirects_binding = args.follow_redirects.get_output(context);
        let headers_binding = args.headers.get_output(context);
        let interval_binding = args.interval.get_output(context);
        let method_binding = args.method.get_output(context);
        let path_binding = args.path.get_output(context);
        let port_binding = args.port.get_output(context);
        let probe_zone_binding = args.probe_zone.get_output(context);
        let retries_binding = args.retries.get_output(context);
        let timeout_binding = args.timeout.get_output(context);
        let type__binding = args.type_.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "cloudflare:index/loadBalancerMonitor:LoadBalancerMonitor".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "allowInsecure".into(),
                    value: &allow_insecure_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "consecutiveDown".into(),
                    value: &consecutive_down_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "consecutiveUp".into(),
                    value: &consecutive_up_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "expectedBody".into(),
                    value: &expected_body_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "expectedCodes".into(),
                    value: &expected_codes_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "followRedirects".into(),
                    value: &follow_redirects_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "headers".into(),
                    value: &headers_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "interval".into(),
                    value: &interval_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "method".into(),
                    value: &method_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "path".into(),
                    value: &path_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "port".into(),
                    value: &port_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "probeZone".into(),
                    value: &probe_zone_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "retries".into(),
                    value: &retries_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "timeout".into(),
                    value: &timeout_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "type".into(),
                    value: &type__binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        LoadBalancerMonitorResult {
            account_id: o.get_field("accountId"),
            allow_insecure: o.get_field("allowInsecure"),
            consecutive_down: o.get_field("consecutiveDown"),
            consecutive_up: o.get_field("consecutiveUp"),
            created_on: o.get_field("createdOn"),
            description: o.get_field("description"),
            expected_body: o.get_field("expectedBody"),
            expected_codes: o.get_field("expectedCodes"),
            follow_redirects: o.get_field("followRedirects"),
            headers: o.get_field("headers"),
            interval: o.get_field("interval"),
            method: o.get_field("method"),
            modified_on: o.get_field("modifiedOn"),
            path: o.get_field("path"),
            port: o.get_field("port"),
            probe_zone: o.get_field("probeZone"),
            retries: o.get_field("retries"),
            timeout: o.get_field("timeout"),
            type_: o.get_field("type"),
        }
    }
}
