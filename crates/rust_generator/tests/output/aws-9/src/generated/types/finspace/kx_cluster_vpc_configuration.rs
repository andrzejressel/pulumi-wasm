#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct KxClusterVpcConfiguration {
    /// IP address type for cluster network configuration parameters. The following type is available: IP_V4 - IP address version 4.
    #[builder(into)]
    #[serde(rename = "ipAddressType")]
    pub r#ip_address_type: Box<String>,
    /// Unique identifier of the VPC security group applied to the VPC endpoint ENI for the cluster.
    /// * `subnet_ids `- (Required) Identifier of the subnet that the Privatelink VPC endpoint uses to connect to the cluster.
    #[builder(into)]
    #[serde(rename = "securityGroupIds")]
    pub r#security_group_ids: Box<Vec<String>>,
    #[builder(into)]
    #[serde(rename = "subnetIds")]
    pub r#subnet_ids: Box<Vec<String>>,
    /// Identifier of the VPC endpoint
    #[builder(into)]
    #[serde(rename = "vpcId")]
    pub r#vpc_id: Box<String>,
}
