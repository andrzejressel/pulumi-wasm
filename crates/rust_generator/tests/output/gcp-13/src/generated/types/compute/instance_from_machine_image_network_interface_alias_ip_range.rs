#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct InstanceFromMachineImageNetworkInterfaceAliasIpRange {
    /// The IP CIDR range represented by this alias IP range.
    #[builder(into)]
    #[serde(rename = "ipCidrRange")]
    pub r#ip_cidr_range: Box<String>,
    /// The subnetwork secondary range name specifying the secondary range from which to allocate the IP CIDR range for this alias IP range.
    #[builder(into, default)]
    #[serde(rename = "subnetworkRangeName")]
    pub r#subnetwork_range_name: Box<Option<String>>,
}
