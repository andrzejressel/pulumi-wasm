/// Represents a Route resource.
///
/// A route is a rule that specifies how certain packets should be handled by
/// the virtual network. Routes are associated with virtual machines by tag,
/// and the set of routes for a particular virtual machine is called its
/// routing table. For each packet leaving a virtual machine, the system
/// searches that virtual machine's routing table for a single best matching
/// route.
///
/// Routes match packets by destination IP address, preferring smaller or more
/// specific ranges over larger ones. If there is a tie, the system selects
/// the route with the smallest priority value. If there is still a tie, it
/// uses the layer three and four packet headers to select just one of the
/// remaining matching routes. The packet is then forwarded as specified by
/// the next_hop field of the winning route -- either to another virtual
/// machine destination, a virtual machine gateway or a Compute
/// Engine-operated gateway. Packets that do not match any route in the
/// sending virtual machine's routing table will be dropped.
///
/// A Route resource must have exactly one specification of either
/// nextHopGateway, nextHopInstance, nextHopIp, nextHopVpnTunnel, or
/// nextHopIlb.
///
///
/// To get more information about Route, see:
///
/// * [API documentation](https://cloud.google.com/compute/docs/reference/rest/v1/routes)
/// * How-to Guides
///     * [Using Routes](https://cloud.google.com/vpc/docs/using-routes)
///
/// ## Example Usage
///
/// ### Route Basic
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = route::create(
///         "default",
///         RouteArgs::builder()
///             .dest_range("15.0.0.0/24")
///             .name("network-route")
///             .network("${defaultNetwork.name}")
///             .next_hop_ip("10.132.1.5")
///             .priority(100)
///             .build_struct(),
///     );
///     let defaultNetwork = network::create(
///         "defaultNetwork",
///         NetworkArgs::builder().name("compute-network").build_struct(),
///     );
/// }
/// ```
/// ### Route Ilb
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:compute:Network
///     properties:
///       name: compute-network
///       autoCreateSubnetworks: false
///   defaultSubnetwork:
///     type: gcp:compute:Subnetwork
///     name: default
///     properties:
///       name: compute-subnet
///       ipCidrRange: 10.0.1.0/24
///       region: us-central1
///       network: ${default.id}
///   hc:
///     type: gcp:compute:HealthCheck
///     properties:
///       name: proxy-health-check
///       checkIntervalSec: 1
///       timeoutSec: 1
///       tcpHealthCheck:
///         port: '80'
///   backend:
///     type: gcp:compute:RegionBackendService
///     properties:
///       name: compute-backend
///       region: us-central1
///       healthChecks: ${hc.id}
///   defaultForwardingRule:
///     type: gcp:compute:ForwardingRule
///     name: default
///     properties:
///       name: compute-forwarding-rule
///       region: us-central1
///       loadBalancingScheme: INTERNAL
///       backendService: ${backend.id}
///       allPorts: true
///       network: ${default.name}
///       subnetwork: ${defaultSubnetwork.name}
///   route-ilb:
///     type: gcp:compute:Route
///     properties:
///       name: route-ilb
///       destRange: 0.0.0.0/0
///       network: ${default.name}
///       nextHopIlb: ${defaultForwardingRule.id}
///       priority: 2000
/// ```
/// ### Route Ilb Vip
///
///
/// ```yaml
/// resources:
///   producer:
///     type: gcp:compute:Network
///     properties:
///       name: producer-vpc
///       autoCreateSubnetworks: false
///   producerSubnetwork:
///     type: gcp:compute:Subnetwork
///     name: producer
///     properties:
///       name: producer-subnet
///       ipCidrRange: 10.0.1.0/24
///       region: us-central1
///       network: ${producer.id}
///   consumer:
///     type: gcp:compute:Network
///     properties:
///       name: consumer-vpc
///       autoCreateSubnetworks: false
///   consumerSubnetwork:
///     type: gcp:compute:Subnetwork
///     name: consumer
///     properties:
///       name: consumer-subnet
///       ipCidrRange: 10.0.2.0/24
///       region: us-central1
///       network: ${consumer.id}
///   peering1:
///     type: gcp:compute:NetworkPeering
///     properties:
///       name: peering-producer-to-consumer
///       network: ${consumer.id}
///       peerNetwork: ${producer.id}
///   peering2:
///     type: gcp:compute:NetworkPeering
///     properties:
///       name: peering-consumer-to-producer
///       network: ${producer.id}
///       peerNetwork: ${consumer.id}
///   hc:
///     type: gcp:compute:HealthCheck
///     properties:
///       name: proxy-health-check
///       checkIntervalSec: 1
///       timeoutSec: 1
///       tcpHealthCheck:
///         port: '80'
///   backend:
///     type: gcp:compute:RegionBackendService
///     properties:
///       name: compute-backend
///       region: us-central1
///       healthChecks: ${hc.id}
///   default:
///     type: gcp:compute:ForwardingRule
///     properties:
///       name: compute-forwarding-rule
///       region: us-central1
///       loadBalancingScheme: INTERNAL
///       backendService: ${backend.id}
///       allPorts: true
///       network: ${producer.name}
///       subnetwork: ${producerSubnetwork.name}
///   route-ilb:
///     type: gcp:compute:Route
///     properties:
///       name: route-ilb
///       destRange: 0.0.0.0/0
///       network: ${consumer.name}
///       nextHopIlb: ${default.ipAddress}
///       priority: 2000
///       tags:
///         - tag1
///         - tag2
///     options:
///       dependsOn:
///         - ${peering1}
///         - ${peering2}
/// ```
///
/// ## Import
///
/// Route can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/global/routes/{{name}}`
///
/// * `{{project}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, Route can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:compute/route:Route default projects/{{project}}/global/routes/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/route:Route default {{project}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/route:Route default {{name}}
/// ```
///
pub mod route {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RouteArgs {
        /// An optional description of this resource. Provide this property
        /// when you create the resource.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The destination range of outgoing packets that this route applies to.
        /// Only IPv4 is supported.
        #[builder(into)]
        pub dest_range: pulumi_wasm_rust::Output<String>,
        /// Name of the resource. Provided by the client when the resource is
        /// created. The name must be 1-63 characters long, and comply with
        /// RFC1035.  Specifically, the name must be 1-63 characters long and
        /// match the regular expression `a-z?` which means
        /// the first character must be a lowercase letter, and all following
        /// characters must be a dash, lowercase letter, or digit, except the
        /// last character, which cannot be a dash.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The network that this route applies to.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub network: pulumi_wasm_rust::Output<String>,
        /// URL to a gateway that should handle matching packets.
        /// Currently, you can only specify the internet gateway, using a full or
        /// partial valid URL:
        /// * `https://www.googleapis.com/compute/v1/projects/project/global/gateways/default-internet-gateway`
        /// * `projects/project/global/gateways/default-internet-gateway`
        /// * `global/gateways/default-internet-gateway`
        /// * The string `default-internet-gateway`.
        #[builder(into, default)]
        pub next_hop_gateway: pulumi_wasm_rust::Output<Option<String>>,
        /// The IP address or URL to a forwarding rule of type
        /// loadBalancingScheme=INTERNAL that should handle matching
        /// packets.
        /// With the GA provider you can only specify the forwarding
        /// rule as a partial or full URL. For example, the following
        /// are all valid values:
        /// * 10.128.0.56
        /// * https://www.googleapis.com/compute/v1/projects/project/regions/region/forwardingRules/forwardingRule
        /// * regions/region/forwardingRules/forwardingRule
        /// When the beta provider, you can also specify the IP address
        /// of a forwarding rule from the same VPC or any peered VPC.
        /// Note that this can only be used when the destinationRange is
        /// a public (non-RFC 1918) IP CIDR range.
        #[builder(into, default)]
        pub next_hop_ilb: pulumi_wasm_rust::Output<Option<String>>,
        /// URL to an instance that should handle matching packets.
        /// You can specify this as a full or partial URL. For example:
        /// * `https://www.googleapis.com/compute/v1/projects/project/zones/zone/instances/instance`
        /// * `projects/project/zones/zone/instances/instance`
        /// * `zones/zone/instances/instance`
        /// * Just the instance name, with the zone in `next_hop_instance_zone`.
        #[builder(into, default)]
        pub next_hop_instance: pulumi_wasm_rust::Output<Option<String>>,
        /// (Optional when `next_hop_instance` is
        /// specified)  The zone of the instance specified in
        /// `next_hop_instance`.  Omit if `next_hop_instance` is specified as
        /// a URL.
        #[builder(into, default)]
        pub next_hop_instance_zone: pulumi_wasm_rust::Output<Option<String>>,
        /// Network IP address of an instance that should handle matching packets.
        #[builder(into, default)]
        pub next_hop_ip: pulumi_wasm_rust::Output<Option<String>>,
        /// URL to a VpnTunnel that should handle matching packets.
        #[builder(into, default)]
        pub next_hop_vpn_tunnel: pulumi_wasm_rust::Output<Option<String>>,
        /// The priority of this route. Priority is used to break ties in cases
        /// where there is more than one matching route of equal prefix length.
        /// In the case of two routes with equal prefix length, the one with the
        /// lowest-numbered priority value wins.
        /// Default value is 1000. Valid range is 0 through 65535.
        #[builder(into, default)]
        pub priority: pulumi_wasm_rust::Output<Option<i32>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// A list of instance tags to which this route applies.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    }
    #[allow(dead_code)]
    pub struct RouteResult {
        /// An optional description of this resource. Provide this property
        /// when you create the resource.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The destination range of outgoing packets that this route applies to.
        /// Only IPv4 is supported.
        pub dest_range: pulumi_wasm_rust::Output<String>,
        /// Name of the resource. Provided by the client when the resource is
        /// created. The name must be 1-63 characters long, and comply with
        /// RFC1035.  Specifically, the name must be 1-63 characters long and
        /// match the regular expression `a-z?` which means
        /// the first character must be a lowercase letter, and all following
        /// characters must be a dash, lowercase letter, or digit, except the
        /// last character, which cannot be a dash.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The network that this route applies to.
        ///
        ///
        /// - - -
        pub network: pulumi_wasm_rust::Output<String>,
        /// URL to a gateway that should handle matching packets.
        /// Currently, you can only specify the internet gateway, using a full or
        /// partial valid URL:
        /// * `https://www.googleapis.com/compute/v1/projects/project/global/gateways/default-internet-gateway`
        /// * `projects/project/global/gateways/default-internet-gateway`
        /// * `global/gateways/default-internet-gateway`
        /// * The string `default-internet-gateway`.
        pub next_hop_gateway: pulumi_wasm_rust::Output<Option<String>>,
        /// The IP address or URL to a forwarding rule of type
        /// loadBalancingScheme=INTERNAL that should handle matching
        /// packets.
        /// With the GA provider you can only specify the forwarding
        /// rule as a partial or full URL. For example, the following
        /// are all valid values:
        /// * 10.128.0.56
        /// * https://www.googleapis.com/compute/v1/projects/project/regions/region/forwardingRules/forwardingRule
        /// * regions/region/forwardingRules/forwardingRule
        /// When the beta provider, you can also specify the IP address
        /// of a forwarding rule from the same VPC or any peered VPC.
        /// Note that this can only be used when the destinationRange is
        /// a public (non-RFC 1918) IP CIDR range.
        pub next_hop_ilb: pulumi_wasm_rust::Output<Option<String>>,
        /// URL to an instance that should handle matching packets.
        /// You can specify this as a full or partial URL. For example:
        /// * `https://www.googleapis.com/compute/v1/projects/project/zones/zone/instances/instance`
        /// * `projects/project/zones/zone/instances/instance`
        /// * `zones/zone/instances/instance`
        /// * Just the instance name, with the zone in `next_hop_instance_zone`.
        pub next_hop_instance: pulumi_wasm_rust::Output<Option<String>>,
        /// (Optional when `next_hop_instance` is
        /// specified)  The zone of the instance specified in
        /// `next_hop_instance`.  Omit if `next_hop_instance` is specified as
        /// a URL.
        pub next_hop_instance_zone: pulumi_wasm_rust::Output<String>,
        /// Internal fixed region-to-region cost that Google Cloud calculates based on factors such as network performance,
        /// distance, and available bandwidth between regions.
        pub next_hop_inter_region_cost: pulumi_wasm_rust::Output<String>,
        /// Network IP address of an instance that should handle matching packets.
        pub next_hop_ip: pulumi_wasm_rust::Output<String>,
        /// Multi-Exit Discriminator, a BGP route metric that indicates the desirability of a particular route in a network.
        pub next_hop_med: pulumi_wasm_rust::Output<String>,
        /// URL to a Network that should handle matching packets.
        pub next_hop_network: pulumi_wasm_rust::Output<String>,
        /// Indicates the origin of the route. Can be IGP (Interior Gateway Protocol), EGP (Exterior Gateway Protocol), or
        /// INCOMPLETE.
        pub next_hop_origin: pulumi_wasm_rust::Output<String>,
        /// URL to a VpnTunnel that should handle matching packets.
        pub next_hop_vpn_tunnel: pulumi_wasm_rust::Output<Option<String>>,
        /// The priority of this route. Priority is used to break ties in cases
        /// where there is more than one matching route of equal prefix length.
        /// In the case of two routes with equal prefix length, the one with the
        /// lowest-numbered priority value wins.
        /// Default value is 1000. Valid range is 0 through 65535.
        pub priority: pulumi_wasm_rust::Output<Option<i32>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// The URI of the created resource.
        pub self_link: pulumi_wasm_rust::Output<String>,
        /// A list of instance tags to which this route applies.
        pub tags: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: RouteArgs) -> RouteResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_inner();
        let dest_range_binding = args.dest_range.get_inner();
        let name_binding = args.name.get_inner();
        let network_binding = args.network.get_inner();
        let next_hop_gateway_binding = args.next_hop_gateway.get_inner();
        let next_hop_ilb_binding = args.next_hop_ilb.get_inner();
        let next_hop_instance_binding = args.next_hop_instance.get_inner();
        let next_hop_instance_zone_binding = args.next_hop_instance_zone.get_inner();
        let next_hop_ip_binding = args.next_hop_ip.get_inner();
        let next_hop_vpn_tunnel_binding = args.next_hop_vpn_tunnel.get_inner();
        let priority_binding = args.priority.get_inner();
        let project_binding = args.project.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:compute/route:Route".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "destRange".into(),
                    value: &dest_range_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "network".into(),
                    value: &network_binding,
                },
                register_interface::ObjectField {
                    name: "nextHopGateway".into(),
                    value: &next_hop_gateway_binding,
                },
                register_interface::ObjectField {
                    name: "nextHopIlb".into(),
                    value: &next_hop_ilb_binding,
                },
                register_interface::ObjectField {
                    name: "nextHopInstance".into(),
                    value: &next_hop_instance_binding,
                },
                register_interface::ObjectField {
                    name: "nextHopInstanceZone".into(),
                    value: &next_hop_instance_zone_binding,
                },
                register_interface::ObjectField {
                    name: "nextHopIp".into(),
                    value: &next_hop_ip_binding,
                },
                register_interface::ObjectField {
                    name: "nextHopVpnTunnel".into(),
                    value: &next_hop_vpn_tunnel_binding,
                },
                register_interface::ObjectField {
                    name: "priority".into(),
                    value: &priority_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "destRange".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "network".into(),
                },
                register_interface::ResultField {
                    name: "nextHopGateway".into(),
                },
                register_interface::ResultField {
                    name: "nextHopIlb".into(),
                },
                register_interface::ResultField {
                    name: "nextHopInstance".into(),
                },
                register_interface::ResultField {
                    name: "nextHopInstanceZone".into(),
                },
                register_interface::ResultField {
                    name: "nextHopInterRegionCost".into(),
                },
                register_interface::ResultField {
                    name: "nextHopIp".into(),
                },
                register_interface::ResultField {
                    name: "nextHopMed".into(),
                },
                register_interface::ResultField {
                    name: "nextHopNetwork".into(),
                },
                register_interface::ResultField {
                    name: "nextHopOrigin".into(),
                },
                register_interface::ResultField {
                    name: "nextHopVpnTunnel".into(),
                },
                register_interface::ResultField {
                    name: "priority".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "selfLink".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        RouteResult {
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            dest_range: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("destRange").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            network: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("network").unwrap(),
            ),
            next_hop_gateway: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("nextHopGateway").unwrap(),
            ),
            next_hop_ilb: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("nextHopIlb").unwrap(),
            ),
            next_hop_instance: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("nextHopInstance").unwrap(),
            ),
            next_hop_instance_zone: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("nextHopInstanceZone").unwrap(),
            ),
            next_hop_inter_region_cost: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("nextHopInterRegionCost").unwrap(),
            ),
            next_hop_ip: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("nextHopIp").unwrap(),
            ),
            next_hop_med: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("nextHopMed").unwrap(),
            ),
            next_hop_network: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("nextHopNetwork").unwrap(),
            ),
            next_hop_origin: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("nextHopOrigin").unwrap(),
            ),
            next_hop_vpn_tunnel: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("nextHopVpnTunnel").unwrap(),
            ),
            priority: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("priority").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            self_link: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("selfLink").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
        }
    }
}
