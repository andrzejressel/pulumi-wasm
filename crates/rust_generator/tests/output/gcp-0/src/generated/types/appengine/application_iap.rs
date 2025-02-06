#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ApplicationIap {
    /// (Optional) Whether the serving infrastructure will authenticate and authorize all incoming requests. 
    /// (default is false)
    #[builder(into, default)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
    /// OAuth2 client ID to use for the authentication flow.
    #[builder(into)]
    #[serde(rename = "oauth2ClientId")]
    pub r#oauth_2_client_id: Box<String>,
    /// OAuth2 client secret to use for the authentication flow.
    /// The SHA-256 hash of the value is returned in the oauth2ClientSecretSha256 field.
    #[builder(into)]
    #[serde(rename = "oauth2ClientSecret")]
    pub r#oauth_2_client_secret: Box<String>,
    /// Hex-encoded SHA-256 hash of the client secret.
    #[builder(into, default)]
    #[serde(rename = "oauth2ClientSecretSha256")]
    pub r#oauth_2_client_secret_sha_256: Box<Option<String>>,
}
