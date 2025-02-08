#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetListenerDefaultActionAuthenticateCognito {
    #[builder(into)]
    #[serde(rename = "authenticationRequestExtraParams")]
    pub r#authentication_request_extra_params: Box<std::collections::HashMap<String, String>>,
    #[builder(into)]
    #[serde(rename = "onUnauthenticatedRequest")]
    pub r#on_unauthenticated_request: Box<String>,
    #[builder(into)]
    #[serde(rename = "scope")]
    pub r#scope: Box<String>,
    #[builder(into)]
    #[serde(rename = "sessionCookieName")]
    pub r#session_cookie_name: Box<String>,
    #[builder(into)]
    #[serde(rename = "sessionTimeout")]
    pub r#session_timeout: Box<i32>,
    #[builder(into)]
    #[serde(rename = "userPoolArn")]
    pub r#user_pool_arn: Box<String>,
    #[builder(into)]
    #[serde(rename = "userPoolClientId")]
    pub r#user_pool_client_id: Box<String>,
    #[builder(into)]
    #[serde(rename = "userPoolDomain")]
    pub r#user_pool_domain: Box<String>,
}
