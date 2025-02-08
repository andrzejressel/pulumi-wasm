#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct PeeringConnectionOptionsRequester {
    /// Allow a local VPC to resolve public DNS hostnames to private IP addresses when queried from instances in the peer VPC.
    #[builder(into, default)]
    #[serde(rename = "allowRemoteVpcDnsResolution")]
    pub r#allow_remote_vpc_dns_resolution: Box<Option<bool>>,
}
