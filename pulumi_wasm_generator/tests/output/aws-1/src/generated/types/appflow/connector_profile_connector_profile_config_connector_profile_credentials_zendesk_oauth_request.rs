#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsZendeskOauthRequest {
    /// The code provided by the connector when it has been authenticated via the connected app.
    #[builder(into, default)]
    #[serde(rename = "authCode")]
    pub r#auth_code: Box<Option<String>>,
    /// The URL to which the authentication server redirects the browser after authorization has been granted.
    #[builder(into, default)]
    #[serde(rename = "redirectUri")]
    pub r#redirect_uri: Box<Option<String>>,
}
