/// Represents a RegionTargetHttpProxy resource, which is used by one or more
/// forwarding rules to route incoming HTTP requests to a URL map.
///
///
/// To get more information about RegionTargetHttpProxy, see:
///
/// * [API documentation](https://cloud.google.com/compute/docs/reference/rest/v1/regionTargetHttpProxies)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/compute/docs/load-balancing/http/target-proxies)
///
/// ## Example Usage
///
/// ### Region Target Http Proxy Basic
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = region_target_http_proxy::create(
///         "default",
///         RegionTargetHttpProxyArgs::builder()
///             .name("test-proxy")
///             .region("us-central1")
///             .url_map("${defaultRegionUrlMap.id}")
///             .build_struct(),
///     );
///     let defaultRegionBackendService = region_backend_service::create(
///         "defaultRegionBackendService",
///         RegionBackendServiceArgs::builder()
///             .health_checks("${defaultRegionHealthCheck.id}")
///             .load_balancing_scheme("INTERNAL_MANAGED")
///             .name("backend-service")
///             .protocol("HTTP")
///             .region("us-central1")
///             .timeout_sec(10)
///             .build_struct(),
///     );
///     let defaultRegionHealthCheck = region_health_check::create(
///         "defaultRegionHealthCheck",
///         RegionHealthCheckArgs::builder()
///             .http_health_check(
///                 RegionHealthCheckHttpHealthCheck::builder().port(80).build_struct(),
///             )
///             .name("http-health-check")
///             .region("us-central1")
///             .build_struct(),
///     );
///     let defaultRegionUrlMap = region_url_map::create(
///         "defaultRegionUrlMap",
///         RegionUrlMapArgs::builder()
///             .default_service("${defaultRegionBackendService.id}")
///             .host_rules(
///                 vec![
///                     RegionUrlMapHostRule::builder().hosts(vec!["mysite.com",])
///                     .pathMatcher("allpaths").build_struct(),
///                 ],
///             )
///             .name("url-map")
///             .path_matchers(
///                 vec![
///                     RegionUrlMapPathMatcher::builder()
///                     .defaultService("${defaultRegionBackendService.id}").name("allpaths")
///                     .pathRules(vec![RegionUrlMapPathMatcherPathRule::builder()
///                     .paths(vec!["/*",]).service("${defaultRegionBackendService.id}")
///                     .build_struct(),]).build_struct(),
///                 ],
///             )
///             .region("us-central1")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Region Target Http Proxy Http Keep Alive Timeout
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = region_target_http_proxy::create(
///         "default",
///         RegionTargetHttpProxyArgs::builder()
///             .http_keep_alive_timeout_sec(600)
///             .name("test-http-keep-alive-timeout-proxy")
///             .region("us-central1")
///             .url_map("${defaultRegionUrlMap.id}")
///             .build_struct(),
///     );
///     let defaultRegionBackendService = region_backend_service::create(
///         "defaultRegionBackendService",
///         RegionBackendServiceArgs::builder()
///             .health_checks("${defaultRegionHealthCheck.id}")
///             .load_balancing_scheme("INTERNAL_MANAGED")
///             .name("backend-service")
///             .port_name("http")
///             .protocol("HTTP")
///             .region("us-central1")
///             .timeout_sec(10)
///             .build_struct(),
///     );
///     let defaultRegionHealthCheck = region_health_check::create(
///         "defaultRegionHealthCheck",
///         RegionHealthCheckArgs::builder()
///             .http_health_check(
///                 RegionHealthCheckHttpHealthCheck::builder().port(80).build_struct(),
///             )
///             .name("http-health-check")
///             .region("us-central1")
///             .build_struct(),
///     );
///     let defaultRegionUrlMap = region_url_map::create(
///         "defaultRegionUrlMap",
///         RegionUrlMapArgs::builder()
///             .default_service("${defaultRegionBackendService.id}")
///             .host_rules(
///                 vec![
///                     RegionUrlMapHostRule::builder().hosts(vec!["mysite.com",])
///                     .pathMatcher("allpaths").build_struct(),
///                 ],
///             )
///             .name("url-map")
///             .path_matchers(
///                 vec![
///                     RegionUrlMapPathMatcher::builder()
///                     .defaultService("${defaultRegionBackendService.id}").name("allpaths")
///                     .pathRules(vec![RegionUrlMapPathMatcherPathRule::builder()
///                     .paths(vec!["/*",]).service("${defaultRegionBackendService.id}")
///                     .build_struct(),]).build_struct(),
///                 ],
///             )
///             .region("us-central1")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Region Target Http Proxy Https Redirect
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = region_target_http_proxy::create(
///         "default",
///         RegionTargetHttpProxyArgs::builder()
///             .name("test-https-redirect-proxy")
///             .region("us-central1")
///             .url_map("${defaultRegionUrlMap.id}")
///             .build_struct(),
///     );
///     let defaultRegionUrlMap = region_url_map::create(
///         "defaultRegionUrlMap",
///         RegionUrlMapArgs::builder()
///             .default_url_redirect(
///                 RegionUrlMapDefaultUrlRedirect::builder()
///                     .httpsRedirect(true)
///                     .stripQuery(false)
///                     .build_struct(),
///             )
///             .name("url-map")
///             .region("us-central1")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// RegionTargetHttpProxy can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/regions/{{region}}/targetHttpProxies/{{name}}`
///
/// * `{{project}}/{{region}}/{{name}}`
///
/// * `{{region}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, RegionTargetHttpProxy can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:compute/regionTargetHttpProxy:RegionTargetHttpProxy default projects/{{project}}/regions/{{region}}/targetHttpProxies/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/regionTargetHttpProxy:RegionTargetHttpProxy default {{project}}/{{region}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/regionTargetHttpProxy:RegionTargetHttpProxy default {{region}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/regionTargetHttpProxy:RegionTargetHttpProxy default {{name}}
/// ```
///
pub mod region_target_http_proxy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RegionTargetHttpProxyArgs {
        /// An optional description of this resource.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Specifies how long to keep a connection open, after completing a response,
        /// while there is no matching traffic (in seconds). If an HTTP keepalive is
        /// not specified, a default value (600 seconds) will be used. For Regional
        /// HTTP(S) load balancer, the minimum allowed value is 5 seconds and the
        /// maximum allowed value is 600 seconds.
        #[builder(into, default)]
        pub http_keep_alive_timeout_sec: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// Name of the resource. Provided by the client when the resource is
        /// created. The name must be 1-63 characters long, and comply with
        /// RFC1035. Specifically, the name must be 1-63 characters long and match
        /// the regular expression `a-z?` which means the
        /// first character must be a lowercase letter, and all following
        /// characters must be a dash, lowercase letter, or digit, except the last
        /// character, which cannot be a dash.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The Region in which the created target https proxy should reside.
        /// If it is not provided, the provider region is used.
        #[builder(into, default)]
        pub region: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A reference to the RegionUrlMap resource that defines the mapping from URL
        /// to the BackendService.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub url_map: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct RegionTargetHttpProxyResult {
        /// Creation timestamp in RFC3339 text format.
        pub creation_timestamp: pulumi_wasm_rust::Output<String>,
        /// An optional description of this resource.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies how long to keep a connection open, after completing a response,
        /// while there is no matching traffic (in seconds). If an HTTP keepalive is
        /// not specified, a default value (600 seconds) will be used. For Regional
        /// HTTP(S) load balancer, the minimum allowed value is 5 seconds and the
        /// maximum allowed value is 600 seconds.
        pub http_keep_alive_timeout_sec: pulumi_wasm_rust::Output<Option<i32>>,
        /// Name of the resource. Provided by the client when the resource is
        /// created. The name must be 1-63 characters long, and comply with
        /// RFC1035. Specifically, the name must be 1-63 characters long and match
        /// the regular expression `a-z?` which means the
        /// first character must be a lowercase letter, and all following
        /// characters must be a dash, lowercase letter, or digit, except the last
        /// character, which cannot be a dash.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// The unique identifier for the resource.
        pub proxy_id: pulumi_wasm_rust::Output<i32>,
        /// The Region in which the created target https proxy should reside.
        /// If it is not provided, the provider region is used.
        pub region: pulumi_wasm_rust::Output<String>,
        /// The URI of the created resource.
        pub self_link: pulumi_wasm_rust::Output<String>,
        /// A reference to the RegionUrlMap resource that defines the mapping from URL
        /// to the BackendService.
        ///
        ///
        /// - - -
        pub url_map: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: RegionTargetHttpProxyArgs,
    ) -> RegionTargetHttpProxyResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_output(context).get_inner();
        let http_keep_alive_timeout_sec_binding = args
            .http_keep_alive_timeout_sec
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let region_binding = args.region.get_output(context).get_inner();
        let url_map_binding = args.url_map.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:compute/regionTargetHttpProxy:RegionTargetHttpProxy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "httpKeepAliveTimeoutSec".into(),
                    value: &http_keep_alive_timeout_sec_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "region".into(),
                    value: &region_binding,
                },
                register_interface::ObjectField {
                    name: "urlMap".into(),
                    value: &url_map_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        RegionTargetHttpProxyResult {
            creation_timestamp: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("creationTimestamp"),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            http_keep_alive_timeout_sec: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("httpKeepAliveTimeoutSec"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            project: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            proxy_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("proxyId"),
            ),
            region: pulumi_wasm_rust::__private::into_domain(o.extract_field("region")),
            self_link: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("selfLink"),
            ),
            url_map: pulumi_wasm_rust::__private::into_domain(o.extract_field("urlMap")),
        }
    }
}
