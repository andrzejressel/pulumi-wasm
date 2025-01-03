#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetOrganizationalUnitDescendantAccountsAccount {
    /// The Amazon Resource Name (ARN) of the account.
    #[builder(into)]
    #[serde(rename = "arn")]
    pub r#arn: Box<String>,
    /// The email address associated with the AWS account.
    #[builder(into)]
    #[serde(rename = "email")]
    pub r#email: Box<String>,
    /// Parent identifier of the organizational units.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Box<String>,
    /// The friendly name of the account.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The status of the account in the organization.
    #[builder(into)]
    #[serde(rename = "status")]
    pub r#status: Box<String>,
}
