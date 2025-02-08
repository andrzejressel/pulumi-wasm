#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct InputVpc {
    /// A list of up to 5 EC2 VPC security group IDs to attach to the Input.
    #[builder(into, default)]
    #[serde(rename = "securityGroupIds")]
    pub r#security_group_ids: Box<Option<Vec<String>>>,
    /// A list of 2 VPC subnet IDs from the same VPC.
    #[builder(into)]
    #[serde(rename = "subnetIds")]
    pub r#subnet_ids: Box<Vec<String>>,
}
