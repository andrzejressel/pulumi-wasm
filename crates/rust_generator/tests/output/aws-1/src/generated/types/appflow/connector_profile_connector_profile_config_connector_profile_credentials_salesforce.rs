#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsSalesforce {
    #[builder(into, default)]
    #[serde(rename = "accessToken")]
    pub r#access_token: Box<Option<String>>,
    /// The secret manager ARN, which contains the client ID and client secret of the connected app.
    #[builder(into, default)]
    #[serde(rename = "clientCredentialsArn")]
    pub r#client_credentials_arn: Box<Option<String>>,
    /// A JSON web token (JWT) that authorizes access to Salesforce records.
    #[builder(into, default)]
    #[serde(rename = "jwtToken")]
    pub r#jwt_token: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "oauth2GrantType")]
    pub r#oauth_2_grant_type: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "oauthRequest")]
    pub r#oauth_request: Box<Option<super::super::types::appflow::ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsSalesforceOauthRequest>>,
    #[builder(into, default)]
    #[serde(rename = "refreshToken")]
    pub r#refresh_token: Box<Option<String>>,
}
