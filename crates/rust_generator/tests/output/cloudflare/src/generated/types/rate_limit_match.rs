#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RateLimitMatch {
    /// Matches HTTP requests (from the client to Cloudflare).
    #[builder(into, default)]
    #[serde(rename = "request")]
    pub r#request: Box<Option<super::types::RateLimitMatchRequest>>,
    /// Matches HTTP responses before they are returned to the client from Cloudflare. If this is defined, then the entire counting of traffic occurs at this stage.
    #[builder(into, default)]
    #[serde(rename = "response")]
    pub r#response: Box<Option<super::types::RateLimitMatchResponse>>,
}
