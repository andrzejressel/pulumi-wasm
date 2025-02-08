#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ListenerRuleActionAuthenticateCognito {
    /// The query parameters to include in the redirect request to the authorization endpoint. Max: 10.
    #[builder(into, default)]
    #[serde(rename = "authenticationRequestExtraParams")]
    pub r#authentication_request_extra_params: Box<Option<std::collections::HashMap<String, String>>>,
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
    /// The ARN of the Cognito user pool.
    #[builder(into)]
    #[serde(rename = "userPoolArn")]
    pub r#user_pool_arn: Box<String>,
    /// The ID of the Cognito user pool client.
    #[builder(into)]
    #[serde(rename = "userPoolClientId")]
    pub r#user_pool_client_id: Box<String>,
    /// The domain prefix or fully-qualified domain name of the Cognito user pool.
    #[builder(into)]
    #[serde(rename = "userPoolDomain")]
    pub r#user_pool_domain: Box<String>,
}
