#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ApiOauth2Authorization {
    /// OAuth authorization server identifier. The name of an OAuth2 Authorization Server.
    #[builder(into)]
    #[serde(rename = "authorizationServerName")]
    pub r#authorization_server_name: Box<String>,
    /// Operations scope.
    #[builder(into, default)]
    #[serde(rename = "scope")]
    pub r#scope: Box<Option<String>>,
}
