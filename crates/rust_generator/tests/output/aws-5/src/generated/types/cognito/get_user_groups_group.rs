#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetUserGroupsGroup {
    /// Description of the user group.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Box<String>,
    /// Name of the user group.
    #[builder(into)]
    #[serde(rename = "groupName")]
    pub r#group_name: Box<String>,
    /// Precedence of the user group.
    #[builder(into)]
    #[serde(rename = "precedence")]
    pub r#precedence: Box<i32>,
    /// ARN of the IAM role to be associated with the user group.
    #[builder(into)]
    #[serde(rename = "roleArn")]
    pub r#role_arn: Box<String>,
}
