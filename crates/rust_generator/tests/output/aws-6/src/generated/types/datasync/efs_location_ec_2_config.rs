#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct EfsLocationEc2Config {
    /// List of Amazon Resource Names (ARNs) of the EC2 Security Groups that are associated with the EFS Mount Target.
    #[builder(into)]
    #[serde(rename = "securityGroupArns")]
    pub r#security_group_arns: Box<Vec<String>>,
    /// Amazon Resource Name (ARN) of the EC2 Subnet that is associated with the EFS Mount Target.
    #[builder(into)]
    #[serde(rename = "subnetArn")]
    pub r#subnet_arn: Box<String>,
}
