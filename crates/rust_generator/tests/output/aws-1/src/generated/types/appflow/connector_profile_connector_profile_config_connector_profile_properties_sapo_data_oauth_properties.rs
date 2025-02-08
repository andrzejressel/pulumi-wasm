#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesSapoDataOauthProperties {
    /// The authorization code url required to redirect to SAP Login Page to fetch authorization code for OAuth type authentication.
    #[builder(into)]
    #[serde(rename = "authCodeUrl")]
    pub r#auth_code_url: Box<String>,
    /// The OAuth scopes required for OAuth type authentication.
    #[builder(into)]
    #[serde(rename = "oauthScopes")]
    pub r#oauth_scopes: Box<Vec<String>>,
    #[builder(into)]
    #[serde(rename = "tokenUrl")]
    pub r#token_url: Box<String>,
}
