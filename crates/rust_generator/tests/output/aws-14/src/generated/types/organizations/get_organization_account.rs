#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetOrganizationAccount {
    /// ARN of the root
    #[builder(into)]
    #[serde(rename = "arn")]
    pub r#arn: Box<String>,
    /// Email of the account
    #[builder(into)]
    #[serde(rename = "email")]
    pub r#email: Box<String>,
    /// Identifier of the root
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Box<String>,
    /// The name of the policy type
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The status of the policy type as it relates to the associated root
    #[builder(into)]
    #[serde(rename = "status")]
    pub r#status: Box<String>,
}
