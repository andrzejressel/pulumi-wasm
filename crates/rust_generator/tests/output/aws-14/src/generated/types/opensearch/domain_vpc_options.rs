#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DomainVpcOptions {
    /// If the domain was created inside a VPC, the names of the availability zones the configured `subnet_ids` were created inside.
    #[builder(into, default)]
    #[serde(rename = "availabilityZones")]
    pub r#availability_zones: Box<Option<Vec<String>>>,
    /// List of VPC Security Group IDs to be applied to the OpenSearch domain endpoints. If omitted, the default Security Group for the VPC will be used.
    #[builder(into, default)]
    #[serde(rename = "securityGroupIds")]
    pub r#security_group_ids: Box<Option<Vec<String>>>,
    /// List of VPC Subnet IDs for the OpenSearch domain endpoints to be created in.
    #[builder(into, default)]
    #[serde(rename = "subnetIds")]
    pub r#subnet_ids: Box<Option<Vec<String>>>,
    /// If the domain was created inside a VPC, the ID of the VPC.
    #[builder(into, default)]
    #[serde(rename = "vpcId")]
    pub r#vpc_id: Box<Option<String>>,
}
