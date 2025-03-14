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
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod target_grpc_proxy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TargetGrpcProxyArgs {
        /// An optional description of this resource.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
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
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// URL to the UrlMap resource that defines the mapping from URL to
        /// the BackendService. The protocol field in the BackendService
        /// must be set to GRPC.
        #[builder(into, default)]
        pub url_map: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
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
        pub validate_for_proxyless: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct TargetGrpcProxyResult {
        /// Creation timestamp in RFC3339 text format.
        pub creation_timestamp: pulumi_gestalt_rust::Output<String>,
        /// An optional description of this resource.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Fingerprint of this resource. A hash of the contents stored in
        /// this object. This field is used in optimistic locking. This field
        /// will be ignored when inserting a TargetGrpcProxy. An up-to-date
        /// fingerprint must be provided in order to patch/update the
        /// TargetGrpcProxy; otherwise, the request will fail with error
        /// 412 conditionNotMet. To see the latest fingerprint, make a get()
        /// request to retrieve the TargetGrpcProxy. A base64-encoded string.
        pub fingerprint: pulumi_gestalt_rust::Output<String>,
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
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The URI of the created resource.
        pub self_link: pulumi_gestalt_rust::Output<String>,
        /// Server-defined URL with id for the resource.
        pub self_link_with_id: pulumi_gestalt_rust::Output<String>,
        /// URL to the UrlMap resource that defines the mapping from URL to
        /// the BackendService. The protocol field in the BackendService
        /// must be set to GRPC.
        pub url_map: pulumi_gestalt_rust::Output<Option<String>>,
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
        pub validate_for_proxyless: pulumi_gestalt_rust::Output<Option<bool>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: TargetGrpcProxyArgs,
    ) -> TargetGrpcProxyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let name_binding = args.name.get_output(context);
        let project_binding = args.project.get_output(context);
        let url_map_binding = args.url_map.get_output(context);
        let validate_for_proxyless_binding = args
            .validate_for_proxyless
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:compute/targetGrpcProxy:TargetGrpcProxy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "urlMap".into(),
                    value: &url_map_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "validateForProxyless".into(),
                    value: &validate_for_proxyless_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        TargetGrpcProxyResult {
            creation_timestamp: o.get_field("creationTimestamp"),
            description: o.get_field("description"),
            fingerprint: o.get_field("fingerprint"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            self_link: o.get_field("selfLink"),
            self_link_with_id: o.get_field("selfLinkWithId"),
            url_map: o.get_field("urlMap"),
            validate_for_proxyless: o.get_field("validateForProxyless"),
        }
    }
}
