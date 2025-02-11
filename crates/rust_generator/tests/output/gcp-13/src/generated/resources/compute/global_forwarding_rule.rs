/// Represents a GlobalForwardingRule resource. Global forwarding rules are
/// used to forward traffic to the correct load balancer for HTTP load
/// balancing. Global forwarding rules can only be used for HTTP load
/// balancing.
///
/// For more information, see https://cloud.google.com/compute/docs/load-balancing/http/
///
///
///
/// ## Example Usage
///
/// ### Global Forwarding Rule Http
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = global_forwarding_rule::create(
///         "default",
///         GlobalForwardingRuleArgs::builder()
///             .name("global-rule")
///             .port_range("80")
///             .target("${defaultTargetHttpProxy.id}")
///             .build_struct(),
///     );
///     let defaultBackendService = backend_service::create(
///         "defaultBackendService",
///         BackendServiceArgs::builder()
///             .health_checks("${defaultHttpHealthCheck.id}")
///             .name("backend")
///             .port_name("http")
///             .protocol("HTTP")
///             .timeout_sec(10)
///             .build_struct(),
///     );
///     let defaultHttpHealthCheck = http_health_check::create(
///         "defaultHttpHealthCheck",
///         HttpHealthCheckArgs::builder()
///             .check_interval_sec(1)
///             .name("check-backend")
///             .request_path("/")
///             .timeout_sec(1)
///             .build_struct(),
///     );
///     let defaultTargetHttpProxy = target_http_proxy::create(
///         "defaultTargetHttpProxy",
///         TargetHttpProxyArgs::builder()
///             .description("a description")
///             .name("target-proxy")
///             .url_map("${defaultURLMap.id}")
///             .build_struct(),
///     );
///     let defaultURLMap = url_map::create(
///         "defaultURLMap",
///         UrlMapArgs::builder()
///             .default_service("${defaultBackendService.id}")
///             .description("a description")
///             .host_rules(
///                 vec![
///                     UrlMapHostRule::builder().hosts(vec!["mysite.com",])
///                     .pathMatcher("allpaths").build_struct(),
///                 ],
///             )
///             .name("url-map-target-proxy")
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
/// ### Global Forwarding Rule Internal
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:compute:GlobalForwardingRule
///     properties:
///       name: global-rule
///       target: ${defaultTargetHttpProxy.id}
///       portRange: '80'
///       loadBalancingScheme: INTERNAL_SELF_MANAGED
///       ipAddress: 0.0.0.0
///       metadataFilters:
///         - filterMatchCriteria: MATCH_ANY
///           filterLabels:
///             - name: PLANET
///               value: MARS
///   defaultTargetHttpProxy:
///     type: gcp:compute:TargetHttpProxy
///     name: default
///     properties:
///       name: target-proxy
///       description: a description
///       urlMap: ${defaultURLMap.id}
///   defaultURLMap:
///     type: gcp:compute:URLMap
///     name: default
///     properties:
///       name: url-map-target-proxy
///       description: a description
///       defaultService: ${defaultBackendService.id}
///       hostRules:
///         - hosts:
///             - mysite.com
///           pathMatcher: allpaths
///       pathMatchers:
///         - name: allpaths
///           defaultService: ${defaultBackendService.id}
///           pathRules:
///             - paths:
///                 - /*
///               service: ${defaultBackendService.id}
///   defaultBackendService:
///     type: gcp:compute:BackendService
///     name: default
///     properties:
///       name: backend
///       portName: http
///       protocol: HTTP
///       timeoutSec: 10
///       loadBalancingScheme: INTERNAL_SELF_MANAGED
///       backends:
///         - group: ${igm.instanceGroup}
///           balancingMode: RATE
///           capacityScaler: 0.4
///           maxRatePerInstance: 50
///       healthChecks: ${defaultHealthCheck.id}
///   igm:
///     type: gcp:compute:InstanceGroupManager
///     properties:
///       name: igm-internal
///       versions:
///         - instanceTemplate: ${instanceTemplate.id}
///           name: primary
///       baseInstanceName: internal-glb
///       zone: us-central1-f
///       targetSize: 1
///   instanceTemplate:
///     type: gcp:compute:InstanceTemplate
///     name: instance_template
///     properties:
///       name: template-backend
///       machineType: e2-medium
///       networkInterfaces:
///         - network: default
///       disks:
///         - sourceImage: ${debianImage.selfLink}
///           autoDelete: true
///           boot: true
///   defaultHealthCheck:
///     type: gcp:compute:HealthCheck
///     name: default
///     properties:
///       name: check-backend
///       checkIntervalSec: 1
///       timeoutSec: 1
///       tcpHealthCheck:
///         port: '80'
/// variables:
///   debianImage:
///     fn::invoke:
///       function: gcp:compute:getImage
///       arguments:
///         family: debian-11
///         project: debian-cloud
/// ```
/// ### Global Forwarding Rule External Managed
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = global_forwarding_rule::create(
///         "default",
///         GlobalForwardingRuleArgs::builder()
///             .load_balancing_scheme("EXTERNAL_MANAGED")
///             .name("global-rule")
///             .network_tier("PREMIUM")
///             .port_range("80")
///             .target("${defaultTargetHttpProxy.id}")
///             .build_struct(),
///     );
///     let defaultBackendService = backend_service::create(
///         "defaultBackendService",
///         BackendServiceArgs::builder()
///             .load_balancing_scheme("EXTERNAL_MANAGED")
///             .name("backend")
///             .port_name("http")
///             .protocol("HTTP")
///             .timeout_sec(10)
///             .build_struct(),
///     );
///     let defaultTargetHttpProxy = target_http_proxy::create(
///         "defaultTargetHttpProxy",
///         TargetHttpProxyArgs::builder()
///             .description("a description")
///             .name("target-proxy")
///             .url_map("${defaultURLMap.id}")
///             .build_struct(),
///     );
///     let defaultURLMap = url_map::create(
///         "defaultURLMap",
///         UrlMapArgs::builder()
///             .default_service("${defaultBackendService.id}")
///             .description("a description")
///             .host_rules(
///                 vec![
///                     UrlMapHostRule::builder().hosts(vec!["mysite.com",])
///                     .pathMatcher("allpaths").build_struct(),
///                 ],
///             )
///             .name("url-map-target-proxy")
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
/// ### Global Forwarding Rule Hybrid
///
///
/// ```yaml
/// configuration:
///   # Roughly mirrors https://cloud.google.com/load-balancing/docs/https/setting-up-ext-https-hybrid
///   subnetworkCidr:
///     type: string
///     default: 10.0.0.0/24
/// resources:
///   default:
///     type: gcp:compute:Network
///     properties:
///       name: my-network
///   internal:
///     type: gcp:compute:Network
///     properties:
///       name: my-internal-network
///       autoCreateSubnetworks: false
///   internalSubnetwork:
///     type: gcp:compute:Subnetwork
///     name: internal
///     properties:
///       name: my-subnetwork
///       network: ${internal.id}
///       ipCidrRange: ${subnetworkCidr}
///       region: us-central1
///       privateIpGoogleAccess: true
///   # Zonal NEG with GCE_VM_IP_PORT
///   defaultNetworkEndpointGroup:
///     type: gcp:compute:NetworkEndpointGroup
///     name: default
///     properties:
///       name: default-neg
///       network: ${default.id}
///       defaultPort: '90'
///       zone: us-central1-a
///       networkEndpointType: GCE_VM_IP_PORT
///   # Zonal NEG with GCE_VM_IP
///   internalNetworkEndpointGroup:
///     type: gcp:compute:NetworkEndpointGroup
///     name: internal
///     properties:
///       name: internal-neg
///       network: ${internal.id}
///       subnetwork: ${internalSubnetwork.id}
///       zone: us-central1-a
///       networkEndpointType: GCE_VM_IP
///   # Hybrid connectivity NEG
///   hybrid:
///     type: gcp:compute:NetworkEndpointGroup
///     properties:
///       name: hybrid-neg
///       network: ${default.id}
///       defaultPort: '90'
///       zone: us-central1-a
///       networkEndpointType: NON_GCP_PRIVATE_IP_PORT
///   hybrid-endpoint:
///     type: gcp:compute:NetworkEndpoint
///     properties:
///       networkEndpointGroup: ${hybrid.name}
///       port: ${hybrid.defaultPort}
///       ipAddress: 127.0.0.1
///   # Backend service for Zonal NEG
///   defaultBackendService:
///     type: gcp:compute:BackendService
///     name: default
///     properties:
///       name: backend-default
///       portName: http
///       protocol: HTTP
///       timeoutSec: 10
///       backends:
///         - group: ${defaultNetworkEndpointGroup.id}
///           balancingMode: RATE
///           maxRatePerEndpoint: 10
///       healthChecks: ${defaultHealthCheck.id}
///   # Backgend service for Hybrid NEG
///   hybridBackendService:
///     type: gcp:compute:BackendService
///     name: hybrid
///     properties:
///       name: backend-hybrid
///       portName: http
///       protocol: HTTP
///       timeoutSec: 10
///       backends:
///         - group: ${hybrid.id}
///           balancingMode: RATE
///           maxRatePerEndpoint: 10
///       healthChecks: ${defaultHealthCheck.id}
///   defaultHealthCheck:
///     type: gcp:compute:HealthCheck
///     name: default
///     properties:
///       name: health-check
///       timeoutSec: 1
///       checkIntervalSec: 1
///       tcpHealthCheck:
///         port: '80'
///   defaultURLMap:
///     type: gcp:compute:URLMap
///     name: default
///     properties:
///       name: url-map-target-proxy
///       description: a description
///       defaultService: ${defaultBackendService.id}
///       hostRules:
///         - hosts:
///             - mysite.com
///           pathMatcher: allpaths
///       pathMatchers:
///         - name: allpaths
///           defaultService: ${defaultBackendService.id}
///           pathRules:
///             - paths:
///                 - /*
///               service: ${defaultBackendService.id}
///             - paths:
///                 - /hybrid
///               service: ${hybridBackendService.id}
///   defaultTargetHttpProxy:
///     type: gcp:compute:TargetHttpProxy
///     name: default
///     properties:
///       name: target-proxy
///       description: a description
///       urlMap: ${defaultURLMap.id}
///   defaultGlobalForwardingRule:
///     type: gcp:compute:GlobalForwardingRule
///     name: default
///     properties:
///       name: global-rule
///       target: ${defaultTargetHttpProxy.id}
///       portRange: '80'
/// ```
/// ### Private Service Connect Google Apis
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = global_address::create(
///         "default",
///         GlobalAddressArgs::builder()
///             .address("100.100.100.106")
///             .address_type("INTERNAL")
///             .name("global-psconnect-ip")
///             .network("${network.id}")
///             .project("${network.project}")
///             .purpose("PRIVATE_SERVICE_CONNECT")
///             .build_struct(),
///     );
///     let defaultGlobalForwardingRule = global_forwarding_rule::create(
///         "defaultGlobalForwardingRule",
///         GlobalForwardingRuleArgs::builder()
///             .ip_address("${default.id}")
///             .load_balancing_scheme("")
///             .name("globalrule")
///             .network("${network.id}")
///             .project("${network.project}")
///             .service_directory_registrations(
///                 GlobalForwardingRuleServiceDirectoryRegistrations::builder()
///                     .namespace("sd-namespace")
///                     .serviceDirectoryRegion("europe-west3")
///                     .build_struct(),
///             )
///             .target("all-apis")
///             .build_struct(),
///     );
///     let network = network::create(
///         "network",
///         NetworkArgs::builder()
///             .auto_create_subnetworks(false)
///             .name("my-network")
///             .project("my-project-name")
///             .build_struct(),
///     );
///     let vpcSubnetwork = subnetwork::create(
///         "vpcSubnetwork",
///         SubnetworkArgs::builder()
///             .ip_cidr_range("10.2.0.0/16")
///             .name("my-subnetwork")
///             .network("${network.id}")
///             .private_ip_google_access(true)
///             .project("${network.project}")
///             .region("us-central1")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Private Service Connect Google Apis No Automate Dns
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = global_address::create(
///         "default",
///         GlobalAddressArgs::builder()
///             .address("100.100.100.106")
///             .address_type("INTERNAL")
///             .name("global-psconnect-ip")
///             .network("${network.id}")
///             .project("${network.project}")
///             .purpose("PRIVATE_SERVICE_CONNECT")
///             .build_struct(),
///     );
///     let defaultGlobalForwardingRule = global_forwarding_rule::create(
///         "defaultGlobalForwardingRule",
///         GlobalForwardingRuleArgs::builder()
///             .ip_address("${default.id}")
///             .load_balancing_scheme("")
///             .name("globalrule")
///             .network("${network.id}")
///             .no_automate_dns_zone(false)
///             .project("${network.project}")
///             .target("all-apis")
///             .build_struct(),
///     );
///     let network = network::create(
///         "network",
///         NetworkArgs::builder()
///             .auto_create_subnetworks(false)
///             .name("my-network")
///             .project("my-project-name")
///             .build_struct(),
///     );
///     let vpcSubnetwork = subnetwork::create(
///         "vpcSubnetwork",
///         SubnetworkArgs::builder()
///             .ip_cidr_range("10.2.0.0/16")
///             .name("my-subnetwork")
///             .network("${network.id}")
///             .private_ip_google_access(true)
///             .project("${network.project}")
///             .region("us-central1")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// GlobalForwardingRule can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/global/forwardingRules/{{name}}`
///
/// * `{{project}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, GlobalForwardingRule can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:compute/globalForwardingRule:GlobalForwardingRule default projects/{{project}}/global/forwardingRules/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/globalForwardingRule:GlobalForwardingRule default {{project}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/globalForwardingRule:GlobalForwardingRule default {{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod global_forwarding_rule {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GlobalForwardingRuleArgs {
        /// This is used in PSC consumer ForwardingRule to control whether the PSC endpoint can be accessed from another region.
        #[builder(into, default)]
        pub allow_psc_global_access: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// An optional description of this resource. Provide this property when
        /// you create the resource.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// IP address for which this forwarding rule accepts traffic. When a client
        /// sends traffic to this IP address, the forwarding rule directs the traffic
        /// to the referenced `target`.
        /// While creating a forwarding rule, specifying an `IPAddress` is
        /// required under the following circumstances:
        /// * When the `target` is set to `targetGrpcProxy` and
        /// `validateForProxyless` is set to `true`, the
        /// `IPAddress` should be set to `0.0.0.0`.
        /// * When the `target` is a Private Service Connect Google APIs
        /// bundle, you must specify an `IPAddress`.
        /// Otherwise, you can optionally specify an IP address that references an
        /// existing static (reserved) IP address resource. When omitted, Google Cloud
        /// assigns an ephemeral IP address.
        /// Use one of the following formats to specify an IP address while creating a
        /// forwarding rule:
        /// * IP address number, as in `100.1.2.3`
        /// * IPv6 address range, as in `2600:1234::/96`
        /// * Full resource URL, as in
        /// `https://www.googleapis.com/compute/v1/projects/project_id/regions/region/addresses/address-name`
        /// * Partial URL or by name, as in:
        /// * `projects/project_id/regions/region/addresses/address-name`
        /// * `regions/region/addresses/address-name`
        /// * `global/addresses/address-name`
        /// * `address-name`
        /// The forwarding rule's `target`,
        /// and in most cases, also the `loadBalancingScheme`, determine the
        /// type of IP address that you can use. For detailed information, see
        /// [IP address
        /// specifications](https://cloud.google.com/load-balancing/docs/forwarding-rule-concepts#ip_address_specifications).
        /// When reading an `IPAddress`, the API always returns the IP
        /// address number.
        #[builder(into, default)]
        pub ip_address: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The IP protocol to which this rule applies.
        /// For protocol forwarding, valid
        /// options are `TCP`, `UDP`, `ESP`,
        /// `AH`, `SCTP`, `ICMP` and
        /// `L3_DEFAULT`.
        /// The valid IP protocols are different for different load balancing products
        /// as described in [Load balancing
        /// features](https://cloud.google.com/load-balancing/docs/features#protocols_from_the_load_balancer_to_the_backends).
        /// Possible values are: `TCP`, `UDP`, `ESP`, `AH`, `SCTP`, `ICMP`.
        #[builder(into, default)]
        pub ip_protocol: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The IP Version that will be used by this global forwarding rule.
        /// Possible values are: `IPV4`, `IPV6`.
        #[builder(into, default)]
        pub ip_version: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Labels to apply to this forwarding rule.  A list of key->value pairs.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies the forwarding rule type.
        /// For more information about forwarding rules, refer to
        /// [Forwarding rule concepts](https://cloud.google.com/load-balancing/docs/forwarding-rule-concepts).
        /// Default value is `EXTERNAL`.
        /// Possible values are: `EXTERNAL`, `EXTERNAL_MANAGED`, `INTERNAL_MANAGED`, `INTERNAL_SELF_MANAGED`.
        #[builder(into, default)]
        pub load_balancing_scheme: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Opaque filter criteria used by Loadbalancer to restrict routing
        /// configuration to a limited set xDS compliant clients. In their xDS
        /// requests to Loadbalancer, xDS clients present node metadata. If a
        /// match takes place, the relevant routing configuration is made available
        /// to those proxies.
        /// For each metadataFilter in this list, if its filterMatchCriteria is set
        /// to MATCH_ANY, at least one of the filterLabels must match the
        /// corresponding label provided in the metadata. If its filterMatchCriteria
        /// is set to MATCH_ALL, then all of its filterLabels must match with
        /// corresponding labels in the provided metadata.
        /// metadataFilters specified here can be overridden by those specified in
        /// the UrlMap that this ForwardingRule references.
        /// metadataFilters only applies to Loadbalancers that have their
        /// loadBalancingScheme set to INTERNAL_SELF_MANAGED.
        /// Structure is documented below.
        #[builder(into, default)]
        pub metadata_filters: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::compute::GlobalForwardingRuleMetadataFilter>>,
        >,
        /// Name of the resource; provided by the client when the resource is created.
        /// The name must be 1-63 characters long, and comply with
        /// [RFC1035](https://www.ietf.org/rfc/rfc1035.txt).
        /// Specifically, the name must be 1-63 characters long and match the regular
        /// expression `a-z?` which means the first
        /// character must be a lowercase letter, and all following characters must
        /// be a dash, lowercase letter, or digit, except the last character, which
        /// cannot be a dash.
        /// For Private Service Connect forwarding rules that forward traffic to Google
        /// APIs, the forwarding rule name must be a 1-20 characters string with
        /// lowercase letters and numbers and must start with a letter.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// This field is not used for external load balancing.
        /// For Internal TCP/UDP Load Balancing, this field identifies the network that
        /// the load balanced IP should belong to for this Forwarding Rule.
        /// If the subnetwork is specified, the network of the subnetwork will be used.
        /// If neither subnetwork nor this field is specified, the default network will
        /// be used.
        /// For Private Service Connect forwarding rules that forward traffic to Google
        /// APIs, a network must be provided.
        #[builder(into, default)]
        pub network: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// This signifies the networking tier used for configuring
        /// this load balancer and can only take the following values:
        /// `PREMIUM`, `STANDARD`.
        /// For regional ForwardingRule, the valid values are `PREMIUM` and
        /// `STANDARD`. For GlobalForwardingRule, the valid value is
        /// `PREMIUM`.
        /// If this field is not specified, it is assumed to be `PREMIUM`.
        /// If `IPAddress` is specified, this value must be equal to the
        /// networkTier of the Address.
        /// Possible values are: `PREMIUM`, `STANDARD`.
        #[builder(into, default)]
        pub network_tier: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// This is used in PSC consumer ForwardingRule to control whether it should try to auto-generate a DNS zone or not. Non-PSC forwarding rules do not use this field.
        #[builder(into, default)]
        pub no_automate_dns_zone: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The `portRange` field has the following limitations:
        /// * It requires that the forwarding rule `IPProtocol` be TCP, UDP, or SCTP,
        /// and
        /// * It's applicable only to the following products: external passthrough
        /// Network Load Balancers, internal and external proxy Network Load
        /// Balancers, internal and external Application Load Balancers, external
        /// protocol forwarding, and Classic VPN.
        /// * Some products have restrictions on what ports can be used. See
        /// [port specifications](https://cloud.google.com/load-balancing/docs/forwarding-rule-concepts#port_specifications)
        /// for details.
        /// For external forwarding rules, two or more forwarding rules cannot use the
        /// same `[IPAddress, IPProtocol]` pair, and cannot have overlapping
        /// `portRange`s.
        /// For internal forwarding rules within the same VPC network, two or more
        /// forwarding rules cannot use the same `[IPAddress, IPProtocol]` pair, and
        /// cannot have overlapping `portRange`s.
        /// @pattern: \d+(?:-\d+)?
        #[builder(into, default)]
        pub port_range: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Service Directory resources to register this forwarding rule with.
        /// Currently, only supports a single Service Directory resource.
        /// Structure is documented below.
        #[builder(into, default)]
        pub service_directory_registrations: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::compute::GlobalForwardingRuleServiceDirectoryRegistrations,
            >,
        >,
        /// If not empty, this Forwarding Rule will only forward the traffic when the source IP address matches one of the IP addresses or CIDR ranges set here. Note that a Forwarding Rule can only have up to 64 source IP ranges, and this field can only be used with a regional Forwarding Rule whose scheme is EXTERNAL. Each sourceIpRange entry should be either an IP address (for example, 1.2.3.4) or a CIDR range (for example, 1.2.3.0/24).
        #[builder(into, default)]
        pub source_ip_ranges: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// This field identifies the subnetwork that the load balanced IP should
        /// belong to for this Forwarding Rule, used in internal load balancing and
        /// network load balancing with IPv6.
        /// If the network specified is in auto subnet mode, this field is optional.
        /// However, a subnetwork must be specified if the network is in custom subnet
        /// mode or when creating external forwarding rule with IPv6.
        #[builder(into, default)]
        pub subnetwork: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The URL of the target resource to receive the matched traffic.  For
        /// regional forwarding rules, this target must be in the same region as the
        /// forwarding rule. For global forwarding rules, this target must be a global
        /// load balancing resource.
        /// The forwarded traffic must be of a type appropriate to the target object.
        /// *  For load balancers, see the "Target" column in [Port specifications](https://cloud.google.com/load-balancing/docs/forwarding-rule-concepts#ip_address_specifications).
        /// *  For Private Service Connect forwarding rules that forward traffic to Google APIs, provide the name of a supported Google API bundle:
        /// *  `vpc-sc` - [ APIs that support VPC Service Controls](https://cloud.google.com/vpc-service-controls/docs/supported-products).
        /// *  `all-apis` - [All supported Google APIs](https://cloud.google.com/vpc/docs/private-service-connect#supported-apis).
        /// For Private Service Connect forwarding rules that forward traffic to managed services, the target must be a service attachment.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub target: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GlobalForwardingRuleResult {
        /// This is used in PSC consumer ForwardingRule to control whether the PSC endpoint can be accessed from another region.
        pub allow_psc_global_access: pulumi_gestalt_rust::Output<Option<bool>>,
        /// [Output Only] The URL for the corresponding base Forwarding Rule. By base Forwarding Rule, we mean the Forwarding Rule that has the same IP address, protocol, and port settings with the current Forwarding Rule, but without sourceIPRanges specified. Always empty if the current Forwarding Rule does not have sourceIPRanges specified.
        pub base_forwarding_rule: pulumi_gestalt_rust::Output<String>,
        /// An optional description of this resource. Provide this property when
        /// you create the resource.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The unique identifier number for the resource. This identifier is defined by the server.
        pub forwarding_rule_id: pulumi_gestalt_rust::Output<i32>,
        /// IP address for which this forwarding rule accepts traffic. When a client
        /// sends traffic to this IP address, the forwarding rule directs the traffic
        /// to the referenced `target`.
        /// While creating a forwarding rule, specifying an `IPAddress` is
        /// required under the following circumstances:
        /// * When the `target` is set to `targetGrpcProxy` and
        /// `validateForProxyless` is set to `true`, the
        /// `IPAddress` should be set to `0.0.0.0`.
        /// * When the `target` is a Private Service Connect Google APIs
        /// bundle, you must specify an `IPAddress`.
        /// Otherwise, you can optionally specify an IP address that references an
        /// existing static (reserved) IP address resource. When omitted, Google Cloud
        /// assigns an ephemeral IP address.
        /// Use one of the following formats to specify an IP address while creating a
        /// forwarding rule:
        /// * IP address number, as in `100.1.2.3`
        /// * IPv6 address range, as in `2600:1234::/96`
        /// * Full resource URL, as in
        /// `https://www.googleapis.com/compute/v1/projects/project_id/regions/region/addresses/address-name`
        /// * Partial URL or by name, as in:
        /// * `projects/project_id/regions/region/addresses/address-name`
        /// * `regions/region/addresses/address-name`
        /// * `global/addresses/address-name`
        /// * `address-name`
        /// The forwarding rule's `target`,
        /// and in most cases, also the `loadBalancingScheme`, determine the
        /// type of IP address that you can use. For detailed information, see
        /// [IP address
        /// specifications](https://cloud.google.com/load-balancing/docs/forwarding-rule-concepts#ip_address_specifications).
        /// When reading an `IPAddress`, the API always returns the IP
        /// address number.
        pub ip_address: pulumi_gestalt_rust::Output<String>,
        /// The IP protocol to which this rule applies.
        /// For protocol forwarding, valid
        /// options are `TCP`, `UDP`, `ESP`,
        /// `AH`, `SCTP`, `ICMP` and
        /// `L3_DEFAULT`.
        /// The valid IP protocols are different for different load balancing products
        /// as described in [Load balancing
        /// features](https://cloud.google.com/load-balancing/docs/features#protocols_from_the_load_balancer_to_the_backends).
        /// Possible values are: `TCP`, `UDP`, `ESP`, `AH`, `SCTP`, `ICMP`.
        pub ip_protocol: pulumi_gestalt_rust::Output<String>,
        /// The IP Version that will be used by this global forwarding rule.
        /// Possible values are: `IPV4`, `IPV6`.
        pub ip_version: pulumi_gestalt_rust::Output<Option<String>>,
        /// The fingerprint used for optimistic locking of this resource.  Used
        /// internally during updates.
        pub label_fingerprint: pulumi_gestalt_rust::Output<String>,
        /// Labels to apply to this forwarding rule.  A list of key->value pairs.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies the forwarding rule type.
        /// For more information about forwarding rules, refer to
        /// [Forwarding rule concepts](https://cloud.google.com/load-balancing/docs/forwarding-rule-concepts).
        /// Default value is `EXTERNAL`.
        /// Possible values are: `EXTERNAL`, `EXTERNAL_MANAGED`, `INTERNAL_MANAGED`, `INTERNAL_SELF_MANAGED`.
        pub load_balancing_scheme: pulumi_gestalt_rust::Output<Option<String>>,
        /// Opaque filter criteria used by Loadbalancer to restrict routing
        /// configuration to a limited set xDS compliant clients. In their xDS
        /// requests to Loadbalancer, xDS clients present node metadata. If a
        /// match takes place, the relevant routing configuration is made available
        /// to those proxies.
        /// For each metadataFilter in this list, if its filterMatchCriteria is set
        /// to MATCH_ANY, at least one of the filterLabels must match the
        /// corresponding label provided in the metadata. If its filterMatchCriteria
        /// is set to MATCH_ALL, then all of its filterLabels must match with
        /// corresponding labels in the provided metadata.
        /// metadataFilters specified here can be overridden by those specified in
        /// the UrlMap that this ForwardingRule references.
        /// metadataFilters only applies to Loadbalancers that have their
        /// loadBalancingScheme set to INTERNAL_SELF_MANAGED.
        /// Structure is documented below.
        pub metadata_filters: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::compute::GlobalForwardingRuleMetadataFilter>>,
        >,
        /// Name of the resource; provided by the client when the resource is created.
        /// The name must be 1-63 characters long, and comply with
        /// [RFC1035](https://www.ietf.org/rfc/rfc1035.txt).
        /// Specifically, the name must be 1-63 characters long and match the regular
        /// expression `a-z?` which means the first
        /// character must be a lowercase letter, and all following characters must
        /// be a dash, lowercase letter, or digit, except the last character, which
        /// cannot be a dash.
        /// For Private Service Connect forwarding rules that forward traffic to Google
        /// APIs, the forwarding rule name must be a 1-20 characters string with
        /// lowercase letters and numbers and must start with a letter.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// This field is not used for external load balancing.
        /// For Internal TCP/UDP Load Balancing, this field identifies the network that
        /// the load balanced IP should belong to for this Forwarding Rule.
        /// If the subnetwork is specified, the network of the subnetwork will be used.
        /// If neither subnetwork nor this field is specified, the default network will
        /// be used.
        /// For Private Service Connect forwarding rules that forward traffic to Google
        /// APIs, a network must be provided.
        pub network: pulumi_gestalt_rust::Output<String>,
        /// This signifies the networking tier used for configuring
        /// this load balancer and can only take the following values:
        /// `PREMIUM`, `STANDARD`.
        /// For regional ForwardingRule, the valid values are `PREMIUM` and
        /// `STANDARD`. For GlobalForwardingRule, the valid value is
        /// `PREMIUM`.
        /// If this field is not specified, it is assumed to be `PREMIUM`.
        /// If `IPAddress` is specified, this value must be equal to the
        /// networkTier of the Address.
        /// Possible values are: `PREMIUM`, `STANDARD`.
        pub network_tier: pulumi_gestalt_rust::Output<String>,
        /// This is used in PSC consumer ForwardingRule to control whether it should try to auto-generate a DNS zone or not. Non-PSC forwarding rules do not use this field.
        pub no_automate_dns_zone: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The `portRange` field has the following limitations:
        /// * It requires that the forwarding rule `IPProtocol` be TCP, UDP, or SCTP,
        /// and
        /// * It's applicable only to the following products: external passthrough
        /// Network Load Balancers, internal and external proxy Network Load
        /// Balancers, internal and external Application Load Balancers, external
        /// protocol forwarding, and Classic VPN.
        /// * Some products have restrictions on what ports can be used. See
        /// [port specifications](https://cloud.google.com/load-balancing/docs/forwarding-rule-concepts#port_specifications)
        /// for details.
        /// For external forwarding rules, two or more forwarding rules cannot use the
        /// same `[IPAddress, IPProtocol]` pair, and cannot have overlapping
        /// `portRange`s.
        /// For internal forwarding rules within the same VPC network, two or more
        /// forwarding rules cannot use the same `[IPAddress, IPProtocol]` pair, and
        /// cannot have overlapping `portRange`s.
        /// @pattern: \d+(?:-\d+)?
        pub port_range: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The PSC connection id of the PSC Forwarding Rule.
        pub psc_connection_id: pulumi_gestalt_rust::Output<String>,
        /// The PSC connection status of the PSC Forwarding Rule. Possible values: `STATUS_UNSPECIFIED`, `PENDING`, `ACCEPTED`, `REJECTED`, `CLOSED`
        pub psc_connection_status: pulumi_gestalt_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The URI of the created resource.
        pub self_link: pulumi_gestalt_rust::Output<String>,
        /// Service Directory resources to register this forwarding rule with.
        /// Currently, only supports a single Service Directory resource.
        /// Structure is documented below.
        pub service_directory_registrations: pulumi_gestalt_rust::Output<
            super::super::types::compute::GlobalForwardingRuleServiceDirectoryRegistrations,
        >,
        /// If not empty, this Forwarding Rule will only forward the traffic when the source IP address matches one of the IP addresses or CIDR ranges set here. Note that a Forwarding Rule can only have up to 64 source IP ranges, and this field can only be used with a regional Forwarding Rule whose scheme is EXTERNAL. Each sourceIpRange entry should be either an IP address (for example, 1.2.3.4) or a CIDR range (for example, 1.2.3.0/24).
        pub source_ip_ranges: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// This field identifies the subnetwork that the load balanced IP should
        /// belong to for this Forwarding Rule, used in internal load balancing and
        /// network load balancing with IPv6.
        /// If the network specified is in auto subnet mode, this field is optional.
        /// However, a subnetwork must be specified if the network is in custom subnet
        /// mode or when creating external forwarding rule with IPv6.
        pub subnetwork: pulumi_gestalt_rust::Output<String>,
        /// The URL of the target resource to receive the matched traffic.  For
        /// regional forwarding rules, this target must be in the same region as the
        /// forwarding rule. For global forwarding rules, this target must be a global
        /// load balancing resource.
        /// The forwarded traffic must be of a type appropriate to the target object.
        /// *  For load balancers, see the "Target" column in [Port specifications](https://cloud.google.com/load-balancing/docs/forwarding-rule-concepts#ip_address_specifications).
        /// *  For Private Service Connect forwarding rules that forward traffic to Google APIs, provide the name of a supported Google API bundle:
        /// *  `vpc-sc` - [ APIs that support VPC Service Controls](https://cloud.google.com/vpc-service-controls/docs/supported-products).
        /// *  `all-apis` - [All supported Google APIs](https://cloud.google.com/vpc/docs/private-service-connect#supported-apis).
        /// For Private Service Connect forwarding rules that forward traffic to managed services, the target must be a service attachment.
        ///
        ///
        /// - - -
        pub target: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: GlobalForwardingRuleArgs,
    ) -> GlobalForwardingRuleResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let allow_psc_global_access_binding = args
            .allow_psc_global_access
            .get_output(context);
        let description_binding = args.description.get_output(context);
        let ip_address_binding = args.ip_address.get_output(context);
        let ip_protocol_binding = args.ip_protocol.get_output(context);
        let ip_version_binding = args.ip_version.get_output(context);
        let labels_binding = args.labels.get_output(context);
        let load_balancing_scheme_binding = args
            .load_balancing_scheme
            .get_output(context);
        let metadata_filters_binding = args.metadata_filters.get_output(context);
        let name_binding = args.name.get_output(context);
        let network_binding = args.network.get_output(context);
        let network_tier_binding = args.network_tier.get_output(context);
        let no_automate_dns_zone_binding = args.no_automate_dns_zone.get_output(context);
        let port_range_binding = args.port_range.get_output(context);
        let project_binding = args.project.get_output(context);
        let service_directory_registrations_binding = args
            .service_directory_registrations
            .get_output(context);
        let source_ip_ranges_binding = args.source_ip_ranges.get_output(context);
        let subnetwork_binding = args.subnetwork.get_output(context);
        let target_binding = args.target.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:compute/globalForwardingRule:GlobalForwardingRule".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "allowPscGlobalAccess".into(),
                    value: &allow_psc_global_access_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ipAddress".into(),
                    value: &ip_address_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ipProtocol".into(),
                    value: &ip_protocol_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ipVersion".into(),
                    value: &ip_version_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "labels".into(),
                    value: &labels_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "loadBalancingScheme".into(),
                    value: &load_balancing_scheme_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "metadataFilters".into(),
                    value: &metadata_filters_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "network".into(),
                    value: &network_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "networkTier".into(),
                    value: &network_tier_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "noAutomateDnsZone".into(),
                    value: &no_automate_dns_zone_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "portRange".into(),
                    value: &port_range_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "serviceDirectoryRegistrations".into(),
                    value: &service_directory_registrations_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sourceIpRanges".into(),
                    value: &source_ip_ranges_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "subnetwork".into(),
                    value: &subnetwork_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "target".into(),
                    value: &target_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        GlobalForwardingRuleResult {
            allow_psc_global_access: o.get_field("allowPscGlobalAccess"),
            base_forwarding_rule: o.get_field("baseForwardingRule"),
            description: o.get_field("description"),
            effective_labels: o.get_field("effectiveLabels"),
            forwarding_rule_id: o.get_field("forwardingRuleId"),
            ip_address: o.get_field("ipAddress"),
            ip_protocol: o.get_field("ipProtocol"),
            ip_version: o.get_field("ipVersion"),
            label_fingerprint: o.get_field("labelFingerprint"),
            labels: o.get_field("labels"),
            load_balancing_scheme: o.get_field("loadBalancingScheme"),
            metadata_filters: o.get_field("metadataFilters"),
            name: o.get_field("name"),
            network: o.get_field("network"),
            network_tier: o.get_field("networkTier"),
            no_automate_dns_zone: o.get_field("noAutomateDnsZone"),
            port_range: o.get_field("portRange"),
            project: o.get_field("project"),
            psc_connection_id: o.get_field("pscConnectionId"),
            psc_connection_status: o.get_field("pscConnectionStatus"),
            pulumi_labels: o.get_field("pulumiLabels"),
            self_link: o.get_field("selfLink"),
            service_directory_registrations: o
                .get_field("serviceDirectoryRegistrations"),
            source_ip_ranges: o.get_field("sourceIpRanges"),
            subnetwork: o.get_field("subnetwork"),
            target: o.get_field("target"),
        }
    }
}
