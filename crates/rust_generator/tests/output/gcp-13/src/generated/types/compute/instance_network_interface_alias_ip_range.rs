#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct InstanceNetworkInterfaceAliasIpRange {
    /// The IP CIDR range represented by this alias IP range. This IP CIDR range
    /// must belong to the specified subnetwork and cannot contain IP addresses reserved by
    /// system or used by other network interfaces. This range may be a single IP address
    /// (e.g. 10.2.3.4), a netmask (e.g. /24) or a CIDR format string (e.g. 10.1.2.0/24).
    #[builder(into)]
    #[serde(rename = "ipCidrRange")]
    pub r#ip_cidr_range: Box<String>,
    /// The subnetwork secondary range name specifying
    /// the secondary range from which to allocate the IP CIDR range for this alias IP
    /// range. If left unspecified, the primary range of the subnetwork will be used.
    #[builder(into, default)]
    #[serde(rename = "subnetworkRangeName")]
    pub r#subnetwork_range_name: Box<Option<String>>,
}
