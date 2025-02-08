#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RateLimitAction {
    /// The type of action to perform. Available values: `simulate`, `ban`, `challenge`, `js_challenge`, `managed_challenge`.
    #[builder(into)]
    #[serde(rename = "mode")]
    pub r#mode: Box<String>,
    /// Custom content-type and body to return, this overrides the custom error for the zone. This field is not required. Omission will result in default HTML error page.
    #[builder(into, default)]
    #[serde(rename = "response")]
    pub r#response: Box<Option<super::types::RateLimitActionResponse>>,
    /// The time in seconds as an integer to perform the mitigation action. This field is required if the `mode` is either `simulate` or `ban`. Must be the same or greater than the period.
    #[builder(into, default)]
    #[serde(rename = "timeout")]
    pub r#timeout: Box<Option<i32>>,
}
