#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct WorkforceCognitoConfig {
    /// The client ID for your Amazon Cognito user pool.
    #[builder(into)]
    #[serde(rename = "clientId")]
    pub r#client_id: Box<String>,
    /// ID for your Amazon Cognito user pool.
    #[builder(into)]
    #[serde(rename = "userPool")]
    pub r#user_pool: Box<String>,
}