#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ConnectionAuthConfigOauth2AuthCodeFlow {
    /// Auth URL for Authorization Code Flow.
    #[builder(into, default)]
    #[serde(rename = "authUri")]
    pub r#auth_uri: Box<Option<String>>,
    /// Client ID for user-provided OAuth app.
    #[builder(into, default)]
    #[serde(rename = "clientId")]
    pub r#client_id: Box<Option<String>>,
    /// Client secret for user-provided OAuth app.
    #[builder(into, default)]
    #[serde(rename = "clientSecret")]
    pub r#client_secret: Box<Option<super::super::types::integrationconnectors::ConnectionAuthConfigOauth2AuthCodeFlowClientSecret>>,
    /// Whether to enable PKCE when the user performs the auth code flow.
    #[builder(into, default)]
    #[serde(rename = "enablePkce")]
    pub r#enable_pkce: Box<Option<bool>>,
    /// Scopes the connection will request when the user performs the auth code flow.
    #[builder(into, default)]
    #[serde(rename = "scopes")]
    pub r#scopes: Box<Option<Vec<String>>>,
}
