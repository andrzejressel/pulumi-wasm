#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct IdentityPoolCognitoIdentityProvider {
    /// The client ID for the Amazon Cognito Identity User Pool.
    #[builder(into, default)]
    #[serde(rename = "clientId")]
    pub r#client_id: Box<Option<String>>,
    /// The provider name for an Amazon Cognito Identity User Pool.
    #[builder(into, default)]
    #[serde(rename = "providerName")]
    pub r#provider_name: Box<Option<String>>,
    /// Whether server-side token validation is enabled for the identity provider’s token or not.
    #[builder(into, default)]
    #[serde(rename = "serverSideTokenCheck")]
    pub r#server_side_token_check: Box<Option<bool>>,
}