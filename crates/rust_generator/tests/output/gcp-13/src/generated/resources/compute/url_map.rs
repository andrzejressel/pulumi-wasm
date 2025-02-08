/// UrlMaps are used to route requests to a backend service based on rules
/// that you define for the host and path of an incoming URL.
///
///
/// To get more information about UrlMap, see:
///
/// * [API documentation](https://cloud.google.com/compute/docs/reference/rest/v1/urlMaps)
///
/// ## Example Usage
///
/// ### Url Map Bucket And Service
///
///
/// ```yaml
/// resources:
///   urlmap:
///     type: gcp:compute:URLMap
///     properties:
///       name: urlmap
///       description: a description
///       defaultService: ${static.id}
///       hostRules:
///         - hosts:
///             - mysite.com
///           pathMatcher: mysite
///         - hosts:
///             - myothersite.com
///           pathMatcher: otherpaths
///       pathMatchers:
///         - name: mysite
///           defaultService: ${static.id}
///           pathRules:
///             - paths:
///                 - /home
///               service: ${static.id}
///             - paths:
///                 - /login
///               service: ${login.id}
///             - paths:
///                 - /static
///               service: ${static.id}
///         - name: otherpaths
///           defaultService: ${static.id}
///       tests:
///         - service: ${static.id}
///           host: example.com
///           path: /home
///   login:
///     type: gcp:compute:BackendService
///     properties:
///       name: login
///       portName: http
///       protocol: HTTP
///       timeoutSec: 10
///       healthChecks: ${default.id}
///   default:
///     type: gcp:compute:HttpHealthCheck
///     properties:
///       name: health-check
///       requestPath: /
///       checkIntervalSec: 1
///       timeoutSec: 1
///   static:
///     type: gcp:compute:BackendBucket
///     properties:
///       name: static-asset-backend-bucket
///       bucketName: ${staticBucket.name}
///       enableCdn: true
///   staticBucket:
///     type: gcp:storage:Bucket
///     name: static
///     properties:
///       name: static-asset-bucket
///       location: US
/// ```
/// ### Url Map Traffic Director Route
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = health_check::create(
///         "default",
///         HealthCheckArgs::builder()
///             .http_health_check(
///                 HealthCheckHttpHealthCheck::builder().port(80).build_struct(),
///             )
///             .name("health-check")
///             .build_struct(),
///     );
///     let home = backend_service::create(
///         "home",
///         BackendServiceArgs::builder()
///             .health_checks("${default.id}")
///             .load_balancing_scheme("INTERNAL_SELF_MANAGED")
///             .name("home")
///             .port_name("http")
///             .protocol("HTTP")
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
/// ### Url Map Traffic Director Route Partial
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = health_check::create(
///         "default",
///         HealthCheckArgs::builder()
///             .http_health_check(
///                 HealthCheckHttpHealthCheck::builder().port(80).build_struct(),
///             )
///             .name("health-check")
///             .build_struct(),
///     );
///     let home = backend_service::create(
///         "home",
///         BackendServiceArgs::builder()
///             .health_checks("${default.id}")
///             .load_balancing_scheme("INTERNAL_SELF_MANAGED")
///             .name("home")
///             .port_name("http")
///             .protocol("HTTP")
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
///                     .matchRules(vec![UrlMapPathMatcherRouteRuleMatchRule::builder()
///                     .headerMatches(vec![UrlMapPathMatcherRouteRuleMatchRuleHeaderMatch::builder()
///                     .exactMatch("match this exactly").headerName("someheader")
///                     .invertMatch(true).build_struct(),]).prefixMatch("/someprefix")
///                     .build_struct(),]).priority(1)
///                     .urlRedirect(UrlMapPathMatcherRouteRuleUrlRedirect::builder()
///                     .pathRedirect("some/path").redirectResponseCode("TEMPORARY_REDIRECT")
///                     .build_struct()).build_struct(),]).build_struct(),
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
/// ### Url Map Traffic Director Path
///
///
/// ```yaml
/// resources:
///   urlmap:
///     type: gcp:compute:URLMap
///     properties:
///       name: urlmap
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
///                   allowOriginRegexes:
///                     - abc.*
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
///     type: gcp:compute:BackendService
///     properties:
///       name: home
///       portName: http
///       protocol: HTTP
///       timeoutSec: 10
///       healthChecks: ${default.id}
///       loadBalancingScheme: INTERNAL_SELF_MANAGED
///   default:
///     type: gcp:compute:HealthCheck
///     properties:
///       name: health-check
///       httpHealthCheck:
///         port: 80
/// ```
/// ### Url Map Traffic Director Path Partial
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = health_check::create(
///         "default",
///         HealthCheckArgs::builder()
///             .http_health_check(
///                 HealthCheckHttpHealthCheck::builder().port(80).build_struct(),
///             )
///             .name("health-check")
///             .build_struct(),
///     );
///     let home = backend_service::create(
///         "home",
///         BackendServiceArgs::builder()
///             .health_checks("${default.id}")
///             .load_balancing_scheme("INTERNAL_SELF_MANAGED")
///             .name("home")
///             .port_name("http")
///             .protocol("HTTP")
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
///                     .name("allpaths").pathRules(vec![UrlMapPathMatcherPathRule::builder()
///                     .paths(vec!["/home",])
///                     .routeAction(UrlMapPathMatcherPathRuleRouteAction::builder()
///                     .corsPolicy(UrlMapPathMatcherPathRuleRouteActionCorsPolicy::builder()
///                     .allowCredentials(true).allowHeaders(vec!["Allowed content",])
///                     .allowMethods(vec!["GET",]).allowOriginRegexes(vec!["abc.*",])
///                     .allowOrigins(vec!["Allowed origin",]).disabled(false)
///                     .exposeHeaders(vec!["Exposed header",]).maxAge(30).build_struct())
///                     .weightedBackendServices(vec![UrlMapPathMatcherPathRuleRouteActionWeightedBackendService::builder()
///                     .backendService("${home.id}")
///                     .headerAction(UrlMapPathMatcherPathRuleRouteActionWeightedBackendServiceHeaderAction::builder()
///                     .requestHeadersToAdds(vec![UrlMapPathMatcherPathRuleRouteActionWeightedBackendServiceHeaderActionRequestHeadersToAdd::builder()
///                     .headerName("AddMe").headerValue("MyValue").replace(true)
///                     .build_struct(),]).requestHeadersToRemoves(vec!["RemoveMe",])
///                     .responseHeadersToAdds(vec![UrlMapPathMatcherPathRuleRouteActionWeightedBackendServiceHeaderActionResponseHeadersToAdd::builder()
///                     .headerName("AddMe").headerValue("MyValue").replace(false)
///                     .build_struct(),]).responseHeadersToRemoves(vec!["RemoveMe",])
///                     .build_struct()).weight(400).build_struct(),]).build_struct())
///                     .build_struct(),]).build_struct(),
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
/// ### Url Map Header Based Routing
///
///
/// ```yaml
/// resources:
///   urlmap:
///     type: gcp:compute:URLMap
///     properties:
///       name: urlmap
///       description: header-based routing example
///       defaultService: ${default.id}
///       hostRules:
///         - hosts:
///             - '*'
///           pathMatcher: allpaths
///       pathMatchers:
///         - name: allpaths
///           defaultService: ${default.id}
///           routeRules:
///             - priority: 1
///               service: ${["service-a"].id}
///               matchRules:
///                 - prefixMatch: /
///                   ignoreCase: true
///                   headerMatches:
///                     - headerName: abtest
///                       exactMatch: a
///             - priority: 2
///               service: ${["service-b"].id}
///               matchRules:
///                 - ignoreCase: true
///                   prefixMatch: /
///                   headerMatches:
///                     - headerName: abtest
///                       exactMatch: b
///   default:
///     type: gcp:compute:BackendService
///     properties:
///       name: default
///       portName: http
///       protocol: HTTP
///       timeoutSec: 10
///       healthChecks: ${defaultHttpHealthCheck.id}
///   service-a:
///     type: gcp:compute:BackendService
///     properties:
///       name: service-a
///       portName: http
///       protocol: HTTP
///       timeoutSec: 10
///       healthChecks: ${defaultHttpHealthCheck.id}
///   service-b:
///     type: gcp:compute:BackendService
///     properties:
///       name: service-b
///       portName: http
///       protocol: HTTP
///       timeoutSec: 10
///       healthChecks: ${defaultHttpHealthCheck.id}
///   defaultHttpHealthCheck:
///     type: gcp:compute:HttpHealthCheck
///     name: default
///     properties:
///       name: health-check
///       requestPath: /
///       checkIntervalSec: 1
///       timeoutSec: 1
/// ```
/// ### Url Map Parameter Based Routing
///
///
/// ```yaml
/// resources:
///   urlmap:
///     type: gcp:compute:URLMap
///     properties:
///       name: urlmap
///       description: parameter-based routing example
///       defaultService: ${default.id}
///       hostRules:
///         - hosts:
///             - '*'
///           pathMatcher: allpaths
///       pathMatchers:
///         - name: allpaths
///           defaultService: ${default.id}
///           routeRules:
///             - priority: 1
///               service: ${["service-a"].id}
///               matchRules:
///                 - prefixMatch: /
///                   ignoreCase: true
///                   queryParameterMatches:
///                     - name: abtest
///                       exactMatch: a
///             - priority: 2
///               service: ${["service-b"].id}
///               matchRules:
///                 - ignoreCase: true
///                   prefixMatch: /
///                   queryParameterMatches:
///                     - name: abtest
///                       exactMatch: b
///   default:
///     type: gcp:compute:BackendService
///     properties:
///       name: default
///       portName: http
///       protocol: HTTP
///       timeoutSec: 10
///       healthChecks: ${defaultHttpHealthCheck.id}
///   service-a:
///     type: gcp:compute:BackendService
///     properties:
///       name: service-a
///       portName: http
///       protocol: HTTP
///       timeoutSec: 10
///       healthChecks: ${defaultHttpHealthCheck.id}
///   service-b:
///     type: gcp:compute:BackendService
///     properties:
///       name: service-b
///       portName: http
///       protocol: HTTP
///       timeoutSec: 10
///       healthChecks: ${defaultHttpHealthCheck.id}
///   defaultHttpHealthCheck:
///     type: gcp:compute:HttpHealthCheck
///     name: default
///     properties:
///       name: health-check
///       requestPath: /
///       checkIntervalSec: 1
///       timeoutSec: 1
/// ```
/// ### Url Map Path Template Match
///
///
/// ```yaml
/// resources:
///   urlmap:
///     type: gcp:compute:URLMap
///     properties:
///       name: urlmap
///       description: a description
///       defaultService: ${static.id}
///       hostRules:
///         - hosts:
///             - mysite.com
///           pathMatcher: mysite
///       pathMatchers:
///         - name: mysite
///           defaultService: ${static.id}
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
///   cart-backend:
///     type: gcp:compute:BackendService
///     properties:
///       name: cart-service
///       portName: http
///       protocol: HTTP
///       timeoutSec: 10
///       loadBalancingScheme: EXTERNAL_MANAGED
///       healthChecks: ${default.id}
///   user-backend:
///     type: gcp:compute:BackendService
///     properties:
///       name: user-service
///       portName: http
///       protocol: HTTP
///       timeoutSec: 10
///       loadBalancingScheme: EXTERNAL_MANAGED
///       healthChecks: ${default.id}
///   default:
///     type: gcp:compute:HttpHealthCheck
///     properties:
///       name: health-check
///       requestPath: /
///       checkIntervalSec: 1
///       timeoutSec: 1
///   static:
///     type: gcp:compute:BackendBucket
///     properties:
///       name: static-asset-backend-bucket
///       bucketName: ${staticBucket.name}
///       enableCdn: true
///   staticBucket:
///     type: gcp:storage:Bucket
///     name: static
///     properties:
///       name: static-asset-bucket
///       location: US
/// ```
/// ### Url Map Custom Error Response Policy
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = http_health_check::create(
///         "default",
///         HttpHealthCheckArgs::builder()
///             .check_interval_sec(1)
///             .name("health-check")
///             .request_path("/")
///             .timeout_sec(1)
///             .build_struct(),
///     );
///     let error = backend_bucket::create(
///         "error",
///         BackendBucketArgs::builder()
///             .bucket_name("${errorBucket.name}")
///             .enable_cdn(true)
///             .name("error-backend-bucket")
///             .build_struct(),
///     );
///     let errorBucket = bucket::create(
///         "errorBucket",
///         BucketArgs::builder().location("US").name("static-asset-bucket").build_struct(),
///     );
///     let example = backend_service::create(
///         "example",
///         BackendServiceArgs::builder()
///             .health_checks("${default.id}")
///             .load_balancing_scheme("EXTERNAL_MANAGED")
///             .name("login")
///             .port_name("http")
///             .protocol("HTTP")
///             .timeout_sec(10)
///             .build_struct(),
///     );
///     let urlmap = url_map::create(
///         "urlmap",
///         UrlMapArgs::builder()
///             .default_custom_error_response_policy(
///                 UrlMapDefaultCustomErrorResponsePolicy::builder()
///                     .errorResponseRules(
///                         vec![
///                             UrlMapDefaultCustomErrorResponsePolicyErrorResponseRule::builder()
///                             .matchResponseCodes(vec!["5xx",]).overrideResponseCode(502)
///                             .path("/internal_error.html").build_struct(),
///                         ],
///                     )
///                     .errorService("${error.id}")
///                     .build_struct(),
///             )
///             .default_service("${example.id}")
///             .description("a description")
///             .host_rules(
///                 vec![
///                     UrlMapHostRule::builder().hosts(vec!["mysite.com",])
///                     .pathMatcher("mysite").build_struct(),
///                 ],
///             )
///             .name("urlmap")
///             .path_matchers(
///                 vec![
///                     UrlMapPathMatcher::builder()
///                     .defaultCustomErrorResponsePolicy(UrlMapPathMatcherDefaultCustomErrorResponsePolicy::builder()
///                     .errorResponseRules(vec![UrlMapPathMatcherDefaultCustomErrorResponsePolicyErrorResponseRule::builder()
///                     .matchResponseCodes(vec!["4xx", "5xx",]).overrideResponseCode(404)
///                     .path("/login_error.html").build_struct(),
///                     UrlMapPathMatcherDefaultCustomErrorResponsePolicyErrorResponseRule::builder()
///                     .matchResponseCodes(vec!["503",]).overrideResponseCode(502)
///                     .path("/bad_gateway.html").build_struct(),])
///                     .errorService("${error.id}").build_struct())
///                     .defaultService("${example.id}").name("mysite")
///                     .pathRules(vec![UrlMapPathMatcherPathRule::builder()
///                     .customErrorResponsePolicy(UrlMapPathMatcherPathRuleCustomErrorResponsePolicy::builder()
///                     .errorResponseRules(vec![UrlMapPathMatcherPathRuleCustomErrorResponsePolicyErrorResponseRule::builder()
///                     .matchResponseCodes(vec!["4xx",]).overrideResponseCode(401)
///                     .path("/login.html").build_struct(),]).errorService("${error.id}")
///                     .build_struct()).paths(vec!["/private/*",]).service("${example.id}")
///                     .build_struct(),]).build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// UrlMap can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/global/urlMaps/{{name}}`
///
/// * `{{project}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, UrlMap can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:compute/uRLMap:URLMap default projects/{{project}}/global/urlMaps/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/uRLMap:URLMap default {{project}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/uRLMap:URLMap default {{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod url_map {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct URLMapArgs {
        /// defaultCustomErrorResponsePolicy specifies how the Load Balancer returns error responses when BackendService or BackendBucket responds with an error.
        /// This policy takes effect at the PathMatcher level and applies only when no policy has been defined for the error code at lower levels like RouteRule and PathRule within this PathMatcher. If an error code does not have a policy defined in defaultCustomErrorResponsePolicy, then a policy defined for the error code in UrlMap.defaultCustomErrorResponsePolicy takes effect.
        /// For example, consider a UrlMap with the following configuration:
        /// UrlMap.defaultCustomErrorResponsePolicy is configured with policies for 5xx and 4xx errors
        /// A RouteRule for /coming_soon/ is configured for the error code 404.
        /// If the request is for www.myotherdomain.com and a 404 is encountered, the policy under UrlMap.defaultCustomErrorResponsePolicy takes effect. If a 404 response is encountered for the request www.example.com/current_events/, the pathMatcher's policy takes effect. If however, the request for www.example.com/coming_soon/ encounters a 404, the policy in RouteRule.customErrorResponsePolicy takes effect. If any of the requests in this example encounter a 500 error code, the policy at UrlMap.defaultCustomErrorResponsePolicy takes effect.
        /// When used in conjunction with pathMatcher.defaultRouteAction.retryPolicy, retries take precedence. Only once all retries are exhausted, the defaultCustomErrorResponsePolicy is applied. While attempting a retry, if load balancer is successful in reaching the service, the defaultCustomErrorResponsePolicy is ignored and the response from the service is returned to the client.
        /// defaultCustomErrorResponsePolicy is supported only for global external Application Load Balancers.
        /// Structure is documented below.
        #[builder(into, default)]
        pub default_custom_error_response_policy: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::compute::UrlMapDefaultCustomErrorResponsePolicy>,
        >,
        /// defaultRouteAction takes effect when none of the hostRules match. The load balancer performs advanced routing actions
        /// like URL rewrites, header transformations, etc. prior to forwarding the request to the selected backend.
        /// If defaultRouteAction specifies any weightedBackendServices, defaultService must not be set. Conversely if defaultService
        /// is set, defaultRouteAction cannot contain any weightedBackendServices.
        /// Only one of defaultRouteAction or defaultUrlRedirect must be set.
        /// Structure is documented below.
        #[builder(into, default)]
        pub default_route_action: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::compute::UrlMapDefaultRouteAction>,
        >,
        /// The backend service or backend bucket to use when none of the given rules match.
        #[builder(into, default)]
        pub default_service: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// When none of the specified hostRules match, the request is redirected to a URL specified
        /// by defaultUrlRedirect. If defaultUrlRedirect is specified, defaultService or
        /// defaultRouteAction must not be set.
        /// Structure is documented below.
        #[builder(into, default)]
        pub default_url_redirect: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::compute::UrlMapDefaultUrlRedirect>,
        >,
        /// An optional description of this resource. Provide this property when you create
        /// the resource.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies changes to request and response headers that need to take effect for
        /// the selected backendService. The headerAction specified here take effect after
        /// headerAction specified under pathMatcher.
        /// Structure is documented below.
        #[builder(into, default)]
        pub header_action: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::compute::UrlMapHeaderAction>,
        >,
        /// The list of HostRules to use against the URL.
        /// Structure is documented below.
        #[builder(into, default)]
        pub host_rules: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::compute::UrlMapHostRule>>,
        >,
        /// Name of the resource. Provided by the client when the resource is created. The
        /// name must be 1-63 characters long, and comply with RFC1035. Specifically, the
        /// name must be 1-63 characters long and match the regular expression
        /// `a-z?` which means the first character must be a lowercase
        /// letter, and all following characters must be a dash, lowercase letter, or digit,
        /// except the last character, which cannot be a dash.
        ///
        ///
        /// - - -
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The list of named PathMatchers to use against the URL.
        /// Structure is documented below.
        #[builder(into, default)]
        pub path_matchers: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::compute::UrlMapPathMatcher>>,
        >,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The list of expected URL mapping tests. Request to update this UrlMap will
        /// succeed only if all of the test cases pass. You can specify a maximum of 100
        /// tests per UrlMap.
        /// Structure is documented below.
        #[builder(into, default)]
        pub tests: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::compute::UrlMapTest>>,
        >,
    }
    #[allow(dead_code)]
    pub struct URLMapResult {
        /// Creation timestamp in RFC3339 text format.
        pub creation_timestamp: pulumi_gestalt_rust::Output<String>,
        /// defaultCustomErrorResponsePolicy specifies how the Load Balancer returns error responses when BackendService or BackendBucket responds with an error.
        /// This policy takes effect at the PathMatcher level and applies only when no policy has been defined for the error code at lower levels like RouteRule and PathRule within this PathMatcher. If an error code does not have a policy defined in defaultCustomErrorResponsePolicy, then a policy defined for the error code in UrlMap.defaultCustomErrorResponsePolicy takes effect.
        /// For example, consider a UrlMap with the following configuration:
        /// UrlMap.defaultCustomErrorResponsePolicy is configured with policies for 5xx and 4xx errors
        /// A RouteRule for /coming_soon/ is configured for the error code 404.
        /// If the request is for www.myotherdomain.com and a 404 is encountered, the policy under UrlMap.defaultCustomErrorResponsePolicy takes effect. If a 404 response is encountered for the request www.example.com/current_events/, the pathMatcher's policy takes effect. If however, the request for www.example.com/coming_soon/ encounters a 404, the policy in RouteRule.customErrorResponsePolicy takes effect. If any of the requests in this example encounter a 500 error code, the policy at UrlMap.defaultCustomErrorResponsePolicy takes effect.
        /// When used in conjunction with pathMatcher.defaultRouteAction.retryPolicy, retries take precedence. Only once all retries are exhausted, the defaultCustomErrorResponsePolicy is applied. While attempting a retry, if load balancer is successful in reaching the service, the defaultCustomErrorResponsePolicy is ignored and the response from the service is returned to the client.
        /// defaultCustomErrorResponsePolicy is supported only for global external Application Load Balancers.
        /// Structure is documented below.
        pub default_custom_error_response_policy: pulumi_gestalt_rust::Output<
            Option<super::super::types::compute::UrlMapDefaultCustomErrorResponsePolicy>,
        >,
        /// defaultRouteAction takes effect when none of the hostRules match. The load balancer performs advanced routing actions
        /// like URL rewrites, header transformations, etc. prior to forwarding the request to the selected backend.
        /// If defaultRouteAction specifies any weightedBackendServices, defaultService must not be set. Conversely if defaultService
        /// is set, defaultRouteAction cannot contain any weightedBackendServices.
        /// Only one of defaultRouteAction or defaultUrlRedirect must be set.
        /// Structure is documented below.
        pub default_route_action: pulumi_gestalt_rust::Output<
            Option<super::super::types::compute::UrlMapDefaultRouteAction>,
        >,
        /// The backend service or backend bucket to use when none of the given rules match.
        pub default_service: pulumi_gestalt_rust::Output<Option<String>>,
        /// When none of the specified hostRules match, the request is redirected to a URL specified
        /// by defaultUrlRedirect. If defaultUrlRedirect is specified, defaultService or
        /// defaultRouteAction must not be set.
        /// Structure is documented below.
        pub default_url_redirect: pulumi_gestalt_rust::Output<
            Option<super::super::types::compute::UrlMapDefaultUrlRedirect>,
        >,
        /// An optional description of this resource. Provide this property when you create
        /// the resource.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Fingerprint of this resource. A hash of the contents stored in this object. This
        /// field is used in optimistic locking.
        pub fingerprint: pulumi_gestalt_rust::Output<String>,
        /// Specifies changes to request and response headers that need to take effect for
        /// the selected backendService. The headerAction specified here take effect after
        /// headerAction specified under pathMatcher.
        /// Structure is documented below.
        pub header_action: pulumi_gestalt_rust::Output<
            Option<super::super::types::compute::UrlMapHeaderAction>,
        >,
        /// The list of HostRules to use against the URL.
        /// Structure is documented below.
        pub host_rules: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::compute::UrlMapHostRule>>,
        >,
        /// The unique identifier for the resource.
        pub map_id: pulumi_gestalt_rust::Output<i32>,
        /// Name of the resource. Provided by the client when the resource is created. The
        /// name must be 1-63 characters long, and comply with RFC1035. Specifically, the
        /// name must be 1-63 characters long and match the regular expression
        /// `a-z?` which means the first character must be a lowercase
        /// letter, and all following characters must be a dash, lowercase letter, or digit,
        /// except the last character, which cannot be a dash.
        ///
        ///
        /// - - -
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The list of named PathMatchers to use against the URL.
        /// Structure is documented below.
        pub path_matchers: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::compute::UrlMapPathMatcher>>,
        >,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The URI of the created resource.
        pub self_link: pulumi_gestalt_rust::Output<String>,
        /// The list of expected URL mapping tests. Request to update this UrlMap will
        /// succeed only if all of the test cases pass. You can specify a maximum of 100
        /// tests per UrlMap.
        /// Structure is documented below.
        pub tests: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::compute::UrlMapTest>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: URLMapArgs,
    ) -> URLMapResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let default_custom_error_response_policy_binding = args
            .default_custom_error_response_policy
            .get_output(context)
            .get_inner();
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
        let header_action_binding = args.header_action.get_output(context).get_inner();
        let host_rules_binding = args.host_rules.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let path_matchers_binding = args.path_matchers.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let tests_binding = args.tests.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:compute/uRLMap:URLMap".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "defaultCustomErrorResponsePolicy".into(),
                    value: &default_custom_error_response_policy_binding,
                },
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
                    name: "headerAction".into(),
                    value: &header_action_binding,
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
                    name: "tests".into(),
                    value: &tests_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        URLMapResult {
            creation_timestamp: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("creationTimestamp"),
            ),
            default_custom_error_response_policy: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("defaultCustomErrorResponsePolicy"),
            ),
            default_route_action: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("defaultRouteAction"),
            ),
            default_service: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("defaultService"),
            ),
            default_url_redirect: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("defaultUrlRedirect"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            fingerprint: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("fingerprint"),
            ),
            header_action: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("headerAction"),
            ),
            host_rules: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("hostRules"),
            ),
            map_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("mapId"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            path_matchers: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("pathMatchers"),
            ),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            self_link: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("selfLink"),
            ),
            tests: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tests")),
        }
    }
}
