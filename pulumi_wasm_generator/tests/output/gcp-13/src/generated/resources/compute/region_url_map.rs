/// UrlMaps are used to route requests to a backend service based on rules
/// that you define for the host and path of an incoming URL.
///
///
///
/// ## Example Usage
///
/// ### Region Url Map Basic
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = region_health_check::create(
///         "default",
///         RegionHealthCheckArgs::builder()
///             .check_interval_sec(1)
///             .http_health_check(
///                 RegionHealthCheckHttpHealthCheck::builder()
///                     .port(80)
///                     .requestPath("/")
///                     .build_struct(),
///             )
///             .name("health-check")
///             .region("us-central1")
///             .timeout_sec(1)
///             .build_struct(),
///     );
///     let home = region_backend_service::create(
///         "home",
///         RegionBackendServiceArgs::builder()
///             .health_checks("${default.id}")
///             .load_balancing_scheme("INTERNAL_MANAGED")
///             .name("home")
///             .protocol("HTTP")
///             .region("us-central1")
///             .timeout_sec(10)
///             .build_struct(),
///     );
///     let login = region_backend_service::create(
///         "login",
///         RegionBackendServiceArgs::builder()
///             .health_checks("${default.id}")
///             .load_balancing_scheme("INTERNAL_MANAGED")
///             .name("login")
///             .protocol("HTTP")
///             .region("us-central1")
///             .timeout_sec(10)
///             .build_struct(),
///     );
///     let regionurlmap = region_url_map::create(
///         "regionurlmap",
///         RegionUrlMapArgs::builder()
///             .default_service("${home.id}")
///             .description("a description")
///             .host_rules(
///                 vec![
///                     RegionUrlMapHostRule::builder().hosts(vec!["mysite.com",])
///                     .pathMatcher("allpaths").build_struct(),
///                 ],
///             )
///             .name("regionurlmap")
///             .path_matchers(
///                 vec![
///                     RegionUrlMapPathMatcher::builder().defaultService("${home.id}")
///                     .name("allpaths")
///                     .pathRules(vec![RegionUrlMapPathMatcherPathRule::builder()
///                     .paths(vec!["/home",]).service("${home.id}").build_struct(),
///                     RegionUrlMapPathMatcherPathRule::builder().paths(vec!["/login",])
///                     .service("${login.id}").build_struct(),]).build_struct(),
///                 ],
///             )
///             .region("us-central1")
///             .tests(
///                 vec![
///                     RegionUrlMapTest::builder().host("hi.com").path("/home")
///                     .service("${home.id}").build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
/// ### Region Url Map Default Route Action
///
///
/// ```yaml
/// resources:
///   regionurlmap:
///     type: gcp:compute:RegionUrlMap
///     properties:
///       region: us-central1
///       name: regionurlmap
///       description: a description
///       defaultRouteAction:
///         retryPolicy:
///           retryConditions:
///             - 5xx
///             - gateway-error
///           numRetries: 3
///           perTryTimeout:
///             seconds: 0
///             nanos: 500
///         requestMirrorPolicy:
///           backendService: ${home.id}
///         weightedBackendServices:
///           - backendService: ${login.id}
///             weight: 200
///             headerAction:
///               requestHeadersToAdds:
///                 - headerName: foo-request-1
///                   headerValue: bar
///                   replace: true
///               requestHeadersToRemoves:
///                 - fizz
///               responseHeadersToAdds:
///                 - headerName: foo-response-1
///                   headerValue: bar
///                   replace: true
///               responseHeadersToRemoves:
///                 - buzz
///           - backendService: ${home.id}
///             weight: 100
///             headerAction:
///               requestHeadersToAdds:
///                 - headerName: foo-request-1
///                   headerValue: bar
///                   replace: true
///                 - headerName: foo-request-2
///                   headerValue: bar
///                   replace: true
///               requestHeadersToRemoves:
///                 - fizz
///               responseHeadersToAdds:
///                 - headerName: foo-response-2
///                   headerValue: bar
///                   replace: true
///                 - headerName: foo-response-1
///                   headerValue: bar
///                   replace: true
///               responseHeadersToRemoves:
///                 - buzz
///         urlRewrite:
///           hostRewrite: dev.example.com
///           pathPrefixRewrite: /v1/api/
///         corsPolicy:
///           disabled: false
///           allowCredentials: true
///           allowHeaders:
///             - foobar
///           allowMethods:
///             - GET
///             - POST
///           allowOrigins:
///             - example.com
///           exposeHeaders:
///             - foobar
///           maxAge: 60
///         faultInjectionPolicy:
///           delay:
///             fixedDelay:
///               seconds: 0
///               nanos: 500
///             percentage: 0.5
///           abort:
///             httpStatus: 500
///             percentage: 0.5
///         timeout:
///           seconds: 0
///           nanos: 500
///       hostRules:
///         - hosts:
///             - mysite.com
///           pathMatcher: allpaths
///       pathMatchers:
///         - name: allpaths
///           defaultService: ${home.id}
///           pathRules:
///             - paths:
///                 - /home
///               service: ${home.id}
///             - paths:
///                 - /login
///               service: ${login.id}
///       tests:
///         - service: ${home.id}
///           host: hi.com
///           path: /home
///   login:
///     type: gcp:compute:RegionBackendService
///     properties:
///       region: us-central1
///       name: login
///       protocol: HTTP
///       loadBalancingScheme: INTERNAL_MANAGED
///       timeoutSec: 10
///       healthChecks: ${default.id}
///   home:
///     type: gcp:compute:RegionBackendService
///     properties:
///       region: us-central1
///       name: home
///       protocol: HTTP
///       loadBalancingScheme: INTERNAL_MANAGED
///       timeoutSec: 10
///       healthChecks: ${default.id}
///   default:
///     type: gcp:compute:RegionHealthCheck
///     properties:
///       region: us-central1
///       name: health-check
///       checkIntervalSec: 1
///       timeoutSec: 1
///       httpHealthCheck:
///         port: 80
///         requestPath: /
/// ```
/// ### Region Url Map L7 Ilb Path
///
///
/// ```yaml
/// resources:
///   regionurlmap:
///     type: gcp:compute:RegionUrlMap
///     properties:
///       name: regionurlmap
///       description: a description
///       defaultService: ${home.id}
///       hostRules:
///         - hosts:
///             - mysite.com
///           pathMatcher: allpaths
///       pathMatchers:
///         - name: allpaths
///           defaultService: ${home.id}
///           pathRules:
///             - paths:
///                 - /home
///               routeAction:
///                 corsPolicy:
///                   allowCredentials: true
///                   allowHeaders:
///                     - Allowed content
///                   allowMethods:
///                     - GET
///                   allowOrigins:
///                     - Allowed origin
///                   exposeHeaders:
///                     - Exposed header
///                   maxAge: 30
///                   disabled: false
///                 faultInjectionPolicy:
///                   abort:
///                     httpStatus: 234
///                     percentage: 5.6
///                   delay:
///                     fixedDelay:
///                       seconds: 0
///                       nanos: 50000
///                     percentage: 7.8
///                 requestMirrorPolicy:
///                   backendService: ${home.id}
///                 retryPolicy:
///                   numRetries: 4
///                   perTryTimeout:
///                     seconds: 30
///                   retryConditions:
///                     - 5xx
///                     - deadline-exceeded
///                 timeout:
///                   seconds: 20
///                   nanos: 7.5e+08
///                 urlRewrite:
///                   hostRewrite: dev.example.com
///                   pathPrefixRewrite: /v1/api/
///                 weightedBackendServices:
///                   - backendService: ${home.id}
///                     weight: 400
///                     headerAction:
///                       requestHeadersToRemoves:
///                         - RemoveMe
///                       requestHeadersToAdds:
///                         - headerName: AddMe
///                           headerValue: MyValue
///                           replace: true
///                       responseHeadersToRemoves:
///                         - RemoveMe
///                       responseHeadersToAdds:
///                         - headerName: AddMe
///                           headerValue: MyValue
///                           replace: false
///       tests:
///         - service: ${home.id}
///           host: hi.com
///           path: /home
///   home:
///     type: gcp:compute:RegionBackendService
///     properties:
///       name: home
///       protocol: HTTP
///       timeoutSec: 10
///       healthChecks: ${default.id}
///       loadBalancingScheme: INTERNAL_MANAGED
///   default:
///     type: gcp:compute:RegionHealthCheck
///     properties:
///       name: health-check
///       httpHealthCheck:
///         port: 80
/// ```
/// ### Region Url Map L7 Ilb Path Partial
///
///
/// ```yaml
/// resources:
///   regionurlmap:
///     type: gcp:compute:RegionUrlMap
///     properties:
///       name: regionurlmap
///       description: a description
///       defaultService: ${home.id}
///       hostRules:
///         - hosts:
///             - mysite.com
///           pathMatcher: allpaths
///       pathMatchers:
///         - name: allpaths
///           defaultService: ${home.id}
///           pathRules:
///             - paths:
///                 - /home
///               routeAction:
///                 retryPolicy:
///                   numRetries: 4
///                   perTryTimeout:
///                     seconds: 30
///                   retryConditions:
///                     - 5xx
///                     - deadline-exceeded
///                 timeout:
///                   seconds: 20
///                   nanos: 7.5e+08
///                 urlRewrite:
///                   hostRewrite: dev.example.com
///                   pathPrefixRewrite: /v1/api/
///                 weightedBackendServices:
///                   - backendService: ${home.id}
///                     weight: 400
///                     headerAction:
///                       responseHeadersToAdds:
///                         - headerName: AddMe
///                           headerValue: MyValue
///                           replace: false
///       tests:
///         - service: ${home.id}
///           host: hi.com
///           path: /home
///   home:
///     type: gcp:compute:RegionBackendService
///     properties:
///       name: home
///       protocol: HTTP
///       timeoutSec: 10
///       healthChecks: ${default.id}
///       loadBalancingScheme: INTERNAL_MANAGED
///   default:
///     type: gcp:compute:RegionHealthCheck
///     properties:
///       name: health-check
///       httpHealthCheck:
///         port: 80
/// ```
/// ### Region Url Map L7 Ilb Route
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = region_health_check::create(
///         "default",
///         RegionHealthCheckArgs::builder()
///             .http_health_check(
///                 RegionHealthCheckHttpHealthCheck::builder().port(80).build_struct(),
///             )
///             .name("health-check")
///             .build_struct(),
///     );
///     let home = region_backend_service::create(
///         "home",
///         RegionBackendServiceArgs::builder()
///             .health_checks("${default.id}")
///             .load_balancing_scheme("INTERNAL_MANAGED")
///             .name("home")
///             .protocol("HTTP")
///             .timeout_sec(10)
///             .build_struct(),
///     );
///     let regionurlmap = region_url_map::create(
///         "regionurlmap",
///         RegionUrlMapArgs::builder()
///             .default_service("${home.id}")
///             .description("a description")
///             .host_rules(
///                 vec![
///                     RegionUrlMapHostRule::builder().hosts(vec!["mysite.com",])
///                     .pathMatcher("allpaths").build_struct(),
///                 ],
///             )
///             .name("regionurlmap")
///             .path_matchers(
///                 vec![
///                     RegionUrlMapPathMatcher::builder().defaultService("${home.id}")
///                     .name("allpaths")
///                     .routeRules(vec![RegionUrlMapPathMatcherRouteRule::builder()
///                     .headerAction(RegionUrlMapPathMatcherRouteRuleHeaderAction::builder()
///                     .requestHeadersToAdds(vec![RegionUrlMapPathMatcherRouteRuleHeaderActionRequestHeadersToAdd::builder()
///                     .headerName("AddSomethingElse").headerValue("MyOtherValue")
///                     .replace(true).build_struct(),])
///                     .requestHeadersToRemoves(vec!["RemoveMe2",])
///                     .responseHeadersToAdds(vec![RegionUrlMapPathMatcherRouteRuleHeaderActionResponseHeadersToAdd::builder()
///                     .headerName("AddMe").headerValue("MyValue").replace(false)
///                     .build_struct(),]).responseHeadersToRemoves(vec!["RemoveMe3",])
///                     .build_struct())
///                     .matchRules(vec![RegionUrlMapPathMatcherRouteRuleMatchRule::builder()
///                     .fullPathMatch("a full path")
///                     .headerMatches(vec![RegionUrlMapPathMatcherRouteRuleMatchRuleHeaderMatch::builder()
///                     .exactMatch("match this exactly").headerName("someheader")
///                     .invertMatch(true).build_struct(),]).ignoreCase(true)
///                     .metadataFilters(vec![RegionUrlMapPathMatcherRouteRuleMatchRuleMetadataFilter::builder()
///                     .filterLabels(vec![RegionUrlMapPathMatcherRouteRuleMatchRuleMetadataFilterFilterLabel::builder()
///                     .name("PLANET").value("MARS").build_struct(),])
///                     .filterMatchCriteria("MATCH_ANY").build_struct(),])
///                     .queryParameterMatches(vec![RegionUrlMapPathMatcherRouteRuleMatchRuleQueryParameterMatch::builder()
///                     .name("a query parameter").presentMatch(true).build_struct(),])
///                     .build_struct(),]).priority(1)
///                     .urlRedirect(RegionUrlMapPathMatcherRouteRuleUrlRedirect::builder()
///                     .hostRedirect("A host").httpsRedirect(false)
///                     .pathRedirect("some/path").redirectResponseCode("TEMPORARY_REDIRECT")
///                     .stripQuery(true).build_struct()).build_struct(),]).build_struct(),
///                 ],
///             )
///             .tests(
///                 vec![
///                     RegionUrlMapTest::builder().host("hi.com").path("/home")
///                     .service("${home.id}").build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
/// ### Region Url Map L7 Ilb Route Partial
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = region_health_check::create(
///         "default",
///         RegionHealthCheckArgs::builder()
///             .http_health_check(
///                 RegionHealthCheckHttpHealthCheck::builder().port(80).build_struct(),
///             )
///             .name("health-check")
///             .build_struct(),
///     );
///     let home = region_backend_service::create(
///         "home",
///         RegionBackendServiceArgs::builder()
///             .health_checks("${default.id}")
///             .load_balancing_scheme("INTERNAL_MANAGED")
///             .name("home")
///             .protocol("HTTP")
///             .timeout_sec(10)
///             .build_struct(),
///     );
///     let regionurlmap = region_url_map::create(
///         "regionurlmap",
///         RegionUrlMapArgs::builder()
///             .default_service("${home.id}")
///             .description("a description")
///             .host_rules(
///                 vec![
///                     RegionUrlMapHostRule::builder().hosts(vec!["mysite.com",])
///                     .pathMatcher("allpaths").build_struct(),
///                 ],
///             )
///             .name("regionurlmap")
///             .path_matchers(
///                 vec![
///                     RegionUrlMapPathMatcher::builder().defaultService("${home.id}")
///                     .name("allpaths")
///                     .routeRules(vec![RegionUrlMapPathMatcherRouteRule::builder()
///                     .headerAction(RegionUrlMapPathMatcherRouteRuleHeaderAction::builder()
///                     .requestHeadersToRemoves(vec!["RemoveMe2",]).build_struct())
///                     .matchRules(vec![RegionUrlMapPathMatcherRouteRuleMatchRule::builder()
///                     .fullPathMatch("a full path")
///                     .headerMatches(vec![RegionUrlMapPathMatcherRouteRuleMatchRuleHeaderMatch::builder()
///                     .exactMatch("match this exactly").headerName("someheader")
///                     .invertMatch(true).build_struct(),])
///                     .queryParameterMatches(vec![RegionUrlMapPathMatcherRouteRuleMatchRuleQueryParameterMatch::builder()
///                     .name("a query parameter").presentMatch(true).build_struct(),])
///                     .build_struct(),]).priority(1).service("${home.id}")
///                     .build_struct(),]).build_struct(),
///                 ],
///             )
///             .tests(
///                 vec![
///                     RegionUrlMapTest::builder().host("hi.com").path("/home")
///                     .service("${home.id}").build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
/// ### Region Url Map Path Template Match
///
///
/// ```yaml
/// resources:
///   urlmap:
///     type: gcp:compute:RegionUrlMap
///     properties:
///       region: us-central1
///       name: urlmap
///       description: a description
///       defaultService: ${["home-backend"].id}
///       hostRules:
///         - hosts:
///             - mysite.com
///           pathMatcher: mysite
///       pathMatchers:
///         - name: mysite
///           defaultService: ${["home-backend"].id}
///           routeRules:
///             - matchRules:
///                 - pathTemplateMatch: /xyzwebservices/v2/xyz/users/{username=*}/carts/{cartid=**}
///               service: ${["cart-backend"].id}
///               priority: 1
///               routeAction:
///                 urlRewrite:
///                   pathTemplateRewrite: /{username}-{cartid}/
///             - matchRules:
///                 - pathTemplateMatch: /xyzwebservices/v2/xyz/users/*/accountinfo/*
///               service: ${["user-backend"].id}
///               priority: 2
///   home-backend:
///     type: gcp:compute:RegionBackendService
///     properties:
///       region: us-central1
///       name: home-service
///       portName: http
///       protocol: HTTP
///       timeoutSec: 10
///       loadBalancingScheme: EXTERNAL_MANAGED
///       healthChecks: ${default.id}
///   cart-backend:
///     type: gcp:compute:RegionBackendService
///     properties:
///       region: us-central1
///       name: cart-service
///       portName: http
///       protocol: HTTP
///       timeoutSec: 10
///       loadBalancingScheme: EXTERNAL_MANAGED
///       healthChecks: ${default.id}
///   user-backend:
///     type: gcp:compute:RegionBackendService
///     properties:
///       region: us-central1
///       name: user-service
///       portName: http
///       protocol: HTTP
///       timeoutSec: 10
///       loadBalancingScheme: EXTERNAL_MANAGED
///       healthChecks: ${default.id}
///   default:
///     type: gcp:compute:RegionHealthCheck
///     properties:
///       region: us-central1
///       name: health-check
///       checkIntervalSec: 1
///       timeoutSec: 1
///       httpHealthCheck:
///         port: 80
///         requestPath: /
/// ```
///
/// ## Import
///
/// RegionUrlMap can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/regions/{{region}}/urlMaps/{{name}}`
///
/// * `{{project}}/{{region}}/{{name}}`
///
/// * `{{region}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, RegionUrlMap can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:compute/regionUrlMap:RegionUrlMap default projects/{{project}}/regions/{{region}}/urlMaps/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/regionUrlMap:RegionUrlMap default {{project}}/{{region}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/regionUrlMap:RegionUrlMap default {{region}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/regionUrlMap:RegionUrlMap default {{name}}
/// ```
///
pub mod region_url_map {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RegionUrlMapArgs {
        /// defaultRouteAction takes effect when none of the hostRules match. The load balancer performs advanced routing actions, such as URL rewrites and header transformations, before forwarding the request to the selected backend. If defaultRouteAction specifies any weightedBackendServices, defaultService must not be set. Conversely if defaultService is set, defaultRouteAction cannot contain any weightedBackendServices.
        /// Only one of defaultRouteAction or defaultUrlRedirect must be set.
        /// URL maps for Classic external HTTP(S) load balancers only support the urlRewrite action within defaultRouteAction.
        /// defaultRouteAction has no effect when the URL map is bound to a target gRPC proxy that has the validateForProxyless field set to true.
        /// Structure is documented below.
        #[builder(into, default)]
        pub default_route_action: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::compute::RegionUrlMapDefaultRouteAction>,
        >,
        /// The full or partial URL of the defaultService resource to which traffic is directed if
        /// none of the hostRules match. If defaultRouteAction is additionally specified, advanced
        /// routing actions like URL Rewrites, etc. take effect prior to sending the request to the
        /// backend. However, if defaultService is specified, defaultRouteAction cannot contain any
        /// weightedBackendServices. Conversely, if routeAction specifies any
        /// weightedBackendServices, service must not be specified.  Only one of defaultService,
        /// defaultUrlRedirect or defaultRouteAction.weightedBackendService must be set.
        #[builder(into, default)]
        pub default_service: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// When none of the specified hostRules match, the request is redirected to a URL specified
        /// by defaultUrlRedirect. If defaultUrlRedirect is specified, defaultService or
        /// defaultRouteAction must not be set.
        /// Structure is documented below.
        #[builder(into, default)]
        pub default_url_redirect: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::compute::RegionUrlMapDefaultUrlRedirect>,
        >,
        /// An optional description of this resource. Provide this property when
        /// you create the resource.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The list of HostRules to use against the URL.
        /// Structure is documented below.
        #[builder(into, default)]
        pub host_rules: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::compute::RegionUrlMapHostRule>>,
        >,
        /// Name of the resource. Provided by the client when the resource is
        /// created. The name must be 1-63 characters long, and comply with
        /// RFC1035. Specifically, the name must be 1-63 characters long and match
        /// the regular expression `a-z?` which means the
        /// first character must be a lowercase letter, and all following
        /// characters must be a dash, lowercase letter, or digit, except the last
        /// character, which cannot be a dash.
        ///
        ///
        /// - - -
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The list of named PathMatchers to use against the URL.
        /// Structure is documented below.
        #[builder(into, default)]
        pub path_matchers: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::compute::RegionUrlMapPathMatcher>>,
        >,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The Region in which the url map should reside.
        /// If it is not provided, the provider region is used.
        #[builder(into, default)]
        pub region: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The list of expected URL mappings. Requests to update this UrlMap will
        /// succeed only if all of the test cases pass.
        /// Structure is documented below.
        #[builder(into, default)]
        pub tests: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::compute::RegionUrlMapTest>>,
        >,
    }
    #[allow(dead_code)]
    pub struct RegionUrlMapResult {
        /// Creation timestamp in RFC3339 text format.
        pub creation_timestamp: pulumi_wasm_rust::Output<String>,
        /// defaultRouteAction takes effect when none of the hostRules match. The load balancer performs advanced routing actions, such as URL rewrites and header transformations, before forwarding the request to the selected backend. If defaultRouteAction specifies any weightedBackendServices, defaultService must not be set. Conversely if defaultService is set, defaultRouteAction cannot contain any weightedBackendServices.
        /// Only one of defaultRouteAction or defaultUrlRedirect must be set.
        /// URL maps for Classic external HTTP(S) load balancers only support the urlRewrite action within defaultRouteAction.
        /// defaultRouteAction has no effect when the URL map is bound to a target gRPC proxy that has the validateForProxyless field set to true.
        /// Structure is documented below.
        pub default_route_action: pulumi_wasm_rust::Output<
            Option<super::super::types::compute::RegionUrlMapDefaultRouteAction>,
        >,
        /// The full or partial URL of the defaultService resource to which traffic is directed if
        /// none of the hostRules match. If defaultRouteAction is additionally specified, advanced
        /// routing actions like URL Rewrites, etc. take effect prior to sending the request to the
        /// backend. However, if defaultService is specified, defaultRouteAction cannot contain any
        /// weightedBackendServices. Conversely, if routeAction specifies any
        /// weightedBackendServices, service must not be specified.  Only one of defaultService,
        /// defaultUrlRedirect or defaultRouteAction.weightedBackendService must be set.
        pub default_service: pulumi_wasm_rust::Output<Option<String>>,
        /// When none of the specified hostRules match, the request is redirected to a URL specified
        /// by defaultUrlRedirect. If defaultUrlRedirect is specified, defaultService or
        /// defaultRouteAction must not be set.
        /// Structure is documented below.
        pub default_url_redirect: pulumi_wasm_rust::Output<
            Option<super::super::types::compute::RegionUrlMapDefaultUrlRedirect>,
        >,
        /// An optional description of this resource. Provide this property when
        /// you create the resource.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Fingerprint of this resource. This field is used internally during
        /// updates of this resource.
        pub fingerprint: pulumi_wasm_rust::Output<String>,
        /// The list of HostRules to use against the URL.
        /// Structure is documented below.
        pub host_rules: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::compute::RegionUrlMapHostRule>>,
        >,
        /// The unique identifier for the resource.
        pub map_id: pulumi_wasm_rust::Output<i32>,
        /// Name of the resource. Provided by the client when the resource is
        /// created. The name must be 1-63 characters long, and comply with
        /// RFC1035. Specifically, the name must be 1-63 characters long and match
        /// the regular expression `a-z?` which means the
        /// first character must be a lowercase letter, and all following
        /// characters must be a dash, lowercase letter, or digit, except the last
        /// character, which cannot be a dash.
        ///
        ///
        /// - - -
        pub name: pulumi_wasm_rust::Output<String>,
        /// The list of named PathMatchers to use against the URL.
        /// Structure is documented below.
        pub path_matchers: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::compute::RegionUrlMapPathMatcher>>,
        >,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// The Region in which the url map should reside.
        /// If it is not provided, the provider region is used.
        pub region: pulumi_wasm_rust::Output<String>,
        /// The URI of the created resource.
        pub self_link: pulumi_wasm_rust::Output<String>,
        /// The list of expected URL mappings. Requests to update this UrlMap will
        /// succeed only if all of the test cases pass.
        /// Structure is documented below.
        pub tests: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::compute::RegionUrlMapTest>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: RegionUrlMapArgs,
    ) -> RegionUrlMapResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let default_route_action_binding = args
            .default_route_action
            .get_output(context)
            .get_inner();
        let default_service_binding = args
            .default_service
            .get_output(context)
            .get_inner();
        let default_url_redirect_binding = args
            .default_url_redirect
            .get_output(context)
            .get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let host_rules_binding = args.host_rules.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let path_matchers_binding = args.path_matchers.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let region_binding = args.region.get_output(context).get_inner();
        let tests_binding = args.tests.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:compute/regionUrlMap:RegionUrlMap".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "defaultRouteAction".into(),
                    value: &default_route_action_binding,
                },
                register_interface::ObjectField {
                    name: "defaultService".into(),
                    value: &default_service_binding,
                },
                register_interface::ObjectField {
                    name: "defaultUrlRedirect".into(),
                    value: &default_url_redirect_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "hostRules".into(),
                    value: &host_rules_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "pathMatchers".into(),
                    value: &path_matchers_binding,
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
                    name: "tests".into(),
                    value: &tests_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "creationTimestamp".into(),
                },
                register_interface::ResultField {
                    name: "defaultRouteAction".into(),
                },
                register_interface::ResultField {
                    name: "defaultService".into(),
                },
                register_interface::ResultField {
                    name: "defaultUrlRedirect".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "fingerprint".into(),
                },
                register_interface::ResultField {
                    name: "hostRules".into(),
                },
                register_interface::ResultField {
                    name: "mapId".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "pathMatchers".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "region".into(),
                },
                register_interface::ResultField {
                    name: "selfLink".into(),
                },
                register_interface::ResultField {
                    name: "tests".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        RegionUrlMapResult {
            creation_timestamp: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("creationTimestamp").unwrap(),
            ),
            default_route_action: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultRouteAction").unwrap(),
            ),
            default_service: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultService").unwrap(),
            ),
            default_url_redirect: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultUrlRedirect").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            fingerprint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("fingerprint").unwrap(),
            ),
            host_rules: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hostRules").unwrap(),
            ),
            map_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("mapId").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            path_matchers: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pathMatchers").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            region: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("region").unwrap(),
            ),
            self_link: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("selfLink").unwrap(),
            ),
            tests: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tests").unwrap(),
            ),
        }
    }
}
