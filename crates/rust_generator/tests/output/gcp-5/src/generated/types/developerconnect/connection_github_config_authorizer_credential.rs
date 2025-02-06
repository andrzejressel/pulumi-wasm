#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ConnectionGithubConfigAuthorizerCredential {
    /// Required. A SecretManager resource containing the OAuth token
    /// that authorizes the connection.
    /// Format: `projects/*/secrets/*/versions/*`.
    #[builder(into)]
    #[serde(rename = "oauthTokenSecretVersion")]
    pub r#oauth_token_secret_version: Box<String>,
    /// (Output)
    /// Output only. The username associated with this token.
    #[builder(into, default)]
    #[serde(rename = "username")]
    pub r#username: Box<Option<String>>,
}
