#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetPrincipalApplicationAssignmentsApplicationAssignment {
    /// ARN of the application.
    #[builder(into)]
    #[serde(rename = "applicationArn")]
    pub r#application_arn: Box<String>,
    /// An identifier for an object in IAM Identity Center, such as a user or group.
    #[builder(into)]
    #[serde(rename = "principalId")]
    pub r#principal_id: Box<String>,
    /// Entity type for which the assignment will be created. Valid values are `USER` or `GROUP`.
    #[builder(into)]
    #[serde(rename = "principalType")]
    pub r#principal_type: Box<String>,
}