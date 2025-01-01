#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetRouterStatusBestRoute {
    /// An optional description of this resource. Provide this property
    /// when you create the resource.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Box<String>,
    /// The destination range of outgoing packets that this route applies to.
    /// Only IPv4 is supported.
    #[builder(into)]
    #[serde(rename = "destRange")]
    pub r#dest_range: Box<String>,
    /// The name of the router.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The network name or resource link to the parent
    /// network of this subnetwork.
    #[builder(into)]
    #[serde(rename = "network")]
    pub r#network: Box<String>,
    /// URL to a gateway that should handle matching packets.
    /// Currently, you can only specify the internet gateway, using a full or
    /// partial valid URL:
    /// * 'https://www.googleapis.com/compute/v1/projects/project/global/gateways/default-internet-gateway'
    /// * 'projects/project/global/gateways/default-internet-gateway'
    /// * 'global/gateways/default-internet-gateway'
    /// * The string 'default-internet-gateway'.
    #[builder(into)]
    #[serde(rename = "nextHopGateway")]
    pub r#next_hop_gateway: Box<String>,
    /// The IP address or URL to a forwarding rule of type
    /// loadBalancingScheme=INTERNAL that should handle matching
    /// packets.
    /// 
    /// With the GA provider you can only specify the forwarding
    /// rule as a partial or full URL. For example, the following
    /// are all valid values:
    /// * 10.128.0.56
    /// * https://www.googleapis.com/compute/v1/projects/project/regions/region/forwardingRules/forwardingRule
    /// * regions/region/forwardingRules/forwardingRule
    /// 
    /// When the beta provider, you can also specify the IP address
    /// of a forwarding rule from the same VPC or any peered VPC.
    /// 
    /// Note that this can only be used when the destinationRange is
    /// a public (non-RFC 1918) IP CIDR range.
    #[builder(into)]
    #[serde(rename = "nextHopIlb")]
    pub r#next_hop_ilb: Box<String>,
    /// URL to an instance that should handle matching packets.
    /// You can specify this as a full or partial URL. For example:
    /// * 'https://www.googleapis.com/compute/v1/projects/project/zones/zone/instances/instance'
    /// * 'projects/project/zones/zone/instances/instance'
    /// * 'zones/zone/instances/instance'
    /// * Just the instance name, with the zone in 'next_hop_instance_zone'.
    #[builder(into)]
    #[serde(rename = "nextHopInstance")]
    pub r#next_hop_instance: Box<String>,
    /// The zone of the instance specified in next_hop_instance. Omit if next_hop_instance is specified as a URL.
    #[builder(into)]
    #[serde(rename = "nextHopInstanceZone")]
    pub r#next_hop_instance_zone: Box<String>,
    /// Internal fixed region-to-region cost that Google Cloud calculates based on factors such as network performance, distance, and available bandwidth between regions.
    #[builder(into)]
    #[serde(rename = "nextHopInterRegionCost")]
    pub r#next_hop_inter_region_cost: Box<String>,
    /// Network IP address of an instance that should handle matching packets.
    #[builder(into)]
    #[serde(rename = "nextHopIp")]
    pub r#next_hop_ip: Box<String>,
    /// Multi-Exit Discriminator, a BGP route metric that indicates the desirability of a particular route in a network.
    #[builder(into)]
    #[serde(rename = "nextHopMed")]
    pub r#next_hop_med: Box<String>,
    /// URL to a Network that should handle matching packets.
    #[builder(into)]
    #[serde(rename = "nextHopNetwork")]
    pub r#next_hop_network: Box<String>,
    /// Indicates the origin of the route. Can be IGP (Interior Gateway Protocol), EGP (Exterior Gateway Protocol), or INCOMPLETE.
    #[builder(into)]
    #[serde(rename = "nextHopOrigin")]
    pub r#next_hop_origin: Box<String>,
    /// URL to a VpnTunnel that should handle matching packets.
    #[builder(into)]
    #[serde(rename = "nextHopVpnTunnel")]
    pub r#next_hop_vpn_tunnel: Box<String>,
    /// The priority of this route. Priority is used to break ties in cases
    /// where there is more than one matching route of equal prefix length.
    /// 
    /// In the case of two routes with equal prefix length, the one with the
    /// lowest-numbered priority value wins.
    /// 
    /// Default value is 1000. Valid range is 0 through 65535.
    #[builder(into)]
    #[serde(rename = "priority")]
    pub r#priority: Box<i32>,
    /// The ID of the project in which the resource
    /// belongs. If it is not provided, the provider project is used.
    #[builder(into)]
    #[serde(rename = "project")]
    pub r#project: Box<String>,
    #[builder(into)]
    #[serde(rename = "selfLink")]
    pub r#self_link: Box<String>,
    /// A list of instance tags to which this route applies.
    #[builder(into)]
    #[serde(rename = "tags")]
    pub r#tags: Box<Vec<String>>,
}
