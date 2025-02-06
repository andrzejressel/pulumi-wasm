#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct UptimeCheckConfigContentMatcherJsonPathMatcher {
    /// Options to perform JSONPath content matching.
    /// Default value is `EXACT_MATCH`.
    /// Possible values are: `EXACT_MATCH`, `REGEX_MATCH`.
    #[builder(into, default)]
    #[serde(rename = "jsonMatcher")]
    pub r#json_matcher: Box<Option<String>>,
    /// JSONPath within the response output pointing to the expected `ContentMatcher::content` to match against.
    #[builder(into)]
    #[serde(rename = "jsonPath")]
    pub r#json_path: Box<String>,
}
