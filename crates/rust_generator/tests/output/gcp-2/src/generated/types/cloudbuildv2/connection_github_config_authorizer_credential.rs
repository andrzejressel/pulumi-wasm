#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ConnectionGithubConfigAuthorizerCredential {
    /// A SecretManager resource containing the OAuth token that authorizes the Cloud Build connection. Format: `projects/*/secrets/*/versions/*`.
    #[builder(into, default)]
    #[serde(rename = "oauthTokenSecretVersion")]
    pub r#oauth_token_secret_version: Box<Option<String>>,
    /// (Output)
    /// Output only. The username associated to this token.
    #[builder(into, default)]
    #[serde(rename = "username")]
    pub r#username: Box<Option<String>>,
}
