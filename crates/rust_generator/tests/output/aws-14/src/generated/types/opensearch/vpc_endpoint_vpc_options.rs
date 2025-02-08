#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct VpcEndpointVpcOptions {
    #[builder(into, default)]
    #[serde(rename = "availabilityZones")]
    pub r#availability_zones: Box<Option<Vec<String>>>,
    /// The list of security group IDs associated with the VPC endpoints for the domain. If you do not provide a security group ID, OpenSearch Service uses the default security group for the VPC.
    #[builder(into, default)]
    #[serde(rename = "securityGroupIds")]
    pub r#security_group_ids: Box<Option<Vec<String>>>,
    /// A list of subnet IDs associated with the VPC endpoints for the domain. If your domain uses multiple Availability Zones, you need to provide two subnet IDs, one per zone. Otherwise, provide only one.
    #[builder(into)]
    #[serde(rename = "subnetIds")]
    pub r#subnet_ids: Box<Vec<String>>,
    #[builder(into, default)]
    #[serde(rename = "vpcId")]
    pub r#vpc_id: Box<Option<String>>,
}
