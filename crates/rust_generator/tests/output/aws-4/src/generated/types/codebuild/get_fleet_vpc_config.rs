#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetFleetVpcConfig {
    /// A list of one or more security groups IDs in your Amazon VPC.
    #[builder(into)]
    #[serde(rename = "securityGroupIds")]
    pub r#security_group_ids: Box<Vec<String>>,
    /// A list of one or more subnet IDs in your Amazon VPC.
    #[builder(into)]
    #[serde(rename = "subnets")]
    pub r#subnets: Box<Vec<String>>,
    /// The ID of the Amazon VPC.
    #[builder(into)]
    #[serde(rename = "vpcId")]
    pub r#vpc_id: Box<String>,
}
