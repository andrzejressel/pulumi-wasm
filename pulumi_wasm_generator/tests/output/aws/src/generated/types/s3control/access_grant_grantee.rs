#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AccessGrantGrantee {
    /// Grantee identifier.
    #[builder(into)]
    #[serde(rename = "granteeIdentifier")]
    pub r#grantee_identifier: Box<String>,
    /// Grantee types. Valid values: `DIRECTORY_USER`, `DIRECTORY_GROUP`, `IAM`.
    #[builder(into)]
    #[serde(rename = "granteeType")]
    pub r#grantee_type: Box<String>,
}