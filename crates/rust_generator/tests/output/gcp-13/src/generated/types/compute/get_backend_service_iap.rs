#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetBackendServiceIap {
    /// Whether the serving infrastructure will authenticate and authorize all incoming requests.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<bool>,
    /// OAuth2 Client ID for IAP
    #[builder(into)]
    #[serde(rename = "oauth2ClientId")]
    pub r#oauth_2_client_id: Box<String>,
    /// OAuth2 Client Secret for IAP
    #[builder(into)]
    #[serde(rename = "oauth2ClientSecret")]
    pub r#oauth_2_client_secret: Box<String>,
    /// OAuth2 Client Secret SHA-256 for IAP
    #[builder(into)]
    #[serde(rename = "oauth2ClientSecretSha256")]
    pub r#oauth_2_client_secret_sha_256: Box<String>,
}
