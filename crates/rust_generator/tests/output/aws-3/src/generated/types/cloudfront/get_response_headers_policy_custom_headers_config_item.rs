#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetResponseHeadersPolicyCustomHeadersConfigItem {
    /// The HTTP header name.
    #[builder(into)]
    #[serde(rename = "header")]
    pub r#header: Box<String>,
    /// Whether CloudFront overrides the X-XSS-Protection HTTP response header received from the origin with the one specified in this response headers policy.
    #[builder(into)]
    #[serde(rename = "override")]
    pub r#override_: Box<bool>,
    /// Value for the HTTP response header.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Box<String>,
}
