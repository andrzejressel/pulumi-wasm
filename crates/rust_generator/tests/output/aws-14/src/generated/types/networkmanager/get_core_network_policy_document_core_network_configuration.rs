#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetCoreNetworkPolicyDocumentCoreNetworkConfiguration {
    /// List of strings containing Autonomous System Numbers (ASNs) to assign to Core Network Edges. By default, the core network automatically assigns an ASN for each Core Network Edge but you can optionally define the ASN in the edge-locations for each Region. The ASN uses an array of integer ranges only from `64512` to `65534` and `4200000000` to `4294967294` expressed as a string like `"64512-65534"`. No other ASN ranges can be used.
    #[builder(into)]
    #[serde(rename = "asnRanges")]
    pub r#asn_ranges: Box<Vec<String>>,
    /// A block value of AWS Region locations where you're creating Core Network Edges. Detailed below.
    #[builder(into)]
    #[serde(rename = "edgeLocations")]
    pub r#edge_locations: Box<Vec<super::super::types::networkmanager::GetCoreNetworkPolicyDocumentCoreNetworkConfigurationEdgeLocation>>,
    /// The Classless Inter-Domain Routing (CIDR) block range used to create tunnels for AWS Transit Gateway Connect. The format is standard AWS CIDR range (for example, `10.0.1.0/24`). You can optionally define the inside CIDR in the Core Network Edges section per Region. The minimum is a `/24` for IPv4 or `/64` for IPv6. You can provide multiple `/24` subnets or a larger CIDR range. If you define a larger CIDR range, new Core Network Edges will be automatically assigned `/24` and `/64` subnets from the larger CIDR. an Inside CIDR block is required for attaching Connect attachments to a Core Network Edge.
    #[builder(into, default)]
    #[serde(rename = "insideCidrBlocks")]
    pub r#inside_cidr_blocks: Box<Option<Vec<String>>>,
    /// Indicates whether the core network forwards traffic over multiple equal-cost routes using VPN. The value can be either `true` or `false`. The default is `true`.
    #[builder(into, default)]
    #[serde(rename = "vpnEcmpSupport")]
    pub r#vpn_ecmp_support: Box<Option<bool>>,
}
