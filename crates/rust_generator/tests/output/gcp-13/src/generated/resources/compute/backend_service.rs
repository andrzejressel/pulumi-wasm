/// A Backend Service defines a group of virtual machines that will serve
/// traffic for load balancing. This resource is a global backend service,
/// appropriate for external load balancing or self-managed internal load balancing.
/// For managed internal load balancing, use a regional backend service instead.
///
/// Currently self-managed internal load balancing is only available in beta.
///
///
/// To get more information about BackendService, see:
///
/// * [API documentation](https://cloud.google.com/compute/docs/reference/v1/backendServices)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/compute/docs/load-balancing/http/backend-service)
///
/// > **Warning:** All arguments including the following potentially sensitive
/// values will be stored in the raw state as plain text: `iap.oauth2_client_secret`, `iap.oauth2_client_secret_sha256`, `security_settings.aws_v4_authentication.access_key`.
///
/// ## Example Usage
///
/// ### Backend Service Basic
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = backend_service::create(
///         "default",
///         BackendServiceArgs::builder()
///             .health_checks("${defaultHttpHealthCheck.id}")
///             .name("backend-service")
///             .build_struct(),
///     );
///     let defaultHttpHealthCheck = http_health_check::create(
///         "defaultHttpHealthCheck",
///         HttpHealthCheckArgs::builder()
///             .check_interval_sec(1)
///             .name("health-check")
///             .request_path("/")
///             .timeout_sec(1)
///             .build_struct(),
///     );
/// }
/// ```
/// ### Backend Service External Iap
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = backend_service::create(
///         "default",
///         BackendServiceArgs::builder()
///             .iap(
///                 BackendServiceIap::builder()
///                     .enabled(true)
///                     .oauth2ClientId("abc")
///                     .oauth2ClientSecret("xyz")
///                     .build_struct(),
///             )
///             .load_balancing_scheme("EXTERNAL")
///             .name("tf-test-backend-service-external")
///             .protocol("HTTP")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Backend Service Cache Simple
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = backend_service::create(
///         "default",
///         BackendServiceArgs::builder()
///             .cdn_policy(
///                 BackendServiceCdnPolicy::builder()
///                     .signedUrlCacheMaxAgeSec(7200)
///                     .build_struct(),
///             )
///             .enable_cdn(true)
///             .health_checks("${defaultHttpHealthCheck.id}")
///             .name("backend-service")
///             .build_struct(),
///     );
///     let defaultHttpHealthCheck = http_health_check::create(
///         "defaultHttpHealthCheck",
///         HttpHealthCheckArgs::builder()
///             .check_interval_sec(1)
///             .name("health-check")
///             .request_path("/")
///             .timeout_sec(1)
///             .build_struct(),
///     );
/// }
/// ```
/// ### Backend Service Cache Include Http Headers
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = backend_service::create(
///         "default",
///         BackendServiceArgs::builder()
///             .cdn_policy(
///                 BackendServiceCdnPolicy::builder()
///                     .cacheKeyPolicy(
///                         BackendServiceCdnPolicyCacheKeyPolicy::builder()
///                             .includeHost(true)
///                             .includeHttpHeaders(vec!["X-My-Header-Field",])
///                             .includeProtocol(true)
///                             .includeQueryString(true)
///                             .build_struct(),
///                     )
///                     .cacheMode("USE_ORIGIN_HEADERS")
///                     .build_struct(),
///             )
///             .enable_cdn(true)
///             .name("backend-service")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Backend Service Cache Include Named Cookies
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = backend_service::create(
///         "default",
///         BackendServiceArgs::builder()
///             .cdn_policy(
///                 BackendServiceCdnPolicy::builder()
///                     .cacheKeyPolicy(
///                         BackendServiceCdnPolicyCacheKeyPolicy::builder()
///                             .includeHost(true)
///                             .includeNamedCookies(
///                                 vec!["__next_preview_data", "__prerender_bypass",],
///                             )
///                             .includeProtocol(true)
///                             .includeQueryString(true)
///                             .build_struct(),
///                     )
///                     .cacheMode("CACHE_ALL_STATIC")
///                     .clientTtl(7200)
///                     .defaultTtl(3600)
///                     .maxTtl(10800)
///                     .build_struct(),
///             )
///             .enable_cdn(true)
///             .name("backend-service")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Backend Service Cache
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = backend_service::create(
///         "default",
///         BackendServiceArgs::builder()
///             .cdn_policy(
///                 BackendServiceCdnPolicy::builder()
///                     .cacheMode("CACHE_ALL_STATIC")
///                     .clientTtl(7200)
///                     .defaultTtl(3600)
///                     .maxTtl(10800)
///                     .negativeCaching(true)
///                     .signedUrlCacheMaxAgeSec(7200)
///                     .build_struct(),
///             )
///             .enable_cdn(true)
///             .health_checks("${defaultHttpHealthCheck.id}")
///             .name("backend-service")
///             .build_struct(),
///     );
///     let defaultHttpHealthCheck = http_health_check::create(
///         "defaultHttpHealthCheck",
///         HttpHealthCheckArgs::builder()
///             .check_interval_sec(1)
///             .name("health-check")
///             .request_path("/")
///             .timeout_sec(1)
///             .build_struct(),
///     );
/// }
/// ```
/// ### Backend Service Cache Bypass Cache On Request Headers
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = backend_service::create(
///         "default",
///         BackendServiceArgs::builder()
///             .cdn_policy(
///                 BackendServiceCdnPolicy::builder()
///                     .bypassCacheOnRequestHeaders(
///                         vec![
///                             BackendServiceCdnPolicyBypassCacheOnRequestHeader::builder()
///                             .headerName("Authorization").build_struct(),
///                             BackendServiceCdnPolicyBypassCacheOnRequestHeader::builder()
///                             .headerName("Proxy-Authorization").build_struct(),
///                         ],
///                     )
///                     .cacheMode("CACHE_ALL_STATIC")
///                     .clientTtl(7200)
///                     .defaultTtl(3600)
///                     .maxTtl(10800)
///                     .negativeCaching(true)
///                     .signedUrlCacheMaxAgeSec(7200)
///                     .build_struct(),
///             )
///             .enable_cdn(true)
///             .health_checks("${defaultHttpHealthCheck.id}")
///             .name("backend-service")
///             .build_struct(),
///     );
///     let defaultHttpHealthCheck = http_health_check::create(
///         "defaultHttpHealthCheck",
///         HttpHealthCheckArgs::builder()
///             .check_interval_sec(1)
///             .name("health-check")
///             .request_path("/")
///             .timeout_sec(1)
///             .build_struct(),
///     );
/// }
/// ```
/// ### Backend Service Traffic Director Round Robin
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = backend_service::create(
///         "default",
///         BackendServiceArgs::builder()
///             .health_checks("${healthCheck.id}")
///             .load_balancing_scheme("INTERNAL_SELF_MANAGED")
///             .locality_lb_policy("ROUND_ROBIN")
///             .name("backend-service")
///             .build_struct(),
///     );
///     let healthCheck = health_check::create(
///         "healthCheck",
///         HealthCheckArgs::builder()
///             .http_health_check(
///                 HealthCheckHttpHealthCheck::builder().port(80).build_struct(),
///             )
///             .name("health-check")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Backend Service Traffic Director Ring Hash
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = backend_service::create(
///         "default",
///         BackendServiceArgs::builder()
///             .circuit_breakers(
///                 BackendServiceCircuitBreakers::builder()
///                     .maxConnections(10)
///                     .build_struct(),
///             )
///             .consistent_hash(
///                 BackendServiceConsistentHash::builder()
///                     .httpCookie(
///                         BackendServiceConsistentHashHttpCookie::builder()
///                             .name("mycookie")
///                             .ttl(
///                                 BackendServiceConsistentHashHttpCookieTtl::builder()
///                                     .nanos(1111)
///                                     .seconds(11)
///                                     .build_struct(),
///                             )
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .health_checks("${healthCheck.id}")
///             .load_balancing_scheme("INTERNAL_SELF_MANAGED")
///             .locality_lb_policy("RING_HASH")
///             .name("backend-service")
///             .outlier_detection(
///                 BackendServiceOutlierDetection::builder()
///                     .consecutiveErrors(2)
///                     .consecutiveGatewayFailure(5)
///                     .enforcingConsecutiveErrors(100)
///                     .enforcingConsecutiveGatewayFailure(0)
///                     .enforcingSuccessRate(100)
///                     .maxEjectionPercent(10)
///                     .successRateMinimumHosts(5)
///                     .successRateRequestVolume(100)
///                     .successRateStdevFactor(1900)
///                     .build_struct(),
///             )
///             .session_affinity("HTTP_COOKIE")
///             .build_struct(),
///     );
///     let healthCheck = health_check::create(
///         "healthCheck",
///         HealthCheckArgs::builder()
///             .http_health_check(
///                 HealthCheckHttpHealthCheck::builder().port(80).build_struct(),
///             )
///             .name("health-check")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Backend Service Stateful Session Affinity
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = backend_service::create(
///         "default",
///         BackendServiceArgs::builder()
///             .health_checks("${healthCheck.id}")
///             .load_balancing_scheme("EXTERNAL_MANAGED")
///             .locality_lb_policy("RING_HASH")
///             .name("backend-service")
///             .session_affinity("STRONG_COOKIE_AFFINITY")
///             .strong_session_affinity_cookie(
///                 BackendServiceStrongSessionAffinityCookie::builder()
///                     .name("mycookie")
///                     .ttl(
///                         BackendServiceStrongSessionAffinityCookieTtl::builder()
///                             .nanos(1111)
///                             .seconds(11)
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
///     let healthCheck = health_check::create(
///         "healthCheck",
///         HealthCheckArgs::builder()
///             .http_health_check(
///                 HealthCheckHttpHealthCheck::builder().port(80).build_struct(),
///             )
///             .name("health-check")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Backend Service Network Endpoint
///
///
/// ```yaml
/// resources:
///   externalProxy:
///     type: gcp:compute:GlobalNetworkEndpointGroup
///     name: external_proxy
///     properties:
///       name: network-endpoint
///       networkEndpointType: INTERNET_FQDN_PORT
///       defaultPort: '443'
///   proxy:
///     type: gcp:compute:GlobalNetworkEndpoint
///     properties:
///       globalNetworkEndpointGroup: ${externalProxy.id}
///       fqdn: test.example.com
///       port: ${externalProxy.defaultPort}
///   default:
///     type: gcp:compute:BackendService
///     properties:
///       name: backend-service
///       enableCdn: true
///       timeoutSec: 10
///       connectionDrainingTimeoutSec: 10
///       customRequestHeaders:
///         - 'host: ${proxy.fqdn}'
///       customResponseHeaders:
///         - 'X-Cache-Hit: {cdn_cache_status}'
///       backends:
///         - group: ${externalProxy.id}
/// ```
/// ### Backend Service External Managed
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = backend_service::create(
///         "default",
///         BackendServiceArgs::builder()
///             .health_checks("${defaultHealthCheck.id}")
///             .load_balancing_scheme("EXTERNAL_MANAGED")
///             .name("backend-service")
///             .build_struct(),
///     );
///     let defaultHealthCheck = health_check::create(
///         "defaultHealthCheck",
///         HealthCheckArgs::builder()
///             .http_health_check(
///                 HealthCheckHttpHealthCheck::builder().port(80).build_struct(),
///             )
///             .name("health-check")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Backend Service Ip Address Selection Policy
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = backend_service::create(
///         "default",
///         BackendServiceArgs::builder()
///             .ip_address_selection_policy("IPV6_ONLY")
///             .load_balancing_scheme("EXTERNAL_MANAGED")
///             .name("backend-service")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// BackendService can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/global/backendServices/{{name}}`
///
/// * `{{project}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, BackendService can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:compute/backendService:BackendService default projects/{{project}}/global/backendServices/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/backendService:BackendService default {{project}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/backendService:BackendService default {{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod backend_service {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BackendServiceArgs {
        /// Lifetime of cookies in seconds if session_affinity is
        /// GENERATED_COOKIE. If set to 0, the cookie is non-persistent and lasts
        /// only until the end of the browser session (or equivalent). The
        /// maximum allowed value for TTL is one day.
        /// When the load balancing scheme is INTERNAL, this field is not used.
        #[builder(into, default)]
        pub affinity_cookie_ttl_sec: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The set of backends that serve this BackendService.
        /// Structure is documented below.
        #[builder(into, default)]
        pub backends: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::compute::BackendServiceBackend>>,
        >,
        /// Cloud CDN configuration for this BackendService.
        /// Structure is documented below.
        #[builder(into, default)]
        pub cdn_policy: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::compute::BackendServiceCdnPolicy>,
        >,
        /// Settings controlling the volume of connections to a backend service. This field
        /// is applicable only when the load_balancing_scheme is set to INTERNAL_SELF_MANAGED.
        /// Structure is documented below.
        #[builder(into, default)]
        pub circuit_breakers: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::compute::BackendServiceCircuitBreakers>,
        >,
        /// Compress text responses using Brotli or gzip compression, based on the client's Accept-Encoding header.
        /// Possible values are: `AUTOMATIC`, `DISABLED`.
        #[builder(into, default)]
        pub compression_mode: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Time for which instance will be drained (not accept new
        /// connections, but still work to finish started).
        #[builder(into, default)]
        pub connection_draining_timeout_sec: pulumi_gestalt_rust::InputOrOutput<
            Option<i32>,
        >,
        /// Consistent Hash-based load balancing can be used to provide soft session
        /// affinity based on HTTP headers, cookies or other properties. This load balancing
        /// policy is applicable only for HTTP connections. The affinity to a particular
        /// destination host will be lost when one or more hosts are added/removed from the
        /// destination service. This field specifies parameters that control consistent
        /// hashing. This field only applies if the load_balancing_scheme is set to
        /// INTERNAL_SELF_MANAGED. This field is only applicable when locality_lb_policy is
        /// set to MAGLEV or RING_HASH.
        /// Structure is documented below.
        #[builder(into, default)]
        pub consistent_hash: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::compute::BackendServiceConsistentHash>,
        >,
        /// Headers that the HTTP/S load balancer should add to proxied
        /// requests.
        #[builder(into, default)]
        pub custom_request_headers: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// Headers that the HTTP/S load balancer should add to proxied
        /// responses.
        #[builder(into, default)]
        pub custom_response_headers: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// An optional description of this resource.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The resource URL for the edge security policy associated with this backend service.
        #[builder(into, default)]
        pub edge_security_policy: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// If true, enable Cloud CDN for this BackendService.
        #[builder(into, default)]
        pub enable_cdn: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The set of URLs to the HttpHealthCheck or HttpsHealthCheck resource
        /// for health checking this BackendService. Currently at most one health
        /// check can be specified.
        /// A health check must be specified unless the backend service uses an internet
        /// or serverless NEG as a backend.
        /// For internal load balancing, a URL to a HealthCheck resource must be specified instead.
        #[builder(into, default)]
        pub health_checks: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Settings for enabling Cloud Identity Aware Proxy
        /// Structure is documented below.
        #[builder(into, default)]
        pub iap: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::compute::BackendServiceIap>,
        >,
        /// Specifies preference of traffic to the backend (from the proxy and from the client for proxyless gRPC).
        /// Possible values are: `IPV4_ONLY`, `PREFER_IPV6`, `IPV6_ONLY`.
        #[builder(into, default)]
        pub ip_address_selection_policy: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Indicates whether the backend service will be used with internal or
        /// external load balancing. A backend service created for one type of
        /// load balancing cannot be used with the other. For more information, refer to
        /// [Choosing a load balancer](https://cloud.google.com/load-balancing/docs/backend-service).
        /// Default value is `EXTERNAL`.
        /// Possible values are: `EXTERNAL`, `INTERNAL_SELF_MANAGED`, `INTERNAL_MANAGED`, `EXTERNAL_MANAGED`.
        #[builder(into, default)]
        pub load_balancing_scheme: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A list of locality load balancing policies to be used in order of
        /// preference. Either the policy or the customPolicy field should be set.
        /// Overrides any value set in the localityLbPolicy field.
        /// localityLbPolicies is only supported when the BackendService is referenced
        /// by a URL Map that is referenced by a target gRPC proxy that has the
        /// validateForProxyless field set to true.
        /// Structure is documented below.
        #[builder(into, default)]
        pub locality_lb_policies: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::compute::BackendServiceLocalityLbPolicy>>,
        >,
        /// The load balancing algorithm used within the scope of the locality.
        /// The possible values are:
        /// * `ROUND_ROBIN`: This is a simple policy in which each healthy backend
        /// is selected in round robin order.
        /// * `LEAST_REQUEST`: An O(1) algorithm which selects two random healthy
        /// hosts and picks the host which has fewer active requests.
        /// * `RING_HASH`: The ring/modulo hash load balancer implements consistent
        /// hashing to backends. The algorithm has the property that the
        /// addition/removal of a host from a set of N hosts only affects
        /// 1/N of the requests.
        /// * `RANDOM`: The load balancer selects a random healthy host.
        /// * `ORIGINAL_DESTINATION`: Backend host is selected based on the client
        /// connection metadata, i.e., connections are opened
        /// to the same address as the destination address of
        /// the incoming connection before the connection
        /// was redirected to the load balancer.
        /// * `MAGLEV`: used as a drop in replacement for the ring hash load balancer.
        /// Maglev is not as stable as ring hash but has faster table lookup
        /// build times and host selection times. For more information about
        /// Maglev, refer to https://ai.google/research/pubs/pub44824
        /// * `WEIGHTED_MAGLEV`: Per-instance weighted Load Balancing via health check
        /// reported weights. Only applicable to loadBalancingScheme
        /// EXTERNAL. If set, the Backend Service must
        /// configure a non legacy HTTP-based Health Check, and
        /// health check replies are expected to contain
        /// non-standard HTTP response header field
        /// X-Load-Balancing-Endpoint-Weight to specify the
        /// per-instance weights. If set, Load Balancing is weight
        /// based on the per-instance weights reported in the last
        /// processed health check replies, as long as every
        /// instance either reported a valid weight or had
        /// UNAVAILABLE_WEIGHT. Otherwise, Load Balancing remains
        /// equal-weight.
        /// locality_lb_policy is applicable to either:
        /// * A regional backend service with the service_protocol set to HTTP, HTTPS, or HTTP2,
        /// and loadBalancingScheme set to INTERNAL_MANAGED.
        /// * A global backend service with the load_balancing_scheme set to INTERNAL_SELF_MANAGED.
        /// * A regional backend service with loadBalancingScheme set to EXTERNAL (External Network
        /// Load Balancing). Only MAGLEV and WEIGHTED_MAGLEV values are possible for External
        /// Network Load Balancing. The default is MAGLEV.
        /// If session_affinity is not NONE, and locality_lb_policy is not set to MAGLEV, WEIGHTED_MAGLEV,
        /// or RING_HASH, session affinity settings will not take effect.
        /// Only ROUND_ROBIN and RING_HASH are supported when the backend service is referenced
        /// by a URL map that is bound to target gRPC proxy that has validate_for_proxyless
        /// field set to true.
        /// Possible values are: `ROUND_ROBIN`, `LEAST_REQUEST`, `RING_HASH`, `RANDOM`, `ORIGINAL_DESTINATION`, `MAGLEV`, `WEIGHTED_MAGLEV`.
        #[builder(into, default)]
        pub locality_lb_policy: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// This field denotes the logging options for the load balancer traffic served by this backend service.
        /// If logging is enabled, logs will be exported to Stackdriver.
        /// Structure is documented below.
        #[builder(into, default)]
        pub log_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::compute::BackendServiceLogConfig>,
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
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Settings controlling eviction of unhealthy hosts from the load balancing pool.
        /// Applicable backend service types can be a global backend service with the
        /// loadBalancingScheme set to INTERNAL_SELF_MANAGED or EXTERNAL_MANAGED.
        /// Structure is documented below.
        #[builder(into, default)]
        pub outlier_detection: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::compute::BackendServiceOutlierDetection>,
        >,
        /// Name of backend port. The same name should appear in the instance
        /// groups referenced by this service. Required when the load balancing
        /// scheme is EXTERNAL.
        #[builder(into, default)]
        pub port_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The protocol this BackendService uses to communicate with backends.
        /// The default is HTTP. **NOTE**: HTTP2 is only valid for beta HTTP/2 load balancer
        /// types and may result in errors if used with the GA API. **NOTE**: With protocol “UNSPECIFIED”,
        /// the backend service can be used by Layer 4 Internal Load Balancing or Network Load Balancing
        /// with TCP/UDP/L3_DEFAULT Forwarding Rule protocol.
        /// Possible values are: `HTTP`, `HTTPS`, `HTTP2`, `TCP`, `SSL`, `GRPC`, `UNSPECIFIED`.
        #[builder(into, default)]
        pub protocol: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The security policy associated with this backend service.
        #[builder(into, default)]
        pub security_policy: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The security settings that apply to this backend service. This field is applicable to either
        /// a regional backend service with the service_protocol set to HTTP, HTTPS, or HTTP2, and
        /// load_balancing_scheme set to INTERNAL_MANAGED; or a global backend service with the
        /// load_balancing_scheme set to INTERNAL_SELF_MANAGED.
        /// Structure is documented below.
        #[builder(into, default)]
        pub security_settings: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::compute::BackendServiceSecuritySettings>,
        >,
        /// URL to networkservices.ServiceLbPolicy resource.
        /// Can only be set if load balancing scheme is EXTERNAL, EXTERNAL_MANAGED, INTERNAL_MANAGED or INTERNAL_SELF_MANAGED and the scope is global.
        #[builder(into, default)]
        pub service_lb_policy: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Type of session affinity to use. The default is NONE. Session affinity is
        /// not applicable if the protocol is UDP.
        /// Possible values are: `NONE`, `CLIENT_IP`, `CLIENT_IP_PORT_PROTO`, `CLIENT_IP_PROTO`, `GENERATED_COOKIE`, `HEADER_FIELD`, `HTTP_COOKIE`, `STRONG_COOKIE_AFFINITY`.
        #[builder(into, default)]
        pub session_affinity: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Describes the HTTP cookie used for stateful session affinity. This field is applicable and required if the sessionAffinity is set to STRONG_COOKIE_AFFINITY.
        /// Structure is documented below.
        #[builder(into, default)]
        pub strong_session_affinity_cookie: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::compute::BackendServiceStrongSessionAffinityCookie,
            >,
        >,
        /// The backend service timeout has a different meaning depending on the type of load balancer.
        /// For more information see, [Backend service settings](https://cloud.google.com/compute/docs/reference/rest/v1/backendServices).
        /// The default is 30 seconds.
        /// The full range of timeout values allowed goes from 1 through 2,147,483,647 seconds.
        #[builder(into, default)]
        pub timeout_sec: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
    }
    #[allow(dead_code)]
    pub struct BackendServiceResult {
        /// Lifetime of cookies in seconds if session_affinity is
        /// GENERATED_COOKIE. If set to 0, the cookie is non-persistent and lasts
        /// only until the end of the browser session (or equivalent). The
        /// maximum allowed value for TTL is one day.
        /// When the load balancing scheme is INTERNAL, this field is not used.
        pub affinity_cookie_ttl_sec: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The set of backends that serve this BackendService.
        /// Structure is documented below.
        pub backends: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::compute::BackendServiceBackend>>,
        >,
        /// Cloud CDN configuration for this BackendService.
        /// Structure is documented below.
        pub cdn_policy: pulumi_gestalt_rust::Output<
            super::super::types::compute::BackendServiceCdnPolicy,
        >,
        /// Settings controlling the volume of connections to a backend service. This field
        /// is applicable only when the load_balancing_scheme is set to INTERNAL_SELF_MANAGED.
        /// Structure is documented below.
        pub circuit_breakers: pulumi_gestalt_rust::Output<
            Option<super::super::types::compute::BackendServiceCircuitBreakers>,
        >,
        /// Compress text responses using Brotli or gzip compression, based on the client's Accept-Encoding header.
        /// Possible values are: `AUTOMATIC`, `DISABLED`.
        pub compression_mode: pulumi_gestalt_rust::Output<Option<String>>,
        /// Time for which instance will be drained (not accept new
        /// connections, but still work to finish started).
        pub connection_draining_timeout_sec: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Consistent Hash-based load balancing can be used to provide soft session
        /// affinity based on HTTP headers, cookies or other properties. This load balancing
        /// policy is applicable only for HTTP connections. The affinity to a particular
        /// destination host will be lost when one or more hosts are added/removed from the
        /// destination service. This field specifies parameters that control consistent
        /// hashing. This field only applies if the load_balancing_scheme is set to
        /// INTERNAL_SELF_MANAGED. This field is only applicable when locality_lb_policy is
        /// set to MAGLEV or RING_HASH.
        /// Structure is documented below.
        pub consistent_hash: pulumi_gestalt_rust::Output<
            Option<super::super::types::compute::BackendServiceConsistentHash>,
        >,
        /// Creation timestamp in RFC3339 text format.
        pub creation_timestamp: pulumi_gestalt_rust::Output<String>,
        /// Headers that the HTTP/S load balancer should add to proxied
        /// requests.
        pub custom_request_headers: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Headers that the HTTP/S load balancer should add to proxied
        /// responses.
        pub custom_response_headers: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// An optional description of this resource.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The resource URL for the edge security policy associated with this backend service.
        pub edge_security_policy: pulumi_gestalt_rust::Output<Option<String>>,
        /// If true, enable Cloud CDN for this BackendService.
        pub enable_cdn: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Fingerprint of this resource. A hash of the contents stored in this
        /// object. This field is used in optimistic locking.
        pub fingerprint: pulumi_gestalt_rust::Output<String>,
        /// The unique identifier for the resource. This identifier is defined by the server.
        pub generated_id: pulumi_gestalt_rust::Output<i32>,
        /// The set of URLs to the HttpHealthCheck or HttpsHealthCheck resource
        /// for health checking this BackendService. Currently at most one health
        /// check can be specified.
        /// A health check must be specified unless the backend service uses an internet
        /// or serverless NEG as a backend.
        /// For internal load balancing, a URL to a HealthCheck resource must be specified instead.
        pub health_checks: pulumi_gestalt_rust::Output<Option<String>>,
        /// Settings for enabling Cloud Identity Aware Proxy
        /// Structure is documented below.
        pub iap: pulumi_gestalt_rust::Output<
            super::super::types::compute::BackendServiceIap,
        >,
        /// Specifies preference of traffic to the backend (from the proxy and from the client for proxyless gRPC).
        /// Possible values are: `IPV4_ONLY`, `PREFER_IPV6`, `IPV6_ONLY`.
        pub ip_address_selection_policy: pulumi_gestalt_rust::Output<Option<String>>,
        /// Indicates whether the backend service will be used with internal or
        /// external load balancing. A backend service created for one type of
        /// load balancing cannot be used with the other. For more information, refer to
        /// [Choosing a load balancer](https://cloud.google.com/load-balancing/docs/backend-service).
        /// Default value is `EXTERNAL`.
        /// Possible values are: `EXTERNAL`, `INTERNAL_SELF_MANAGED`, `INTERNAL_MANAGED`, `EXTERNAL_MANAGED`.
        pub load_balancing_scheme: pulumi_gestalt_rust::Output<Option<String>>,
        /// A list of locality load balancing policies to be used in order of
        /// preference. Either the policy or the customPolicy field should be set.
        /// Overrides any value set in the localityLbPolicy field.
        /// localityLbPolicies is only supported when the BackendService is referenced
        /// by a URL Map that is referenced by a target gRPC proxy that has the
        /// validateForProxyless field set to true.
        /// Structure is documented below.
        pub locality_lb_policies: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::compute::BackendServiceLocalityLbPolicy>>,
        >,
        /// The load balancing algorithm used within the scope of the locality.
        /// The possible values are:
        /// * `ROUND_ROBIN`: This is a simple policy in which each healthy backend
        /// is selected in round robin order.
        /// * `LEAST_REQUEST`: An O(1) algorithm which selects two random healthy
        /// hosts and picks the host which has fewer active requests.
        /// * `RING_HASH`: The ring/modulo hash load balancer implements consistent
        /// hashing to backends. The algorithm has the property that the
        /// addition/removal of a host from a set of N hosts only affects
        /// 1/N of the requests.
        /// * `RANDOM`: The load balancer selects a random healthy host.
        /// * `ORIGINAL_DESTINATION`: Backend host is selected based on the client
        /// connection metadata, i.e., connections are opened
        /// to the same address as the destination address of
        /// the incoming connection before the connection
        /// was redirected to the load balancer.
        /// * `MAGLEV`: used as a drop in replacement for the ring hash load balancer.
        /// Maglev is not as stable as ring hash but has faster table lookup
        /// build times and host selection times. For more information about
        /// Maglev, refer to https://ai.google/research/pubs/pub44824
        /// * `WEIGHTED_MAGLEV`: Per-instance weighted Load Balancing via health check
        /// reported weights. Only applicable to loadBalancingScheme
        /// EXTERNAL. If set, the Backend Service must
        /// configure a non legacy HTTP-based Health Check, and
        /// health check replies are expected to contain
        /// non-standard HTTP response header field
        /// X-Load-Balancing-Endpoint-Weight to specify the
        /// per-instance weights. If set, Load Balancing is weight
        /// based on the per-instance weights reported in the last
        /// processed health check replies, as long as every
        /// instance either reported a valid weight or had
        /// UNAVAILABLE_WEIGHT. Otherwise, Load Balancing remains
        /// equal-weight.
        /// locality_lb_policy is applicable to either:
        /// * A regional backend service with the service_protocol set to HTTP, HTTPS, or HTTP2,
        /// and loadBalancingScheme set to INTERNAL_MANAGED.
        /// * A global backend service with the load_balancing_scheme set to INTERNAL_SELF_MANAGED.
        /// * A regional backend service with loadBalancingScheme set to EXTERNAL (External Network
        /// Load Balancing). Only MAGLEV and WEIGHTED_MAGLEV values are possible for External
        /// Network Load Balancing. The default is MAGLEV.
        /// If session_affinity is not NONE, and locality_lb_policy is not set to MAGLEV, WEIGHTED_MAGLEV,
        /// or RING_HASH, session affinity settings will not take effect.
        /// Only ROUND_ROBIN and RING_HASH are supported when the backend service is referenced
        /// by a URL map that is bound to target gRPC proxy that has validate_for_proxyless
        /// field set to true.
        /// Possible values are: `ROUND_ROBIN`, `LEAST_REQUEST`, `RING_HASH`, `RANDOM`, `ORIGINAL_DESTINATION`, `MAGLEV`, `WEIGHTED_MAGLEV`.
        pub locality_lb_policy: pulumi_gestalt_rust::Output<Option<String>>,
        /// This field denotes the logging options for the load balancer traffic served by this backend service.
        /// If logging is enabled, logs will be exported to Stackdriver.
        /// Structure is documented below.
        pub log_config: pulumi_gestalt_rust::Output<
            super::super::types::compute::BackendServiceLogConfig,
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
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Settings controlling eviction of unhealthy hosts from the load balancing pool.
        /// Applicable backend service types can be a global backend service with the
        /// loadBalancingScheme set to INTERNAL_SELF_MANAGED or EXTERNAL_MANAGED.
        /// Structure is documented below.
        pub outlier_detection: pulumi_gestalt_rust::Output<
            Option<super::super::types::compute::BackendServiceOutlierDetection>,
        >,
        /// Name of backend port. The same name should appear in the instance
        /// groups referenced by this service. Required when the load balancing
        /// scheme is EXTERNAL.
        pub port_name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The protocol this BackendService uses to communicate with backends.
        /// The default is HTTP. **NOTE**: HTTP2 is only valid for beta HTTP/2 load balancer
        /// types and may result in errors if used with the GA API. **NOTE**: With protocol “UNSPECIFIED”,
        /// the backend service can be used by Layer 4 Internal Load Balancing or Network Load Balancing
        /// with TCP/UDP/L3_DEFAULT Forwarding Rule protocol.
        /// Possible values are: `HTTP`, `HTTPS`, `HTTP2`, `TCP`, `SSL`, `GRPC`, `UNSPECIFIED`.
        pub protocol: pulumi_gestalt_rust::Output<String>,
        /// The security policy associated with this backend service.
        pub security_policy: pulumi_gestalt_rust::Output<Option<String>>,
        /// The security settings that apply to this backend service. This field is applicable to either
        /// a regional backend service with the service_protocol set to HTTP, HTTPS, or HTTP2, and
        /// load_balancing_scheme set to INTERNAL_MANAGED; or a global backend service with the
        /// load_balancing_scheme set to INTERNAL_SELF_MANAGED.
        /// Structure is documented below.
        pub security_settings: pulumi_gestalt_rust::Output<
            Option<super::super::types::compute::BackendServiceSecuritySettings>,
        >,
        /// The URI of the created resource.
        pub self_link: pulumi_gestalt_rust::Output<String>,
        /// URL to networkservices.ServiceLbPolicy resource.
        /// Can only be set if load balancing scheme is EXTERNAL, EXTERNAL_MANAGED, INTERNAL_MANAGED or INTERNAL_SELF_MANAGED and the scope is global.
        pub service_lb_policy: pulumi_gestalt_rust::Output<Option<String>>,
        /// Type of session affinity to use. The default is NONE. Session affinity is
        /// not applicable if the protocol is UDP.
        /// Possible values are: `NONE`, `CLIENT_IP`, `CLIENT_IP_PORT_PROTO`, `CLIENT_IP_PROTO`, `GENERATED_COOKIE`, `HEADER_FIELD`, `HTTP_COOKIE`, `STRONG_COOKIE_AFFINITY`.
        pub session_affinity: pulumi_gestalt_rust::Output<String>,
        /// Describes the HTTP cookie used for stateful session affinity. This field is applicable and required if the sessionAffinity is set to STRONG_COOKIE_AFFINITY.
        /// Structure is documented below.
        pub strong_session_affinity_cookie: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::compute::BackendServiceStrongSessionAffinityCookie,
            >,
        >,
        /// The backend service timeout has a different meaning depending on the type of load balancer.
        /// For more information see, [Backend service settings](https://cloud.google.com/compute/docs/reference/rest/v1/backendServices).
        /// The default is 30 seconds.
        /// The full range of timeout values allowed goes from 1 through 2,147,483,647 seconds.
        pub timeout_sec: pulumi_gestalt_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: BackendServiceArgs,
    ) -> BackendServiceResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let affinity_cookie_ttl_sec_binding_1 = args
            .affinity_cookie_ttl_sec
            .get_output(context);
        let affinity_cookie_ttl_sec_binding = affinity_cookie_ttl_sec_binding_1
            .get_inner();
        let backends_binding_1 = args.backends.get_output(context);
        let backends_binding = backends_binding_1.get_inner();
        let cdn_policy_binding_1 = args.cdn_policy.get_output(context);
        let cdn_policy_binding = cdn_policy_binding_1.get_inner();
        let circuit_breakers_binding_1 = args.circuit_breakers.get_output(context);
        let circuit_breakers_binding = circuit_breakers_binding_1.get_inner();
        let compression_mode_binding_1 = args.compression_mode.get_output(context);
        let compression_mode_binding = compression_mode_binding_1.get_inner();
        let connection_draining_timeout_sec_binding_1 = args
            .connection_draining_timeout_sec
            .get_output(context);
        let connection_draining_timeout_sec_binding = connection_draining_timeout_sec_binding_1
            .get_inner();
        let consistent_hash_binding_1 = args.consistent_hash.get_output(context);
        let consistent_hash_binding = consistent_hash_binding_1.get_inner();
        let custom_request_headers_binding_1 = args
            .custom_request_headers
            .get_output(context);
        let custom_request_headers_binding = custom_request_headers_binding_1
            .get_inner();
        let custom_response_headers_binding_1 = args
            .custom_response_headers
            .get_output(context);
        let custom_response_headers_binding = custom_response_headers_binding_1
            .get_inner();
        let description_binding_1 = args.description.get_output(context);
        let description_binding = description_binding_1.get_inner();
        let edge_security_policy_binding_1 = args
            .edge_security_policy
            .get_output(context);
        let edge_security_policy_binding = edge_security_policy_binding_1.get_inner();
        let enable_cdn_binding_1 = args.enable_cdn.get_output(context);
        let enable_cdn_binding = enable_cdn_binding_1.get_inner();
        let health_checks_binding_1 = args.health_checks.get_output(context);
        let health_checks_binding = health_checks_binding_1.get_inner();
        let iap_binding_1 = args.iap.get_output(context);
        let iap_binding = iap_binding_1.get_inner();
        let ip_address_selection_policy_binding_1 = args
            .ip_address_selection_policy
            .get_output(context);
        let ip_address_selection_policy_binding = ip_address_selection_policy_binding_1
            .get_inner();
        let load_balancing_scheme_binding_1 = args
            .load_balancing_scheme
            .get_output(context);
        let load_balancing_scheme_binding = load_balancing_scheme_binding_1.get_inner();
        let locality_lb_policies_binding_1 = args
            .locality_lb_policies
            .get_output(context);
        let locality_lb_policies_binding = locality_lb_policies_binding_1.get_inner();
        let locality_lb_policy_binding_1 = args.locality_lb_policy.get_output(context);
        let locality_lb_policy_binding = locality_lb_policy_binding_1.get_inner();
        let log_config_binding_1 = args.log_config.get_output(context);
        let log_config_binding = log_config_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let outlier_detection_binding_1 = args.outlier_detection.get_output(context);
        let outlier_detection_binding = outlier_detection_binding_1.get_inner();
        let port_name_binding_1 = args.port_name.get_output(context);
        let port_name_binding = port_name_binding_1.get_inner();
        let project_binding_1 = args.project.get_output(context);
        let project_binding = project_binding_1.get_inner();
        let protocol_binding_1 = args.protocol.get_output(context);
        let protocol_binding = protocol_binding_1.get_inner();
        let security_policy_binding_1 = args.security_policy.get_output(context);
        let security_policy_binding = security_policy_binding_1.get_inner();
        let security_settings_binding_1 = args.security_settings.get_output(context);
        let security_settings_binding = security_settings_binding_1.get_inner();
        let service_lb_policy_binding_1 = args.service_lb_policy.get_output(context);
        let service_lb_policy_binding = service_lb_policy_binding_1.get_inner();
        let session_affinity_binding_1 = args.session_affinity.get_output(context);
        let session_affinity_binding = session_affinity_binding_1.get_inner();
        let strong_session_affinity_cookie_binding_1 = args
            .strong_session_affinity_cookie
            .get_output(context);
        let strong_session_affinity_cookie_binding = strong_session_affinity_cookie_binding_1
            .get_inner();
        let timeout_sec_binding_1 = args.timeout_sec.get_output(context);
        let timeout_sec_binding = timeout_sec_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:compute/backendService:BackendService".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "affinityCookieTtlSec".into(),
                    value: &affinity_cookie_ttl_sec_binding,
                },
                register_interface::ObjectField {
                    name: "backends".into(),
                    value: &backends_binding,
                },
                register_interface::ObjectField {
                    name: "cdnPolicy".into(),
                    value: &cdn_policy_binding,
                },
                register_interface::ObjectField {
                    name: "circuitBreakers".into(),
                    value: &circuit_breakers_binding,
                },
                register_interface::ObjectField {
                    name: "compressionMode".into(),
                    value: &compression_mode_binding,
                },
                register_interface::ObjectField {
                    name: "connectionDrainingTimeoutSec".into(),
                    value: &connection_draining_timeout_sec_binding,
                },
                register_interface::ObjectField {
                    name: "consistentHash".into(),
                    value: &consistent_hash_binding,
                },
                register_interface::ObjectField {
                    name: "customRequestHeaders".into(),
                    value: &custom_request_headers_binding,
                },
                register_interface::ObjectField {
                    name: "customResponseHeaders".into(),
                    value: &custom_response_headers_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "edgeSecurityPolicy".into(),
                    value: &edge_security_policy_binding,
                },
                register_interface::ObjectField {
                    name: "enableCdn".into(),
                    value: &enable_cdn_binding,
                },
                register_interface::ObjectField {
                    name: "healthChecks".into(),
                    value: &health_checks_binding,
                },
                register_interface::ObjectField {
                    name: "iap".into(),
                    value: &iap_binding,
                },
                register_interface::ObjectField {
                    name: "ipAddressSelectionPolicy".into(),
                    value: &ip_address_selection_policy_binding,
                },
                register_interface::ObjectField {
                    name: "loadBalancingScheme".into(),
                    value: &load_balancing_scheme_binding,
                },
                register_interface::ObjectField {
                    name: "localityLbPolicies".into(),
                    value: &locality_lb_policies_binding,
                },
                register_interface::ObjectField {
                    name: "localityLbPolicy".into(),
                    value: &locality_lb_policy_binding,
                },
                register_interface::ObjectField {
                    name: "logConfig".into(),
                    value: &log_config_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "outlierDetection".into(),
                    value: &outlier_detection_binding,
                },
                register_interface::ObjectField {
                    name: "portName".into(),
                    value: &port_name_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "protocol".into(),
                    value: &protocol_binding,
                },
                register_interface::ObjectField {
                    name: "securityPolicy".into(),
                    value: &security_policy_binding,
                },
                register_interface::ObjectField {
                    name: "securitySettings".into(),
                    value: &security_settings_binding,
                },
                register_interface::ObjectField {
                    name: "serviceLbPolicy".into(),
                    value: &service_lb_policy_binding,
                },
                register_interface::ObjectField {
                    name: "sessionAffinity".into(),
                    value: &session_affinity_binding,
                },
                register_interface::ObjectField {
                    name: "strongSessionAffinityCookie".into(),
                    value: &strong_session_affinity_cookie_binding,
                },
                register_interface::ObjectField {
                    name: "timeoutSec".into(),
                    value: &timeout_sec_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        BackendServiceResult {
            affinity_cookie_ttl_sec: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("affinityCookieTtlSec"),
            ),
            backends: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("backends"),
            ),
            cdn_policy: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("cdnPolicy"),
            ),
            circuit_breakers: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("circuitBreakers"),
            ),
            compression_mode: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("compressionMode"),
            ),
            connection_draining_timeout_sec: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("connectionDrainingTimeoutSec"),
            ),
            consistent_hash: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("consistentHash"),
            ),
            creation_timestamp: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("creationTimestamp"),
            ),
            custom_request_headers: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("customRequestHeaders"),
            ),
            custom_response_headers: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("customResponseHeaders"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            edge_security_policy: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("edgeSecurityPolicy"),
            ),
            enable_cdn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("enableCdn"),
            ),
            fingerprint: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("fingerprint"),
            ),
            generated_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("generatedId"),
            ),
            health_checks: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("healthChecks"),
            ),
            iap: pulumi_gestalt_rust::__private::into_domain(o.extract_field("iap")),
            ip_address_selection_policy: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ipAddressSelectionPolicy"),
            ),
            load_balancing_scheme: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("loadBalancingScheme"),
            ),
            locality_lb_policies: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("localityLbPolicies"),
            ),
            locality_lb_policy: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("localityLbPolicy"),
            ),
            log_config: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("logConfig"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            outlier_detection: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("outlierDetection"),
            ),
            port_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("portName"),
            ),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            protocol: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("protocol"),
            ),
            security_policy: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("securityPolicy"),
            ),
            security_settings: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("securitySettings"),
            ),
            self_link: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("selfLink"),
            ),
            service_lb_policy: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("serviceLbPolicy"),
            ),
            session_affinity: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sessionAffinity"),
            ),
            strong_session_affinity_cookie: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("strongSessionAffinityCookie"),
            ),
            timeout_sec: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("timeoutSec"),
            ),
        }
    }
}
