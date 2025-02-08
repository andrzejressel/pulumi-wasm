#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ListenerDefaultActionAuthenticateCognito {
    /// Query parameters to include in the redirect request to the authorization endpoint. Max: 10. See below.
    #[builder(into, default)]
    #[serde(rename = "authenticationRequestExtraParams")]
    pub r#authentication_request_extra_params: Box<Option<std::collections::HashMap<String, String>>>,
    /// Behavior if the user is not authenticated. Valid values are `deny`, `allow` and `authenticate`.
    #[builder(into, default)]
    #[serde(rename = "onUnauthenticatedRequest")]
    pub r#on_unauthenticated_request: Box<Option<String>>,
    /// Set of user claims to be requested from the IdP.
    #[builder(into, default)]
    #[serde(rename = "scope")]
    pub r#scope: Box<Option<String>>,
    /// Name of the cookie used to maintain session information.
    #[builder(into, default)]
    #[serde(rename = "sessionCookieName")]
    pub r#session_cookie_name: Box<Option<String>>,
    /// Maximum duration of the authentication session, in seconds.
    #[builder(into, default)]
    #[serde(rename = "sessionTimeout")]
    pub r#session_timeout: Box<Option<i32>>,
    /// ARN of the Cognito user pool.
    #[builder(into)]
    #[serde(rename = "userPoolArn")]
    pub r#user_pool_arn: Box<String>,
    /// ID of the Cognito user pool client.
    #[builder(into)]
    #[serde(rename = "userPoolClientId")]
    pub r#user_pool_client_id: Box<String>,
    /// Domain prefix or fully-qualified domain name of the Cognito user pool.
    /// 
    /// The following arguments are optional:
    #[builder(into)]
    #[serde(rename = "userPoolDomain")]
    pub r#user_pool_domain: Box<String>,
}
