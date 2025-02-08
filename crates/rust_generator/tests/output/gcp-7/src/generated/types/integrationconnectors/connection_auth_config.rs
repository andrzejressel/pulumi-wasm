#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ConnectionAuthConfig {
    /// List containing additional auth configs.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "additionalVariables")]
    pub r#additional_variables: Box<Option<Vec<super::super::types::integrationconnectors::ConnectionAuthConfigAdditionalVariable>>>,
    /// The type of authentication configured.
    #[builder(into, default)]
    #[serde(rename = "authKey")]
    pub r#auth_key: Box<Option<String>>,
    /// authType of the Connection
    /// Possible values are: `USER_PASSWORD`.
    #[builder(into)]
    #[serde(rename = "authType")]
    pub r#auth_type: Box<String>,
    /// Parameters to support Oauth 2.0 Auth Code Grant Authentication.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "oauth2AuthCodeFlow")]
    pub r#oauth_2_auth_code_flow: Box<Option<super::super::types::integrationconnectors::ConnectionAuthConfigOauth2AuthCodeFlow>>,
    /// OAuth3 Client Credentials for Authentication.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "oauth2ClientCredentials")]
    pub r#oauth_2_client_credentials: Box<Option<super::super::types::integrationconnectors::ConnectionAuthConfigOauth2ClientCredentials>>,
    /// OAuth2 JWT Bearer for Authentication.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "oauth2JwtBearer")]
    pub r#oauth_2_jwt_bearer: Box<Option<super::super::types::integrationconnectors::ConnectionAuthConfigOauth2JwtBearer>>,
    /// SSH Public Key for Authentication.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "sshPublicKey")]
    pub r#ssh_public_key: Box<Option<super::super::types::integrationconnectors::ConnectionAuthConfigSshPublicKey>>,
    /// User password for Authentication.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "userPassword")]
    pub r#user_password: Box<Option<super::super::types::integrationconnectors::ConnectionAuthConfigUserPassword>>,
}
