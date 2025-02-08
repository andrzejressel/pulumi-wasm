#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DatabaseInstanceSettingsInsightsConfig {
    /// True if Query Insights feature is enabled.
    #[builder(into, default)]
    #[serde(rename = "queryInsightsEnabled")]
    pub r#query_insights_enabled: Box<Option<bool>>,
    /// Number of query execution plans captured by Insights per minute for all queries combined. Between 0 and 20. Default to 5.
    #[builder(into, default)]
    #[serde(rename = "queryPlansPerMinute")]
    pub r#query_plans_per_minute: Box<Option<i32>>,
    /// Maximum query length stored in bytes. Between 256 and 4500. Default to 1024. Higher query lengths are more useful for analytical queries, but they also require more memory. Changing the query length requires you to restart the instance. You can still add tags to queries that exceed the length limit.
    #[builder(into, default)]
    #[serde(rename = "queryStringLength")]
    pub r#query_string_length: Box<Option<i32>>,
    /// True if Query Insights will record application tags from query when enabled.
    #[builder(into, default)]
    #[serde(rename = "recordApplicationTags")]
    pub r#record_application_tags: Box<Option<bool>>,
    /// True if Query Insights will record client address when enabled.
    #[builder(into, default)]
    #[serde(rename = "recordClientAddress")]
    pub r#record_client_address: Box<Option<bool>>,
}
