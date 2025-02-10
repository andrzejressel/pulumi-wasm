/// A Region Backend Service defines a regionally-scoped group of virtual
/// machines that will serve traffic for load balancing.
///
///
/// To get more information about RegionBackendService, see:
///
/// * [API documentation](https://cloud.google.com/compute/docs/reference/latest/regionBackendServices)
/// * How-to Guides
///     * [Internal TCP/UDP Load Balancing](https://cloud.google.com/compute/docs/load-balancing/internal/)
///
///
///
/// ## Example Usage
///
/// ### Region Backend Service Basic
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:compute:RegionBackendService
///     properties:
///       name: region-service
///       region: us-central1
///       healthChecks: ${defaultHealthCheck.id}
///       connectionDrainingTimeoutSec: 10
///       sessionAffinity: CLIENT_IP
///   defaultHealthCheck:
///     type: gcp:compute:HealthCheck
///     name: default
///     properties:
///       name: rbs-health-check
///       checkIntervalSec: 1
///       timeoutSec: 1
///       tcpHealthCheck:
///         port: '80'
/// ```
/// ### Region Backend Service External Iap
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = region_backend_service::create(
///         "default",
///         RegionBackendServiceArgs::builder()
///             .iap(
///                 RegionBackendServiceIap::builder()
///                     .enabled(true)
///                     .oauth2ClientId("abc")
///                     .oauth2ClientSecret("xyz")
///                     .build_struct(),
///             )
///             .load_balancing_scheme("EXTERNAL")
///             .name("tf-test-region-service-external")
///             .protocol("HTTP")
///             .region("us-central1")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Region Backend Service Cache
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = region_backend_service::create(
///         "default",
///         RegionBackendServiceArgs::builder()
///             .cdn_policy(
///                 RegionBackendServiceCdnPolicy::builder()
///                     .cacheMode("CACHE_ALL_STATIC")
///                     .clientTtl(7200)
///                     .defaultTtl(3600)
///                     .maxTtl(10800)
///                     .negativeCaching(true)
///                     .signedUrlCacheMaxAgeSec(7200)
///                     .build_struct(),
///             )
///             .enable_cdn(true)
///             .health_checks("${defaultRegionHealthCheck.id}")
///             .load_balancing_scheme("EXTERNAL")
///             .name("region-service")
///             .protocol("HTTP")
///             .region("us-central1")
///             .build_struct(),
///     );
///     let defaultRegionHealthCheck = region_health_check::create(
///         "defaultRegionHealthCheck",
///         RegionHealthCheckArgs::builder()
///             .http_health_check(
///                 RegionHealthCheckHttpHealthCheck::builder().port(80).build_struct(),
///             )
///             .name("rbs-health-check")
///             .region("us-central1")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Region Backend Service Ilb Round Robin
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = region_backend_service::create(
///         "default",
///         RegionBackendServiceArgs::builder()
///             .health_checks("${healthCheck.id}")
///             .load_balancing_scheme("INTERNAL_MANAGED")
///             .locality_lb_policy("ROUND_ROBIN")
///             .name("region-service")
///             .protocol("HTTP")
///             .region("us-central1")
///             .build_struct(),
///     );
///     let healthCheck = health_check::create(
///         "healthCheck",
///         HealthCheckArgs::builder()
///             .http_health_check(
///                 HealthCheckHttpHealthCheck::builder().port(80).build_struct(),
///             )
///             .name("rbs-health-check")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Region Backend Service External
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = region_backend_service::create(
///         "default",
///         RegionBackendServiceArgs::builder()
///             .health_checks("${healthCheck.id}")
///             .load_balancing_scheme("EXTERNAL")
///             .name("region-service")
///             .protocol("TCP")
///             .region("us-central1")
///             .build_struct(),
///     );
///     let healthCheck = region_health_check::create(
///         "healthCheck",
///         RegionHealthCheckArgs::builder()
///             .name("rbs-health-check")
///             .region("us-central1")
///             .tcp_health_check(
///                 RegionHealthCheckTcpHealthCheck::builder().port(80).build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
/// ### Region Backend Service External Weighted
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = region_backend_service::create(
///         "default",
///         RegionBackendServiceArgs::builder()
///             .health_checks("${healthCheck.id}")
///             .load_balancing_scheme("EXTERNAL")
///             .locality_lb_policy("WEIGHTED_MAGLEV")
///             .name("region-service")
///             .protocol("TCP")
///             .region("us-central1")
///             .build_struct(),
///     );
///     let healthCheck = region_health_check::create(
///         "healthCheck",
///         RegionHealthCheckArgs::builder()
///             .http_health_check(
///                 RegionHealthCheckHttpHealthCheck::builder().port(80).build_struct(),
///             )
///             .name("rbs-health-check")
///             .region("us-central1")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Region Backend Service Ilb Ring Hash
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = region_backend_service::create(
///         "default",
///         RegionBackendServiceArgs::builder()
///             .circuit_breakers(
///                 RegionBackendServiceCircuitBreakers::builder()
///                     .maxConnections(10)
///                     .build_struct(),
///             )
///             .consistent_hash(
///                 RegionBackendServiceConsistentHash::builder()
///                     .httpCookie(
///                         RegionBackendServiceConsistentHashHttpCookie::builder()
///                             .name("mycookie")
///                             .ttl(
///                                 RegionBackendServiceConsistentHashHttpCookieTtl::builder()
///                                     .nanos(1111)
///                                     .seconds(11)
///                                     .build_struct(),
///                             )
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .health_checks("${healthCheck.id}")
///             .load_balancing_scheme("INTERNAL_MANAGED")
///             .locality_lb_policy("RING_HASH")
///             .name("region-service")
///             .outlier_detection(
///                 RegionBackendServiceOutlierDetection::builder()
///                     .consecutiveErrors(2)
///                     .build_struct(),
///             )
///             .protocol("HTTP")
///             .region("us-central1")
///             .session_affinity("HTTP_COOKIE")
///             .build_struct(),
///     );
///     let healthCheck = health_check::create(
///         "healthCheck",
///         HealthCheckArgs::builder()
///             .http_health_check(
///                 HealthCheckHttpHealthCheck::builder().port(80).build_struct(),
///             )
///             .name("rbs-health-check")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Region Backend Service Ilb Stateful Session Affinity
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = region_backend_service::create(
///         "default",
///         RegionBackendServiceArgs::builder()
///             .health_checks("${healthCheck.id}")
///             .load_balancing_scheme("INTERNAL_MANAGED")
///             .locality_lb_policy("RING_HASH")
///             .name("region-service")
///             .protocol("HTTP")
///             .region("us-central1")
///             .session_affinity("STRONG_COOKIE_AFFINITY")
///             .strong_session_affinity_cookie(
///                 RegionBackendServiceStrongSessionAffinityCookie::builder()
///                     .name("mycookie")
///                     .ttl(
///                         RegionBackendServiceStrongSessionAffinityCookieTtl::builder()
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
///             .name("rbs-health-check")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Region Backend Service Balancing Mode
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:compute:RegionBackendService
///     properties:
///       loadBalancingScheme: INTERNAL_MANAGED
///       backends:
///         - group: ${rigm.instanceGroup}
///           balancingMode: UTILIZATION
///           capacityScaler: 1
///       region: us-central1
///       name: region-service
///       protocol: HTTP
///       timeoutSec: 10
///       healthChecks: ${defaultRegionHealthCheck.id}
///   rigm:
///     type: gcp:compute:RegionInstanceGroupManager
///     properties:
///       region: us-central1
///       name: rbs-rigm
///       versions:
///         - instanceTemplate: ${instanceTemplate.id}
///           name: primary
///       baseInstanceName: internal-glb
///       targetSize: 1
///   instanceTemplate:
///     type: gcp:compute:InstanceTemplate
///     name: instance_template
///     properties:
///       name: template-region-service
///       machineType: e2-medium
///       networkInterfaces:
///         - network: ${defaultNetwork.id}
///           subnetwork: ${defaultSubnetwork.id}
///       disks:
///         - sourceImage: ${debianImage.selfLink}
///           autoDelete: true
///           boot: true
///       tags:
///         - allow-ssh
///         - load-balanced-backend
///   defaultRegionHealthCheck:
///     type: gcp:compute:RegionHealthCheck
///     name: default
///     properties:
///       region: us-central1
///       name: rbs-health-check
///       httpHealthCheck:
///         portSpecification: USE_SERVING_PORT
///   defaultNetwork:
///     type: gcp:compute:Network
///     name: default
///     properties:
///       name: rbs-net
///       autoCreateSubnetworks: false
///       routingMode: REGIONAL
///   defaultSubnetwork:
///     type: gcp:compute:Subnetwork
///     name: default
///     properties:
///       name: rbs-net-default
///       ipCidrRange: 10.1.2.0/24
///       region: us-central1
///       network: ${defaultNetwork.id}
/// variables:
///   debianImage:
///     fn::invoke:
///       function: gcp:compute:getImage
///       arguments:
///         family: debian-11
///         project: debian-cloud
/// ```
/// ### Region Backend Service Connection Tracking
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = region_backend_service::create(
///         "default",
///         RegionBackendServiceArgs::builder()
///             .connection_draining_timeout_sec(10)
///             .connection_tracking_policy(
///                 RegionBackendServiceConnectionTrackingPolicy::builder()
///                     .connectionPersistenceOnUnhealthyBackends("NEVER_PERSIST")
///                     .enableStrongAffinity(true)
///                     .idleTimeoutSec(60)
///                     .trackingMode("PER_SESSION")
///                     .build_struct(),
///             )
///             .health_checks("${healthCheck.id}")
///             .load_balancing_scheme("EXTERNAL")
///             .name("region-service")
///             .protocol("TCP")
///             .region("us-central1")
///             .session_affinity("CLIENT_IP")
///             .build_struct(),
///     );
///     let healthCheck = region_health_check::create(
///         "healthCheck",
///         RegionHealthCheckArgs::builder()
///             .name("rbs-health-check")
///             .region("us-central1")
///             .tcp_health_check(
///                 RegionHealthCheckTcpHealthCheck::builder().port(22).build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
/// ### Region Backend Service Ip Address Selection Policy
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = region_backend_service::create(
///         "default",
///         RegionBackendServiceArgs::builder()
///             .health_checks("${healthCheck.id}")
///             .ip_address_selection_policy("IPV6_ONLY")
///             .load_balancing_scheme("EXTERNAL_MANAGED")
///             .name("region-service")
///             .protocol("HTTP")
///             .region("us-central1")
///             .build_struct(),
///     );
///     let healthCheck = region_health_check::create(
///         "healthCheck",
///         RegionHealthCheckArgs::builder()
///             .name("rbs-health-check")
///             .region("us-central1")
///             .tcp_health_check(
///                 RegionHealthCheckTcpHealthCheck::builder().port(80).build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// RegionBackendService can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/regions/{{region}}/backendServices/{{name}}`
///
/// * `{{project}}/{{region}}/{{name}}`
///
/// * `{{region}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, RegionBackendService can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:compute/regionBackendService:RegionBackendService default projects/{{project}}/regions/{{region}}/backendServices/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/regionBackendService:RegionBackendService default {{project}}/{{region}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/regionBackendService:RegionBackendService default {{region}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/regionBackendService:RegionBackendService default {{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod region_backend_service {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RegionBackendServiceArgs {
        /// Lifetime of cookies in seconds if session_affinity is
        /// GENERATED_COOKIE. If set to 0, the cookie is non-persistent and lasts
        /// only until the end of the browser session (or equivalent). The
        /// maximum allowed value for TTL is one day.
        /// When the load balancing scheme is INTERNAL, this field is not used.
        #[builder(into, default)]
        pub affinity_cookie_ttl_sec: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The set of backends that serve this RegionBackendService.
        /// Structure is documented below.
        #[builder(into, default)]
        pub backends: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::compute::RegionBackendServiceBackend>>,
        >,
        /// Cloud CDN configuration for this BackendService.
        /// Structure is documented below.
        #[builder(into, default)]
        pub cdn_policy: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::compute::RegionBackendServiceCdnPolicy>,
        >,
        /// Settings controlling the volume of connections to a backend service. This field
        /// is applicable only when the `load_balancing_scheme` is set to INTERNAL_MANAGED
        /// and the `protocol` is set to HTTP, HTTPS, or HTTP2.
        /// Structure is documented below.
        #[builder(into, default)]
        pub circuit_breakers: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::compute::RegionBackendServiceCircuitBreakers>,
        >,
        /// Time for which instance will be drained (not accept new
        /// connections, but still work to finish started).
        #[builder(into, default)]
        pub connection_draining_timeout_sec: pulumi_gestalt_rust::InputOrOutput<
            Option<i32>,
        >,
        /// Connection Tracking configuration for this BackendService.
        /// This is available only for Layer 4 Internal Load Balancing and
        /// Network Load Balancing.
        /// Structure is documented below.
        #[builder(into, default)]
        pub connection_tracking_policy: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::compute::RegionBackendServiceConnectionTrackingPolicy,
            >,
        >,
        /// Consistent Hash-based load balancing can be used to provide soft session
        /// affinity based on HTTP headers, cookies or other properties. This load balancing
        /// policy is applicable only for HTTP connections. The affinity to a particular
        /// destination host will be lost when one or more hosts are added/removed from the
        /// destination service. This field specifies parameters that control consistent
        /// hashing.
        /// This field only applies when all of the following are true -
        #[builder(into, default)]
        pub consistent_hash: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::compute::RegionBackendServiceConsistentHash>,
        >,
        /// An optional description of this resource.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// If true, enable Cloud CDN for this RegionBackendService.
        #[builder(into, default)]
        pub enable_cdn: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Policy for failovers.
        /// Structure is documented below.
        #[builder(into, default)]
        pub failover_policy: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::compute::RegionBackendServiceFailoverPolicy>,
        >,
        /// The set of URLs to HealthCheck resources for health checking
        /// this RegionBackendService. Currently at most one health
        /// check can be specified.
        /// A health check must be specified unless the backend service uses an internet
        /// or serverless NEG as a backend.
        #[builder(into, default)]
        pub health_checks: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Settings for enabling Cloud Identity Aware Proxy
        /// Structure is documented below.
        #[builder(into, default)]
        pub iap: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::compute::RegionBackendServiceIap>,
        >,
        /// Specifies preference of traffic to the backend (from the proxy and from the client for proxyless gRPC).
        /// Possible values are: `IPV4_ONLY`, `PREFER_IPV6`, `IPV6_ONLY`.
        #[builder(into, default)]
        pub ip_address_selection_policy: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Indicates what kind of load balancing this regional backend service
        /// will be used for. A backend service created for one type of load
        /// balancing cannot be used with the other(s). For more information, refer to
        /// [Choosing a load balancer](https://cloud.google.com/load-balancing/docs/backend-service).
        /// Default value is `INTERNAL`.
        /// Possible values are: `EXTERNAL`, `EXTERNAL_MANAGED`, `INTERNAL`, `INTERNAL_MANAGED`.
        #[builder(into, default)]
        pub load_balancing_scheme: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
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
            Option<super::super::types::compute::RegionBackendServiceLogConfig>,
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
        /// The URL of the network to which this backend service belongs.
        /// This field can only be specified when the load balancing scheme is set to INTERNAL.
        #[builder(into, default)]
        pub network: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Settings controlling eviction of unhealthy hosts from the load balancing pool.
        /// This field is applicable only when the `load_balancing_scheme` is set
        /// to INTERNAL_MANAGED and the `protocol` is set to HTTP, HTTPS, or HTTP2.
        /// Structure is documented below.
        #[builder(into, default)]
        pub outlier_detection: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::compute::RegionBackendServiceOutlierDetection>,
        >,
        /// A named port on a backend instance group representing the port for
        /// communication to the backend VMs in that group. Required when the
        /// loadBalancingScheme is EXTERNAL, EXTERNAL_MANAGED, INTERNAL_MANAGED, or INTERNAL_SELF_MANAGED
        /// and the backends are instance groups. The named port must be defined on each
        /// backend instance group. This parameter has no meaning if the backends are NEGs. API sets a
        /// default of "http" if not given.
        /// Must be omitted when the loadBalancingScheme is INTERNAL (Internal TCP/UDP Load Balancing).
        #[builder(into, default)]
        pub port_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The protocol this RegionBackendService uses to communicate with backends.
        /// The default is HTTP. **NOTE**: HTTP2 is only valid for beta HTTP/2 load balancer
        /// types and may result in errors if used with the GA API.
        /// Possible values are: `HTTP`, `HTTPS`, `HTTP2`, `SSL`, `TCP`, `UDP`, `GRPC`, `UNSPECIFIED`.
        #[builder(into, default)]
        pub protocol: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Region in which the created backend service should reside.
        /// If it is not provided, the provider region is used.
        #[builder(into, default)]
        pub region: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The security policy associated with this backend service.
        #[builder(into, default)]
        pub security_policy: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Type of session affinity to use. The default is NONE. Session affinity is
        /// not applicable if the protocol is UDP.
        /// Possible values are: `NONE`, `CLIENT_IP`, `CLIENT_IP_PORT_PROTO`, `CLIENT_IP_PROTO`, `GENERATED_COOKIE`, `HEADER_FIELD`, `HTTP_COOKIE`, `CLIENT_IP_NO_DESTINATION`, `STRONG_COOKIE_AFFINITY`.
        #[builder(into, default)]
        pub session_affinity: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Describes the HTTP cookie used for stateful session affinity. This field is applicable and required if the sessionAffinity is set to STRONG_COOKIE_AFFINITY.
        /// Structure is documented below.
        #[builder(into, default)]
        pub strong_session_affinity_cookie: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::compute::RegionBackendServiceStrongSessionAffinityCookie,
            >,
        >,
        /// Subsetting configuration for this BackendService. Currently this is applicable only for Internal TCP/UDP load balancing and Internal HTTP(S) load balancing.
        /// Structure is documented below.
        #[builder(into, default)]
        pub subsetting: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::compute::RegionBackendServiceSubsetting>,
        >,
        /// The backend service timeout has a different meaning depending on the type of load balancer.
        /// For more information see, [Backend service settings](https://cloud.google.com/compute/docs/reference/rest/v1/backendServices).
        /// The default is 30 seconds.
        /// The full range of timeout values allowed goes from 1 through 2,147,483,647 seconds.
        #[builder(into, default)]
        pub timeout_sec: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
    }
    #[allow(dead_code)]
    pub struct RegionBackendServiceResult {
        /// Lifetime of cookies in seconds if session_affinity is
        /// GENERATED_COOKIE. If set to 0, the cookie is non-persistent and lasts
        /// only until the end of the browser session (or equivalent). The
        /// maximum allowed value for TTL is one day.
        /// When the load balancing scheme is INTERNAL, this field is not used.
        pub affinity_cookie_ttl_sec: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The set of backends that serve this RegionBackendService.
        /// Structure is documented below.
        pub backends: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::compute::RegionBackendServiceBackend>>,
        >,
        /// Cloud CDN configuration for this BackendService.
        /// Structure is documented below.
        pub cdn_policy: pulumi_gestalt_rust::Output<
            super::super::types::compute::RegionBackendServiceCdnPolicy,
        >,
        /// Settings controlling the volume of connections to a backend service. This field
        /// is applicable only when the `load_balancing_scheme` is set to INTERNAL_MANAGED
        /// and the `protocol` is set to HTTP, HTTPS, or HTTP2.
        /// Structure is documented below.
        pub circuit_breakers: pulumi_gestalt_rust::Output<
            Option<super::super::types::compute::RegionBackendServiceCircuitBreakers>,
        >,
        /// Time for which instance will be drained (not accept new
        /// connections, but still work to finish started).
        pub connection_draining_timeout_sec: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Connection Tracking configuration for this BackendService.
        /// This is available only for Layer 4 Internal Load Balancing and
        /// Network Load Balancing.
        /// Structure is documented below.
        pub connection_tracking_policy: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::compute::RegionBackendServiceConnectionTrackingPolicy,
            >,
        >,
        /// Consistent Hash-based load balancing can be used to provide soft session
        /// affinity based on HTTP headers, cookies or other properties. This load balancing
        /// policy is applicable only for HTTP connections. The affinity to a particular
        /// destination host will be lost when one or more hosts are added/removed from the
        /// destination service. This field specifies parameters that control consistent
        /// hashing.
        /// This field only applies when all of the following are true -
        pub consistent_hash: pulumi_gestalt_rust::Output<
            Option<super::super::types::compute::RegionBackendServiceConsistentHash>,
        >,
        /// Creation timestamp in RFC3339 text format.
        pub creation_timestamp: pulumi_gestalt_rust::Output<String>,
        /// An optional description of this resource.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// If true, enable Cloud CDN for this RegionBackendService.
        pub enable_cdn: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Policy for failovers.
        /// Structure is documented below.
        pub failover_policy: pulumi_gestalt_rust::Output<
            Option<super::super::types::compute::RegionBackendServiceFailoverPolicy>,
        >,
        /// Fingerprint of this resource. A hash of the contents stored in this
        /// object. This field is used in optimistic locking.
        pub fingerprint: pulumi_gestalt_rust::Output<String>,
        /// The unique identifier for the resource. This identifier is defined by the server.
        pub generated_id: pulumi_gestalt_rust::Output<i32>,
        /// The set of URLs to HealthCheck resources for health checking
        /// this RegionBackendService. Currently at most one health
        /// check can be specified.
        /// A health check must be specified unless the backend service uses an internet
        /// or serverless NEG as a backend.
        pub health_checks: pulumi_gestalt_rust::Output<Option<String>>,
        /// Settings for enabling Cloud Identity Aware Proxy
        /// Structure is documented below.
        pub iap: pulumi_gestalt_rust::Output<
            super::super::types::compute::RegionBackendServiceIap,
        >,
        /// Specifies preference of traffic to the backend (from the proxy and from the client for proxyless gRPC).
        /// Possible values are: `IPV4_ONLY`, `PREFER_IPV6`, `IPV6_ONLY`.
        pub ip_address_selection_policy: pulumi_gestalt_rust::Output<Option<String>>,
        /// Indicates what kind of load balancing this regional backend service
        /// will be used for. A backend service created for one type of load
        /// balancing cannot be used with the other(s). For more information, refer to
        /// [Choosing a load balancer](https://cloud.google.com/load-balancing/docs/backend-service).
        /// Default value is `INTERNAL`.
        /// Possible values are: `EXTERNAL`, `EXTERNAL_MANAGED`, `INTERNAL`, `INTERNAL_MANAGED`.
        pub load_balancing_scheme: pulumi_gestalt_rust::Output<Option<String>>,
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
            super::super::types::compute::RegionBackendServiceLogConfig,
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
        /// The URL of the network to which this backend service belongs.
        /// This field can only be specified when the load balancing scheme is set to INTERNAL.
        pub network: pulumi_gestalt_rust::Output<Option<String>>,
        /// Settings controlling eviction of unhealthy hosts from the load balancing pool.
        /// This field is applicable only when the `load_balancing_scheme` is set
        /// to INTERNAL_MANAGED and the `protocol` is set to HTTP, HTTPS, or HTTP2.
        /// Structure is documented below.
        pub outlier_detection: pulumi_gestalt_rust::Output<
            Option<super::super::types::compute::RegionBackendServiceOutlierDetection>,
        >,
        /// A named port on a backend instance group representing the port for
        /// communication to the backend VMs in that group. Required when the
        /// loadBalancingScheme is EXTERNAL, EXTERNAL_MANAGED, INTERNAL_MANAGED, or INTERNAL_SELF_MANAGED
        /// and the backends are instance groups. The named port must be defined on each
        /// backend instance group. This parameter has no meaning if the backends are NEGs. API sets a
        /// default of "http" if not given.
        /// Must be omitted when the loadBalancingScheme is INTERNAL (Internal TCP/UDP Load Balancing).
        pub port_name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The protocol this RegionBackendService uses to communicate with backends.
        /// The default is HTTP. **NOTE**: HTTP2 is only valid for beta HTTP/2 load balancer
        /// types and may result in errors if used with the GA API.
        /// Possible values are: `HTTP`, `HTTPS`, `HTTP2`, `SSL`, `TCP`, `UDP`, `GRPC`, `UNSPECIFIED`.
        pub protocol: pulumi_gestalt_rust::Output<String>,
        /// The Region in which the created backend service should reside.
        /// If it is not provided, the provider region is used.
        pub region: pulumi_gestalt_rust::Output<String>,
        /// The security policy associated with this backend service.
        pub security_policy: pulumi_gestalt_rust::Output<Option<String>>,
        /// The URI of the created resource.
        pub self_link: pulumi_gestalt_rust::Output<String>,
        /// Type of session affinity to use. The default is NONE. Session affinity is
        /// not applicable if the protocol is UDP.
        /// Possible values are: `NONE`, `CLIENT_IP`, `CLIENT_IP_PORT_PROTO`, `CLIENT_IP_PROTO`, `GENERATED_COOKIE`, `HEADER_FIELD`, `HTTP_COOKIE`, `CLIENT_IP_NO_DESTINATION`, `STRONG_COOKIE_AFFINITY`.
        pub session_affinity: pulumi_gestalt_rust::Output<String>,
        /// Describes the HTTP cookie used for stateful session affinity. This field is applicable and required if the sessionAffinity is set to STRONG_COOKIE_AFFINITY.
        /// Structure is documented below.
        pub strong_session_affinity_cookie: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::compute::RegionBackendServiceStrongSessionAffinityCookie,
            >,
        >,
        /// Subsetting configuration for this BackendService. Currently this is applicable only for Internal TCP/UDP load balancing and Internal HTTP(S) load balancing.
        /// Structure is documented below.
        pub subsetting: pulumi_gestalt_rust::Output<
            Option<super::super::types::compute::RegionBackendServiceSubsetting>,
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
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: RegionBackendServiceArgs,
    ) -> RegionBackendServiceResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let affinity_cookie_ttl_sec_binding = args
            .affinity_cookie_ttl_sec
            .get_output(context);
        let backends_binding = args.backends.get_output(context);
        let cdn_policy_binding = args.cdn_policy.get_output(context);
        let circuit_breakers_binding = args.circuit_breakers.get_output(context);
        let connection_draining_timeout_sec_binding = args
            .connection_draining_timeout_sec
            .get_output(context);
        let connection_tracking_policy_binding = args
            .connection_tracking_policy
            .get_output(context);
        let consistent_hash_binding = args.consistent_hash.get_output(context);
        let description_binding = args.description.get_output(context);
        let enable_cdn_binding = args.enable_cdn.get_output(context);
        let failover_policy_binding = args.failover_policy.get_output(context);
        let health_checks_binding = args.health_checks.get_output(context);
        let iap_binding = args.iap.get_output(context);
        let ip_address_selection_policy_binding = args
            .ip_address_selection_policy
            .get_output(context);
        let load_balancing_scheme_binding = args
            .load_balancing_scheme
            .get_output(context);
        let locality_lb_policy_binding = args.locality_lb_policy.get_output(context);
        let log_config_binding = args.log_config.get_output(context);
        let name_binding = args.name.get_output(context);
        let network_binding = args.network.get_output(context);
        let outlier_detection_binding = args.outlier_detection.get_output(context);
        let port_name_binding = args.port_name.get_output(context);
        let project_binding = args.project.get_output(context);
        let protocol_binding = args.protocol.get_output(context);
        let region_binding = args.region.get_output(context);
        let security_policy_binding = args.security_policy.get_output(context);
        let session_affinity_binding = args.session_affinity.get_output(context);
        let strong_session_affinity_cookie_binding = args
            .strong_session_affinity_cookie
            .get_output(context);
        let subsetting_binding = args.subsetting.get_output(context);
        let timeout_sec_binding = args.timeout_sec.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:compute/regionBackendService:RegionBackendService".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "affinityCookieTtlSec".into(),
                    value: affinity_cookie_ttl_sec_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "backends".into(),
                    value: backends_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cdnPolicy".into(),
                    value: cdn_policy_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "circuitBreakers".into(),
                    value: circuit_breakers_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "connectionDrainingTimeoutSec".into(),
                    value: connection_draining_timeout_sec_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "connectionTrackingPolicy".into(),
                    value: connection_tracking_policy_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "consistentHash".into(),
                    value: consistent_hash_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enableCdn".into(),
                    value: enable_cdn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "failoverPolicy".into(),
                    value: failover_policy_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "healthChecks".into(),
                    value: health_checks_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "iap".into(),
                    value: iap_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ipAddressSelectionPolicy".into(),
                    value: ip_address_selection_policy_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "loadBalancingScheme".into(),
                    value: load_balancing_scheme_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "localityLbPolicy".into(),
                    value: locality_lb_policy_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "logConfig".into(),
                    value: log_config_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "network".into(),
                    value: network_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "outlierDetection".into(),
                    value: outlier_detection_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "portName".into(),
                    value: port_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "protocol".into(),
                    value: protocol_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "region".into(),
                    value: region_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "securityPolicy".into(),
                    value: security_policy_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sessionAffinity".into(),
                    value: session_affinity_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "strongSessionAffinityCookie".into(),
                    value: strong_session_affinity_cookie_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "subsetting".into(),
                    value: subsetting_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "timeoutSec".into(),
                    value: timeout_sec_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        RegionBackendServiceResult {
            affinity_cookie_ttl_sec: o.get_field("affinityCookieTtlSec"),
            backends: o.get_field("backends"),
            cdn_policy: o.get_field("cdnPolicy"),
            circuit_breakers: o.get_field("circuitBreakers"),
            connection_draining_timeout_sec: o.get_field("connectionDrainingTimeoutSec"),
            connection_tracking_policy: o.get_field("connectionTrackingPolicy"),
            consistent_hash: o.get_field("consistentHash"),
            creation_timestamp: o.get_field("creationTimestamp"),
            description: o.get_field("description"),
            enable_cdn: o.get_field("enableCdn"),
            failover_policy: o.get_field("failoverPolicy"),
            fingerprint: o.get_field("fingerprint"),
            generated_id: o.get_field("generatedId"),
            health_checks: o.get_field("healthChecks"),
            iap: o.get_field("iap"),
            ip_address_selection_policy: o.get_field("ipAddressSelectionPolicy"),
            load_balancing_scheme: o.get_field("loadBalancingScheme"),
            locality_lb_policy: o.get_field("localityLbPolicy"),
            log_config: o.get_field("logConfig"),
            name: o.get_field("name"),
            network: o.get_field("network"),
            outlier_detection: o.get_field("outlierDetection"),
            port_name: o.get_field("portName"),
            project: o.get_field("project"),
            protocol: o.get_field("protocol"),
            region: o.get_field("region"),
            security_policy: o.get_field("securityPolicy"),
            self_link: o.get_field("selfLink"),
            session_affinity: o.get_field("sessionAffinity"),
            strong_session_affinity_cookie: o.get_field("strongSessionAffinityCookie"),
            subsetting: o.get_field("subsetting"),
            timeout_sec: o.get_field("timeoutSec"),
        }
    }
}
