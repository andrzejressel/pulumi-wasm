#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AnalyticsConfigurationFilter {
    /// Object prefix for filtering.
    #[builder(into, default)]
    #[serde(rename = "prefix")]
    pub r#prefix: Box<Option<String>>,
    /// Set of object tags for filtering.
    #[builder(into, default)]
    #[serde(rename = "tags")]
    pub r#tags: Box<Option<std::collections::HashMap<String, String>>>,
}
