#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetSubnetworkSecondaryIpRange {
    /// The range of IP addresses belonging to this subnetwork
    /// secondary range.
    #[builder(into)]
    #[serde(rename = "ipCidrRange")]
    pub r#ip_cidr_range: Box<String>,
    /// The name associated with this subnetwork secondary range, used
    /// when adding an alias IP range to a VM instance.
    #[builder(into)]
    #[serde(rename = "rangeName")]
    pub r#range_name: Box<String>,
}
