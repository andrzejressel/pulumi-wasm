#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsSapoData {
    /// The SAPOData basic authentication credentials.
    #[builder(into, default)]
    #[serde(rename = "basicAuthCredentials")]
    pub r#basic_auth_credentials: Box<Option<super::super::types::appflow::ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsSapoDataBasicAuthCredentials>>,
    /// The SAPOData OAuth type authentication credentials.
    #[builder(into, default)]
    #[serde(rename = "oauthCredentials")]
    pub r#oauth_credentials: Box<Option<super::super::types::appflow::ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsSapoDataOauthCredentials>>,
}
