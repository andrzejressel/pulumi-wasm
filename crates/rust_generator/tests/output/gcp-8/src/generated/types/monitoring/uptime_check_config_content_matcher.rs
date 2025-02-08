#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct UptimeCheckConfigContentMatcher {
    /// String or regex content to match (max 1024 bytes)
    #[builder(into)]
    #[serde(rename = "content")]
    pub r#content: Box<String>,
    /// Information needed to perform a JSONPath content match. Used for `ContentMatcherOption::MATCHES_JSON_PATH` and `ContentMatcherOption::NOT_MATCHES_JSON_PATH`.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "jsonPathMatcher")]
    pub r#json_path_matcher: Box<Option<super::super::types::monitoring::UptimeCheckConfigContentMatcherJsonPathMatcher>>,
    /// The type of content matcher that will be applied to the server output, compared to the content string when the check is run.
    /// Default value is `CONTAINS_STRING`.
    /// Possible values are: `CONTAINS_STRING`, `NOT_CONTAINS_STRING`, `MATCHES_REGEX`, `NOT_MATCHES_REGEX`, `MATCHES_JSON_PATH`, `NOT_MATCHES_JSON_PATH`.
    #[builder(into, default)]
    #[serde(rename = "matcher")]
    pub r#matcher: Box<Option<String>>,
}
