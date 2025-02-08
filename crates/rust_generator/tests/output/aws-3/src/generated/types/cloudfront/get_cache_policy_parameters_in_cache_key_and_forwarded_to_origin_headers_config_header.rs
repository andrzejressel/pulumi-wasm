#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetCachePolicyParametersInCacheKeyAndForwardedToOriginHeadersConfigHeader {
    /// List of item names (`cookies`, `headers`, or `query_strings`).
    #[builder(into)]
    #[serde(rename = "items")]
    pub r#items: Box<Vec<String>>,
}
