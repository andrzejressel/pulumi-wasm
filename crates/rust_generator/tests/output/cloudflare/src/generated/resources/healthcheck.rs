/// Standalone Health Checks provide a way to monitor origin servers
/// without needing a Cloudflare Load Balancer.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let httpHealthCheck = healthcheck::create(
///         "httpHealthCheck",
///         HealthcheckArgs::builder()
///             .address("example.com")
///             .allow_insecure(false)
///             .check_regions(vec!["WEU", "EEU",])
///             .consecutive_fails(3)
///             .consecutive_successes(2)
///             .description("example http health check")
///             .expected_body("alive")
///             .expected_codes(vec!["2xx", "301",])
///             .follow_redirects(true)
///             .headers(
///                 vec![
///                     HealthcheckHeader::builder().header("Host")
///                     .values(vec!["example.com",]).build_struct(),
///                 ],
///             )
///             .interval(60)
///             .method("GET")
///             .name("http-health-check")
///             .path("/health")
///             .port(443)
///             .retries(2)
///             .suspended(false)
///             .timeout(10)
///             .type_("HTTPS")
///             .zone_id("${cloudflareZoneId}")
///             .build_struct(),
///     );
///     let tcpHealthCheck = healthcheck::create(
///         "tcpHealthCheck",
///         HealthcheckArgs::builder()
///             .address("example.com")
///             .check_regions(vec!["WEU", "EEU",])
///             .consecutive_fails(3)
///             .consecutive_successes(2)
///             .description("example tcp health check")
///             .interval(60)
///             .method("connection_established")
///             .name("tcp-health-check")
///             .port(22)
///             .retries(2)
///             .suspended(false)
///             .timeout(10)
///             .type_("TCP")
///             .zone_id("${cloudflareZoneId}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Use the Zone ID and Healthcheck ID to import.
///
/// ```sh
/// $ pulumi import cloudflare:index/healthcheck:Healthcheck example <zone_id>/<healthcheck_id>
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod healthcheck {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct HealthcheckArgs {
        /// The hostname or IP address of the origin server to run health checks on.
        #[builder(into)]
        pub address: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Do not validate the certificate when the health check uses HTTPS. Defaults to `false`.
        #[builder(into, default)]
        pub allow_insecure: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// A list of regions from which to run health checks. If not set, Cloudflare will pick a default region. Available values: `WNAM`, `ENAM`, `WEU`, `EEU`, `NSAM`, `SSAM`, `OC`, `ME`, `NAF`, `SAF`, `IN`, `SEAS`, `NEAS`, `ALL_REGIONS`.
        #[builder(into, default)]
        pub check_regions: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The number of consecutive fails required from a health check before changing the health to unhealthy. Defaults to `1`.
        #[builder(into, default)]
        pub consecutive_fails: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The number of consecutive successes required from a health check before changing the health to healthy. Defaults to `1`.
        #[builder(into, default)]
        pub consecutive_successes: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// A human-readable description of the health check.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A case-insensitive sub-string to look for in the response body. If this string is not found the origin will be marked as unhealthy.
        #[builder(into, default)]
        pub expected_body: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The expected HTTP response codes (e.g. '200') or code ranges (e.g. '2xx' for all codes starting with 2) of the health check.
        #[builder(into, default)]
        pub expected_codes: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Follow redirects if the origin returns a 3xx status code. Defaults to `false`.
        #[builder(into, default)]
        pub follow_redirects: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The HTTP request headers to send in the health check. It is recommended you set a Host header by default. The User-Agent header cannot be overridden.
        #[builder(into, default)]
        pub headers: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::types::HealthcheckHeader>>,
        >,
        /// The interval between each health check. Shorter intervals may give quicker notifications if the origin status changes, but will increase the load on the origin as we check from multiple locations. Defaults to `60`.
        #[builder(into, default)]
        pub interval: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The HTTP method to use for the health check. Available values: `connection_established`, `GET`, `HEAD`.
        #[builder(into, default)]
        pub method: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A short name to identify the health check. Only alphanumeric characters, hyphens, and underscores are allowed.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The endpoint path to health check against. Defaults to `/`.
        #[builder(into, default)]
        pub path: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Port number to connect to for the health check. Defaults to `80`.
        #[builder(into, default)]
        pub port: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The number of retries to attempt in case of a timeout before marking the origin as unhealthy. Retries are attempted immediately. Defaults to `2`.
        #[builder(into, default)]
        pub retries: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// If suspended, no health checks are sent to the origin. Defaults to `false`.
        #[builder(into, default)]
        pub suspended: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The timeout (in seconds) before marking the health check as failed. Defaults to `5`.
        #[builder(into, default)]
        pub timeout: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The protocol to use for the health check. Available values: `TCP`, `HTTP`, `HTTPS`.
        #[builder(into)]
        pub type_: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub zone_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct HealthcheckResult {
        /// The hostname or IP address of the origin server to run health checks on.
        pub address: pulumi_gestalt_rust::Output<String>,
        /// Do not validate the certificate when the health check uses HTTPS. Defaults to `false`.
        pub allow_insecure: pulumi_gestalt_rust::Output<Option<bool>>,
        /// A list of regions from which to run health checks. If not set, Cloudflare will pick a default region. Available values: `WNAM`, `ENAM`, `WEU`, `EEU`, `NSAM`, `SSAM`, `OC`, `ME`, `NAF`, `SAF`, `IN`, `SEAS`, `NEAS`, `ALL_REGIONS`.
        pub check_regions: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The number of consecutive fails required from a health check before changing the health to unhealthy. Defaults to `1`.
        pub consecutive_fails: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The number of consecutive successes required from a health check before changing the health to healthy. Defaults to `1`.
        pub consecutive_successes: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Creation time.
        pub created_on: pulumi_gestalt_rust::Output<String>,
        /// A human-readable description of the health check.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// A case-insensitive sub-string to look for in the response body. If this string is not found the origin will be marked as unhealthy.
        pub expected_body: pulumi_gestalt_rust::Output<Option<String>>,
        /// The expected HTTP response codes (e.g. '200') or code ranges (e.g. '2xx' for all codes starting with 2) of the health check.
        pub expected_codes: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Follow redirects if the origin returns a 3xx status code. Defaults to `false`.
        pub follow_redirects: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The HTTP request headers to send in the health check. It is recommended you set a Host header by default. The User-Agent header cannot be overridden.
        pub headers: pulumi_gestalt_rust::Output<
            Option<Vec<super::types::HealthcheckHeader>>,
        >,
        /// The interval between each health check. Shorter intervals may give quicker notifications if the origin status changes, but will increase the load on the origin as we check from multiple locations. Defaults to `60`.
        pub interval: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The HTTP method to use for the health check. Available values: `connection_established`, `GET`, `HEAD`.
        pub method: pulumi_gestalt_rust::Output<String>,
        /// Last modified time.
        pub modified_on: pulumi_gestalt_rust::Output<String>,
        /// A short name to identify the health check. Only alphanumeric characters, hyphens, and underscores are allowed.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The endpoint path to health check against. Defaults to `/`.
        pub path: pulumi_gestalt_rust::Output<Option<String>>,
        /// Port number to connect to for the health check. Defaults to `80`.
        pub port: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The number of retries to attempt in case of a timeout before marking the origin as unhealthy. Retries are attempted immediately. Defaults to `2`.
        pub retries: pulumi_gestalt_rust::Output<Option<i32>>,
        /// If suspended, no health checks are sent to the origin. Defaults to `false`.
        pub suspended: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The timeout (in seconds) before marking the health check as failed. Defaults to `5`.
        pub timeout: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The protocol to use for the health check. Available values: `TCP`, `HTTP`, `HTTPS`.
        pub type_: pulumi_gestalt_rust::Output<String>,
        /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        pub zone_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: HealthcheckArgs,
    ) -> HealthcheckResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let address_binding = args.address.get_output(context);
        let allow_insecure_binding = args.allow_insecure.get_output(context);
        let check_regions_binding = args.check_regions.get_output(context);
        let consecutive_fails_binding = args.consecutive_fails.get_output(context);
        let consecutive_successes_binding = args
            .consecutive_successes
            .get_output(context);
        let description_binding = args.description.get_output(context);
        let expected_body_binding = args.expected_body.get_output(context);
        let expected_codes_binding = args.expected_codes.get_output(context);
        let follow_redirects_binding = args.follow_redirects.get_output(context);
        let headers_binding = args.headers.get_output(context);
        let interval_binding = args.interval.get_output(context);
        let method_binding = args.method.get_output(context);
        let name_binding = args.name.get_output(context);
        let path_binding = args.path.get_output(context);
        let port_binding = args.port.get_output(context);
        let retries_binding = args.retries.get_output(context);
        let suspended_binding = args.suspended.get_output(context);
        let timeout_binding = args.timeout.get_output(context);
        let type__binding = args.type_.get_output(context);
        let zone_id_binding = args.zone_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "cloudflare:index/healthcheck:Healthcheck".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "address".into(),
                    value: address_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "allowInsecure".into(),
                    value: allow_insecure_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "checkRegions".into(),
                    value: check_regions_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "consecutiveFails".into(),
                    value: consecutive_fails_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "consecutiveSuccesses".into(),
                    value: consecutive_successes_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "expectedBody".into(),
                    value: expected_body_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "expectedCodes".into(),
                    value: expected_codes_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "followRedirects".into(),
                    value: follow_redirects_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "headers".into(),
                    value: headers_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "interval".into(),
                    value: interval_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "method".into(),
                    value: method_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "path".into(),
                    value: path_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "port".into(),
                    value: port_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "retries".into(),
                    value: retries_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "suspended".into(),
                    value: suspended_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "timeout".into(),
                    value: timeout_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "type".into(),
                    value: type__binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "zoneId".into(),
                    value: zone_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        HealthcheckResult {
            address: o.get_field("address"),
            allow_insecure: o.get_field("allowInsecure"),
            check_regions: o.get_field("checkRegions"),
            consecutive_fails: o.get_field("consecutiveFails"),
            consecutive_successes: o.get_field("consecutiveSuccesses"),
            created_on: o.get_field("createdOn"),
            description: o.get_field("description"),
            expected_body: o.get_field("expectedBody"),
            expected_codes: o.get_field("expectedCodes"),
            follow_redirects: o.get_field("followRedirects"),
            headers: o.get_field("headers"),
            interval: o.get_field("interval"),
            method: o.get_field("method"),
            modified_on: o.get_field("modifiedOn"),
            name: o.get_field("name"),
            path: o.get_field("path"),
            port: o.get_field("port"),
            retries: o.get_field("retries"),
            suspended: o.get_field("suspended"),
            timeout: o.get_field("timeout"),
            type_: o.get_field("type"),
            zone_id: o.get_field("zoneId"),
        }
    }
}
