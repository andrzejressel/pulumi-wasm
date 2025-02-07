#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AssessmentRolesAll {
    /// Amazon Resource Name (ARN) of the IAM role.
    #[builder(into)]
    #[serde(rename = "roleArn")]
    pub r#role_arn: Box<String>,
    /// Type of customer persona. For assessment creation, type must always be `PROCESS_OWNER`.
    #[builder(into)]
    #[serde(rename = "roleType")]
    pub r#role_type: Box<String>,
}
