#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetLoadBalancerSubnetMapping {
    #[builder(into)]
    #[serde(rename = "allocationId")]
    pub r#allocation_id: Box<String>,
    #[builder(into)]
    #[serde(rename = "ipv6Address")]
    pub r#ipv_6_address: Box<String>,
    #[builder(into)]
    #[serde(rename = "outpostId")]
    pub r#outpost_id: Box<String>,
    #[builder(into)]
    #[serde(rename = "privateIpv4Address")]
    pub r#private_ipv_4_address: Box<String>,
    #[builder(into)]
    #[serde(rename = "subnetId")]
    pub r#subnet_id: Box<String>,
}
