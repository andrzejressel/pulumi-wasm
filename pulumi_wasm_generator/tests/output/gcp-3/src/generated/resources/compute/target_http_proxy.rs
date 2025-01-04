/// Represents a TargetHttpProxy resource, which is used by one or more global
/// forwarding rule to route incoming HTTP requests to a URL map.
///
///
/// To get more information about TargetHttpProxy, see:
///
/// * [API documentation](https://cloud.google.com/compute/docs/reference/v1/targetHttpProxies)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/compute/docs/load-balancing/http/target-proxies)
///
/// ## Example Usage
///
/// ### Target Http Proxy Basic
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = target_http_proxy::create(
///         "default",
///         TargetHttpProxyArgs::builder()
///             .name("test-proxy")
///             .url_map("${defaultURLMap.id}")
///             .build_struct(),
///     );
///     let defaultBackendService = backend_service::create(
///         "defaultBackendService",
///         BackendServiceArgs::builder()
///             .health_checks("${defaultHttpHealthCheck.id}")
///             .name("backend-service")
///             .port_name("http")
///             .protocol("HTTP")
///             .timeout_sec(10)
///             .build_struct(),
///     );
///     let defaultHttpHealthCheck = http_health_check::create(
///         "defaultHttpHealthCheck",
///         HttpHealthCheckArgs::builder()
///             .check_interval_sec(1)
///             .name("http-health-check")
///             .request_path("/")
///             .timeout_sec(1)
///             .build_struct(),
///     );
///     let defaultURLMap = url_map::create(
///         "defaultURLMap",
///         UrlMapArgs::builder()
///             .default_service("${defaultBackendService.id}")
///             .host_rules(
///                 vec![
///                     UrlMapHostRule::builder().hosts(vec!["mysite.com",])
///                     .pathMatcher("allpaths").build_struct(),
///                 ],
///             )
///             .name("url-map")
///             .path_matchers(
///                 vec![
///                     UrlMapPathMatcher::builder()
///                     .defaultService("${defaultBackendService.id}").name("allpaths")
///                     .pathRules(vec![UrlMapPathMatcherPathRule::builder()
///                     .paths(vec!["/*",]).service("${defaultBackendService.id}")
///                     .build_struct(),]).build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
/// ### Target Http Proxy Http Keep Alive Timeout
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = target_http_proxy::create(
///         "default",
///         TargetHttpProxyArgs::builder()
///             .http_keep_alive_timeout_sec(610)
///             .name("test-http-keep-alive-timeout-proxy")
///             .url_map("${defaultURLMap.id}")
///             .build_struct(),
///     );
///     let defaultBackendService = backend_service::create(
///         "defaultBackendService",
///         BackendServiceArgs::builder()
///             .health_checks("${defaultHttpHealthCheck.id}")
///             .load_balancing_scheme("EXTERNAL_MANAGED")
///             .name("backend-service")
///             .port_name("http")
///             .protocol("HTTP")
///             .timeout_sec(10)
///             .build_struct(),
///     );
///     let defaultHttpHealthCheck = http_health_check::create(
///         "defaultHttpHealthCheck",
///         HttpHealthCheckArgs::builder()
///             .check_interval_sec(1)
///             .name("http-health-check")
///             .request_path("/")
///             .timeout_sec(1)
///             .build_struct(),
///     );
///     let defaultURLMap = url_map::create(
///         "defaultURLMap",
///         UrlMapArgs::builder()
///             .default_service("${defaultBackendService.id}")
///             .host_rules(
///                 vec![
///                     UrlMapHostRule::builder().hosts(vec!["mysite.com",])
///                     .pathMatcher("allpaths").build_struct(),
///                 ],
///             )
///             .name("url-map")
///             .path_matchers(
///                 vec![
///                     UrlMapPathMatcher::builder()
///                     .defaultService("${defaultBackendService.id}").name("allpaths")
///                     .pathRules(vec![UrlMapPathMatcherPathRule::builder()
///                     .paths(vec!["/*",]).service("${defaultBackendService.id}")
///                     .build_struct(),]).build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
/// ### Target Http Proxy Https Redirect
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = target_http_proxy::create(
///         "default",
///         TargetHttpProxyArgs::builder()
///             .name("test-https-redirect-proxy")
///             .url_map("${defaultURLMap.id}")
///             .build_struct(),
///     );
///     let defaultURLMap = url_map::create(
///         "defaultURLMap",
///         UrlMapArgs::builder()
///             .default_url_redirect(
///                 UrlMapDefaultUrlRedirect::builder()
///                     .httpsRedirect(true)
///                     .stripQuery(false)
///                     .build_struct(),
///             )
///             .name("url-map")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// TargetHttpProxy can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/global/targetHttpProxies/{{name}}`
///
/// * `{{project}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, TargetHttpProxy can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:compute/targetHttpProxy:TargetHttpProxy default projects/{{project}}/global/targetHttpProxies/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/targetHttpProxy:TargetHttpProxy default {{project}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/targetHttpProxy:TargetHttpProxy default {{name}}
/// ```
///
pub mod target_http_proxy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TargetHttpProxyArgs {
        /// An optional description of this resource.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies how long to keep a connection open, after completing a response,
        /// while there is no matching traffic (in seconds). If an HTTP keepalive is
        /// not specified, a default value will be used. For Global
        /// external HTTP(S) load balancer, the default value is 610 seconds, the
        /// minimum allowed value is 5 seconds and the maximum allowed value is 1200
        /// seconds. For cross-region internal HTTP(S) load balancer, the default
        /// value is 600 seconds, the minimum allowed value is 5 seconds, and the
        /// maximum allowed value is 600 seconds. For Global external HTTP(S) load
        /// balancer (classic), this option is not available publicly.
        #[builder(into, default)]
        pub http_keep_alive_timeout_sec: pulumi_wasm_rust::Output<Option<i32>>,
        /// Name of the resource. Provided by the client when the resource is
        /// created. The name must be 1-63 characters long, and comply with
        /// RFC1035. Specifically, the name must be 1-63 characters long and match
        /// the regular expression `a-z?` which means the
        /// first character must be a lowercase letter, and all following
        /// characters must be a dash, lowercase letter, or digit, except the last
        /// character, which cannot be a dash.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// This field only applies when the forwarding rule that references
        /// this target proxy has a loadBalancingScheme set to INTERNAL_SELF_MANAGED.
        #[builder(into, default)]
        pub proxy_bind: pulumi_wasm_rust::Output<Option<bool>>,
        /// A reference to the UrlMap resource that defines the mapping from URL
        /// to the BackendService.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub url_map: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct TargetHttpProxyResult {
        /// Creation timestamp in RFC3339 text format.
        pub creation_timestamp: pulumi_wasm_rust::Output<String>,
        /// An optional description of this resource.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies how long to keep a connection open, after completing a response,
        /// while there is no matching traffic (in seconds). If an HTTP keepalive is
        /// not specified, a default value will be used. For Global
        /// external HTTP(S) load balancer, the default value is 610 seconds, the
        /// minimum allowed value is 5 seconds and the maximum allowed value is 1200
        /// seconds. For cross-region internal HTTP(S) load balancer, the default
        /// value is 600 seconds, the minimum allowed value is 5 seconds, and the
        /// maximum allowed value is 600 seconds. For Global external HTTP(S) load
        /// balancer (classic), this option is not available publicly.
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
        /// This field only applies when the forwarding rule that references
        /// this target proxy has a loadBalancingScheme set to INTERNAL_SELF_MANAGED.
        pub proxy_bind: pulumi_wasm_rust::Output<bool>,
        /// The unique identifier for the resource.
        pub proxy_id: pulumi_wasm_rust::Output<i32>,
        /// The URI of the created resource.
        pub self_link: pulumi_wasm_rust::Output<String>,
        /// A reference to the UrlMap resource that defines the mapping from URL
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
    pub fn create(name: &str, args: TargetHttpProxyArgs) -> TargetHttpProxyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_inner();
        let http_keep_alive_timeout_sec_binding = args
            .http_keep_alive_timeout_sec
            .get_inner();
        let name_binding = args.name.get_inner();
        let project_binding = args.project.get_inner();
        let proxy_bind_binding = args.proxy_bind.get_inner();
        let url_map_binding = args.url_map.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:compute/targetHttpProxy:TargetHttpProxy".into(),
            name: name.to_string(),
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
                    name: "proxyBind".into(),
                    value: &proxy_bind_binding,
                },
                register_interface::ObjectField {
                    name: "urlMap".into(),
                    value: &url_map_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "creationTimestamp".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "httpKeepAliveTimeoutSec".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "proxyBind".into(),
                },
                register_interface::ResultField {
                    name: "proxyId".into(),
                },
                register_interface::ResultField {
                    name: "selfLink".into(),
                },
                register_interface::ResultField {
                    name: "urlMap".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        TargetHttpProxyResult {
            creation_timestamp: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("creationTimestamp").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            http_keep_alive_timeout_sec: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("httpKeepAliveTimeoutSec").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            proxy_bind: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("proxyBind").unwrap(),
            ),
            proxy_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("proxyId").unwrap(),
            ),
            self_link: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("selfLink").unwrap(),
            ),
            url_map: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("urlMap").unwrap(),
            ),
        }
    }
}
