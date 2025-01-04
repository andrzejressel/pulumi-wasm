#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct OrganizationNonMasterAccount {
    /// ARN of the root
    #[builder(into, default)]
    #[serde(rename = "arn")]
    pub r#arn: Box<Option<String>>,
    /// Email of the account
    #[builder(into, default)]
    #[serde(rename = "email")]
    pub r#email: Box<Option<String>>,
    /// Identifier of the root
    #[builder(into, default)]
    #[serde(rename = "id")]
    pub r#id: Box<Option<String>>,
    /// The name of the policy type
    #[builder(into, default)]
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    /// The status of the policy type as it relates to the associated root
    #[builder(into, default)]
    #[serde(rename = "status")]
    pub r#status: Box<Option<String>>,
}
