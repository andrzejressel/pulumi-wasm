#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct StandardWebTestRequest {
    /// The WebTest request body.
    #[builder(into, default)]
    #[serde(rename = "body")]
    pub r#body: Box<Option<String>>,
    /// Should the following of redirects be enabled? Defaults to `true`.
    #[builder(into, default)]
    #[serde(rename = "followRedirectsEnabled")]
    pub r#follow_redirects_enabled: Box<Option<bool>>,
    /// One or more `header` blocks as defined above.
    #[builder(into, default)]
    #[serde(rename = "headers")]
    pub r#headers: Box<Option<Vec<super::super::types::appinsights::StandardWebTestRequestHeader>>>,
    /// Which HTTP verb to use for the call. Options are 'GET', 'POST', 'PUT', 'PATCH', 'DELETE', 'HEAD', and 'OPTIONS'. Defaults to `GET`.
    #[builder(into, default)]
    #[serde(rename = "httpVerb")]
    pub r#http_verb: Box<Option<String>>,
    /// Should the parsing of dependend requests be enabled? Defaults to `true`.
    #[builder(into, default)]
    #[serde(rename = "parseDependentRequestsEnabled")]
    pub r#parse_dependent_requests_enabled: Box<Option<bool>>,
    /// The WebTest request URL.
    #[builder(into)]
    #[serde(rename = "url")]
    pub r#url: Box<String>,
}
