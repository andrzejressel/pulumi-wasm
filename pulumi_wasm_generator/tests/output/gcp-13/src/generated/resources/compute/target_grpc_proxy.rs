/// Represents a Target gRPC Proxy resource. A target gRPC proxy is a component
/// of load balancers intended for load balancing gRPC traffic. Global forwarding
/// rules reference a target gRPC proxy. The Target gRPC Proxy references
/// a URL map which specifies how traffic routes to gRPC backend services.
///
///
/// To get more information about TargetGrpcProxy, see:
///
/// * [API documentation](https://cloud.google.com/compute/docs/reference/rest/v1/targetGrpcProxies)
/// * How-to Guides
///     * [Using Target gRPC Proxies](https://cloud.google.com/traffic-director/docs/proxyless-overview)
///
/// ## Example Usage
///
/// ### Target Grpc Proxy Basic
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = target_grpc_proxy::create(
///         "default",
///         TargetGrpcProxyArgs::builder()
///             .name("proxy")
///             .url_map("${urlmap.id}")
///             .validate_for_proxyless(true)
///             .build_struct(),
///     );
///     let defaultHealthCheck = health_check::create(
///         "defaultHealthCheck",
///         HealthCheckArgs::builder()
///             .check_interval_sec(1)
///             .grpc_health_check(
///                 HealthCheckGrpcHealthCheck::builder()
///                     .grpcServiceName("testservice")
///                     .portName("health-check-port")
///                     .portSpecification("USE_NAMED_PORT")
///                     .build_struct(),
///             )
///             .name("healthcheck")
///             .timeout_sec(1)
///             .build_struct(),
///     );
///     let home = backend_service::create(
///         "home",
///         BackendServiceArgs::builder()
///             .health_checks("${defaultHealthCheck.id}")
///             .load_balancing_scheme("INTERNAL_SELF_MANAGED")
///             .name("backend")
///             .port_name("grpc")
///             .protocol("GRPC")
///             .timeout_sec(10)
///             .build_struct(),
///     );
///     let urlmap = url_map::create(
///         "urlmap",
///         UrlMapArgs::builder()
///             .default_service("${home.id}")
///             .description("a description")
///             .host_rules(
///                 vec![
///                     UrlMapHostRule::builder().hosts(vec!["mysite.com",])
///                     .pathMatcher("allpaths").build_struct(),
///                 ],
///             )
///             .name("urlmap")
///             .path_matchers(
///                 vec![
///                     UrlMapPathMatcher::builder().defaultService("${home.id}")
///                     .name("allpaths")
///                     .routeRules(vec![UrlMapPathMatcherRouteRule::builder()
///                     .headerAction(UrlMapPathMatcherRouteRuleHeaderAction::builder()
///                     .requestHeadersToAdds(vec![UrlMapPathMatcherRouteRuleHeaderActionRequestHeadersToAdd::builder()
///                     .headerName("AddSomethingElse").headerValue("MyOtherValue")
///                     .replace(true).build_struct(),])
///                     .requestHeadersToRemoves(vec!["RemoveMe2",])
///                     .responseHeadersToAdds(vec![UrlMapPathMatcherRouteRuleHeaderActionResponseHeadersToAdd::builder()
///                     .headerName("AddMe").headerValue("MyValue").replace(false)
///                     .build_struct(),]).responseHeadersToRemoves(vec!["RemoveMe3",])
///                     .build_struct())
///                     .matchRules(vec![UrlMapPathMatcherRouteRuleMatchRule::builder()
///                     .fullPathMatch("a full path")
///                     .headerMatches(vec![UrlMapPathMatcherRouteRuleMatchRuleHeaderMatch::builder()
///                     .exactMatch("match this exactly").headerName("someheader")
///                     .invertMatch(true).build_struct(),]).ignoreCase(true)
///                     .metadataFilters(vec![UrlMapPathMatcherRouteRuleMatchRuleMetadataFilter::builder()
///                     .filterLabels(vec![UrlMapPathMatcherRouteRuleMatchRuleMetadataFilterFilterLabel::builder()
///                     .name("PLANET").value("MARS").build_struct(),])
///                     .filterMatchCriteria("MATCH_ANY").build_struct(),])
///                     .queryParameterMatches(vec![UrlMapPathMatcherRouteRuleMatchRuleQueryParameterMatch::builder()
///                     .name("a query parameter").presentMatch(true).build_struct(),])
///                     .build_struct(),]).priority(1)
///                     .urlRedirect(UrlMapPathMatcherRouteRuleUrlRedirect::builder()
///                     .hostRedirect("A host").httpsRedirect(false)
///                     .pathRedirect("some/path").redirectResponseCode("TEMPORARY_REDIRECT")
///                     .stripQuery(true).build_struct()).build_struct(),]).build_struct(),
///                 ],
///             )
///             .tests(
///                 vec![
///                     UrlMapTest::builder().host("hi.com").path("/home")
///                     .service("${home.id}").build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// TargetGrpcProxy can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/global/targetGrpcProxies/{{name}}`
///
/// * `{{project}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, TargetGrpcProxy can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:compute/targetGrpcProxy:TargetGrpcProxy default projects/{{project}}/global/targetGrpcProxies/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/targetGrpcProxy:TargetGrpcProxy default {{project}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/targetGrpcProxy:TargetGrpcProxy default {{name}}
/// ```
///
pub mod target_grpc_proxy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TargetGrpcProxyArgs {
        /// An optional description of this resource.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Name of the resource. Provided by the client when the resource
        /// is created. The name must be 1-63 characters long, and comply
        /// with RFC1035. Specifically, the name must be 1-63 characters long
        /// and match the regular expression `a-z?` which
        /// means the first character must be a lowercase letter, and all
        /// following characters must be a dash, lowercase letter, or digit,
        /// except the last character, which cannot be a dash.
        ///
        ///
        /// - - -
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// URL to the UrlMap resource that defines the mapping from URL to
        /// the BackendService. The protocol field in the BackendService
        /// must be set to GRPC.
        #[builder(into, default)]
        pub url_map: pulumi_wasm_rust::Output<Option<String>>,
        /// If true, indicates that the BackendServices referenced by
        /// the urlMap may be accessed by gRPC applications without using
        /// a sidecar proxy. This will enable configuration checks on urlMap
        /// and its referenced BackendServices to not allow unsupported features.
        /// A gRPC application must use "xds:///" scheme in the target URI
        /// of the service it is connecting to. If false, indicates that the
        /// BackendServices referenced by the urlMap will be accessed by gRPC
        /// applications via a sidecar proxy. In this case, a gRPC application
        /// must not use "xds:///" scheme in the target URI of the service
        /// it is connecting to
        #[builder(into, default)]
        pub validate_for_proxyless: pulumi_wasm_rust::Output<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct TargetGrpcProxyResult {
        /// Creation timestamp in RFC3339 text format.
        pub creation_timestamp: pulumi_wasm_rust::Output<String>,
        /// An optional description of this resource.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Fingerprint of this resource. A hash of the contents stored in
        /// this object. This field is used in optimistic locking. This field
        /// will be ignored when inserting a TargetGrpcProxy. An up-to-date
        /// fingerprint must be provided in order to patch/update the
        /// TargetGrpcProxy; otherwise, the request will fail with error
        /// 412 conditionNotMet. To see the latest fingerprint, make a get()
        /// request to retrieve the TargetGrpcProxy. A base64-encoded string.
        pub fingerprint: pulumi_wasm_rust::Output<String>,
        /// Name of the resource. Provided by the client when the resource
        /// is created. The name must be 1-63 characters long, and comply
        /// with RFC1035. Specifically, the name must be 1-63 characters long
        /// and match the regular expression `a-z?` which
        /// means the first character must be a lowercase letter, and all
        /// following characters must be a dash, lowercase letter, or digit,
        /// except the last character, which cannot be a dash.
        ///
        ///
        /// - - -
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// The URI of the created resource.
        pub self_link: pulumi_wasm_rust::Output<String>,
        /// Server-defined URL with id for the resource.
        pub self_link_with_id: pulumi_wasm_rust::Output<String>,
        /// URL to the UrlMap resource that defines the mapping from URL to
        /// the BackendService. The protocol field in the BackendService
        /// must be set to GRPC.
        pub url_map: pulumi_wasm_rust::Output<Option<String>>,
        /// If true, indicates that the BackendServices referenced by
        /// the urlMap may be accessed by gRPC applications without using
        /// a sidecar proxy. This will enable configuration checks on urlMap
        /// and its referenced BackendServices to not allow unsupported features.
        /// A gRPC application must use "xds:///" scheme in the target URI
        /// of the service it is connecting to. If false, indicates that the
        /// BackendServices referenced by the urlMap will be accessed by gRPC
        /// applications via a sidecar proxy. In this case, a gRPC application
        /// must not use "xds:///" scheme in the target URI of the service
        /// it is connecting to
        pub validate_for_proxyless: pulumi_wasm_rust::Output<Option<bool>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: TargetGrpcProxyArgs) -> TargetGrpcProxyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_inner();
        let name_binding = args.name.get_inner();
        let project_binding = args.project.get_inner();
        let url_map_binding = args.url_map.get_inner();
        let validate_for_proxyless_binding = args.validate_for_proxyless.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:compute/targetGrpcProxy:TargetGrpcProxy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
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
                    name: "urlMap".into(),
                    value: &url_map_binding,
                },
                register_interface::ObjectField {
                    name: "validateForProxyless".into(),
                    value: &validate_for_proxyless_binding,
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
                    name: "fingerprint".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "selfLink".into(),
                },
                register_interface::ResultField {
                    name: "selfLinkWithId".into(),
                },
                register_interface::ResultField {
                    name: "urlMap".into(),
                },
                register_interface::ResultField {
                    name: "validateForProxyless".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        TargetGrpcProxyResult {
            creation_timestamp: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("creationTimestamp").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            fingerprint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("fingerprint").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            self_link: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("selfLink").unwrap(),
            ),
            self_link_with_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("selfLinkWithId").unwrap(),
            ),
            url_map: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("urlMap").unwrap(),
            ),
            validate_for_proxyless: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("validateForProxyless").unwrap(),
            ),
        }
    }
}
