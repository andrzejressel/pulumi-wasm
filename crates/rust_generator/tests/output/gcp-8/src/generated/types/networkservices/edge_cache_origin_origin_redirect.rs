#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct EdgeCacheOriginOriginRedirect {
    /// The set of redirect response codes that the CDN
    /// follows. Values of
    /// [RedirectConditions](https://cloud.google.com/media-cdn/docs/reference/rest/v1/projects.locations.edgeCacheOrigins#redirectconditions)
    /// are accepted.
    #[builder(into, default)]
    #[serde(rename = "redirectConditions")]
    pub r#redirect_conditions: Box<Option<Vec<String>>>,
}
