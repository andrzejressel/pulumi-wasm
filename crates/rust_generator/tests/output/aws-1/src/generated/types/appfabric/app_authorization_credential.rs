#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AppAuthorizationCredential {
    /// Contains API key credential information.
    #[builder(into, default)]
    #[serde(rename = "apiKeyCredentials")]
    pub r#api_key_credentials: Box<Option<Vec<super::super::types::appfabric::AppAuthorizationCredentialApiKeyCredential>>>,
    /// Contains OAuth2 client credential information.
    #[builder(into, default)]
    #[serde(rename = "oauth2Credential")]
    pub r#oauth_2_credential: Box<Option<super::super::types::appfabric::AppAuthorizationCredentialOauth2Credential>>,
}
