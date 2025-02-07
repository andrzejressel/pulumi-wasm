#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ApiOpenidAuthentication {
    /// How to send token to the server. A list of zero or more methods. Valid values are `authorizationHeader` and `query`.
    #[builder(into, default)]
    #[serde(rename = "bearerTokenSendingMethods")]
    pub r#bearer_token_sending_methods: Box<Option<Vec<String>>>,
    /// OpenID Connect provider identifier. The name of an OpenID Connect Provider.
    #[builder(into)]
    #[serde(rename = "openidProviderName")]
    pub r#openid_provider_name: Box<String>,
}
