#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AuthConfigDecryptedCredential {
    /// Auth token credential.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "authToken")]
    pub r#auth_token: Box<Option<super::super::types::applicationintegration::AuthConfigDecryptedCredentialAuthToken>>,
    /// Credential type associated with auth configs.
    #[builder(into)]
    #[serde(rename = "credentialType")]
    pub r#credential_type: Box<String>,
    /// JWT credential.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "jwt")]
    pub r#jwt: Box<Option<super::super::types::applicationintegration::AuthConfigDecryptedCredentialJwt>>,
    /// OAuth2 authorization code credential.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "oauth2AuthorizationCode")]
    pub r#oauth_2_authorization_code: Box<Option<super::super::types::applicationintegration::AuthConfigDecryptedCredentialOauth2AuthorizationCode>>,
    /// OAuth2 client credentials.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "oauth2ClientCredentials")]
    pub r#oauth_2_client_credentials: Box<Option<super::super::types::applicationintegration::AuthConfigDecryptedCredentialOauth2ClientCredentials>>,
    /// Google OIDC ID Token.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "oidcToken")]
    pub r#oidc_token: Box<Option<super::super::types::applicationintegration::AuthConfigDecryptedCredentialOidcToken>>,
    /// Service account credential.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "serviceAccountCredentials")]
    pub r#service_account_credentials: Box<Option<super::super::types::applicationintegration::AuthConfigDecryptedCredentialServiceAccountCredentials>>,
    /// Username and password credential.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "usernameAndPassword")]
    pub r#username_and_password: Box<Option<super::super::types::applicationintegration::AuthConfigDecryptedCredentialUsernameAndPassword>>,
}
