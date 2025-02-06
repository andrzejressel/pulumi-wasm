#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsHoneycode {
    #[builder(into, default)]
    #[serde(rename = "accessToken")]
    pub r#access_token: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "oauthRequest")]
    pub r#oauth_request: Box<Option<super::super::types::appflow::ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsHoneycodeOauthRequest>>,
    #[builder(into, default)]
    #[serde(rename = "refreshToken")]
    pub r#refresh_token: Box<Option<String>>,
}
