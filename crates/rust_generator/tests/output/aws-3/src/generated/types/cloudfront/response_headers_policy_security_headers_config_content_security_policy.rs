#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ResponseHeadersPolicySecurityHeadersConfigContentSecurityPolicy {
    /// The policy directives and their values that CloudFront includes as values for the `Content-Security-Policy` HTTP response header.
    #[builder(into)]
    #[serde(rename = "contentSecurityPolicy")]
    pub r#content_security_policy: Box<String>,
    /// Whether CloudFront overrides the `Content-Security-Policy` HTTP response header received from the origin with the one specified in this response headers policy.
    #[builder(into)]
    #[serde(rename = "override")]
    pub r#override_: Box<bool>,
}
