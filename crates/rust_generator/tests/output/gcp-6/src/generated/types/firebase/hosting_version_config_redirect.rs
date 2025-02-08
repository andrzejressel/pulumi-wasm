#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct HostingVersionConfigRedirect {
    /// The user-supplied glob to match against the request URL path.
    #[builder(into, default)]
    #[serde(rename = "glob")]
    pub r#glob: Box<Option<String>>,
    /// The value to put in the HTTP location header of the response.
    /// The location can contain capture group values from the pattern using a : prefix to identify
    /// the segment and an optional * to capture the rest of the URL. For example:
    #[builder(into)]
    #[serde(rename = "location")]
    pub r#location: Box<String>,
    /// The user-supplied RE2 regular expression to match against the request URL path.
    #[builder(into, default)]
    #[serde(rename = "regex")]
    pub r#regex: Box<Option<String>>,
    /// The status HTTP code to return in the response. It must be a valid 3xx status code.
    #[builder(into)]
    #[serde(rename = "statusCode")]
    pub r#status_code: Box<i32>,
}
