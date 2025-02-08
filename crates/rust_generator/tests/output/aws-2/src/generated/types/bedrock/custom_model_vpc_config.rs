#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct CustomModelVpcConfig {
    /// VPC configuration security group IDs.
    #[builder(into)]
    #[serde(rename = "securityGroupIds")]
    pub r#security_group_ids: Box<Vec<String>>,
    /// VPC configuration subnets.
    #[builder(into)]
    #[serde(rename = "subnetIds")]
    pub r#subnet_ids: Box<Vec<String>>,
}
