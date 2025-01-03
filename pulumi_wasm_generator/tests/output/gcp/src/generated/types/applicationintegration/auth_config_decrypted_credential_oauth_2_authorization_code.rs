#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AuthConfigDecryptedCredentialOauth2AuthorizationCode {
    /// The auth url endpoint to send the auth code request to.
    #[builder(into, default)]
    #[serde(rename = "authEndpoint")]
    pub r#auth_endpoint: Box<Option<String>>,
    /// The client's id.
    #[builder(into, default)]
    #[serde(rename = "clientId")]
    pub r#client_id: Box<Option<String>>,
    /// The client's secret.
    #[builder(into, default)]
    #[serde(rename = "clientSecret")]
    pub r#client_secret: Box<Option<String>>,
    /// A space-delimited list of requested scope permissions.
    #[builder(into, default)]
    #[serde(rename = "scope")]
    pub r#scope: Box<Option<String>>,
    /// The token url endpoint to send the token request to.
    #[builder(into, default)]
    #[serde(rename = "tokenEndpoint")]
    pub r#token_endpoint: Box<Option<String>>,
}
