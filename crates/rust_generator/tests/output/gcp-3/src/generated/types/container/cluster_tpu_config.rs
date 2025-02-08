#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ClusterTpuConfig {
    /// Whether Cloud TPU integration is enabled or not
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<bool>,
    /// IPv4 CIDR block reserved for Cloud TPU in the VPC.
    #[builder(into, default)]
    #[serde(rename = "ipv4CidrBlock")]
    pub r#ipv_4_cidr_block: Box<Option<String>>,
    /// Whether to use service networking for Cloud TPU or not
    #[builder(into, default)]
    #[serde(rename = "useServiceNetworking")]
    pub r#use_service_networking: Box<Option<bool>>,
}
