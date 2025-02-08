#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ResolverEndpointIpAddress {
    /// IPv4 address in the subnet that you want to use for DNS queries.
    #[builder(into, default)]
    #[serde(rename = "ip")]
    pub r#ip: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "ipId")]
    pub r#ip_id: Box<Option<String>>,
    /// IPv6 address in the subnet that you want to use for DNS queries.
    #[builder(into, default)]
    #[serde(rename = "ipv6")]
    pub r#ipv_6: Box<Option<String>>,
    /// ID of the subnet that contains the IP address.
    #[builder(into)]
    #[serde(rename = "subnetId")]
    pub r#subnet_id: Box<String>,
}
