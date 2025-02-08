#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ListenerRuleActionAuthenticateOidc {
    /// The query parameters to include in the redirect request to the authorization endpoint. Max: 10.
    #[builder(into, default)]
    #[serde(rename = "authenticationRequestExtraParams")]
    pub r#authentication_request_extra_params: Box<Option<std::collections::HashMap<String, String>>>,
    /// The authorization endpoint of the IdP.
    #[builder(into)]
    #[serde(rename = "authorizationEndpoint")]
    pub r#authorization_endpoint: Box<String>,
    /// The OAuth 2.0 client identifier.
    #[builder(into)]
    #[serde(rename = "clientId")]
    pub r#client_id: Box<String>,
    /// The OAuth 2.0 client secret.
    #[builder(into)]
    #[serde(rename = "clientSecret")]
    pub r#client_secret: Box<String>,
    /// The OIDC issuer identifier of the IdP.
    #[builder(into)]
    #[serde(rename = "issuer")]
    pub r#issuer: Box<String>,
    /// The behavior if the user is not authenticated. Valid values: `deny`, `allow` and `authenticate`
    #[builder(into, default)]
    #[serde(rename = "onUnauthenticatedRequest")]
    pub r#on_unauthenticated_request: Box<Option<String>>,
    /// The set of user claims to be requested from the IdP.
    #[builder(into, default)]
    #[serde(rename = "scope")]
    pub r#scope: Box<Option<String>>,
    /// The name of the cookie used to maintain session information.
    #[builder(into, default)]
    #[serde(rename = "sessionCookieName")]
    pub r#session_cookie_name: Box<Option<String>>,
    /// The maximum duration of the authentication session, in seconds.
    #[builder(into, default)]
    #[serde(rename = "sessionTimeout")]
    pub r#session_timeout: Box<Option<i32>>,
    /// The token endpoint of the IdP.
    #[builder(into)]
    #[serde(rename = "tokenEndpoint")]
    pub r#token_endpoint: Box<String>,
    /// The user info endpoint of the IdP.
    #[builder(into)]
    #[serde(rename = "userInfoEndpoint")]
    pub r#user_info_endpoint: Box<String>,
}
