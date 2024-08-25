#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct RateLimitAction {
    /// The type of action to perform. Available values: `simulate`, `ban`, `challenge`, `js_challenge`, `managed_challenge`.
    #[serde(rename = "mode")]
    pub r#mode: Box<String>,
    /// Custom content-type and body to return, this overrides the custom error for the zone. This field is not required. Omission will result in default HTML error page.
    #[serde(rename = "response")]
    pub r#response: Box<Option<crate::types::RateLimitActionResponse>>,
    /// The time in seconds as an integer to perform the mitigation action. This field is required if the `mode` is either `simulate` or `ban`. Must be the same or greater than the period.
    #[serde(rename = "timeout")]
    pub r#timeout: Box<Option<i32>>,
}
