#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct InstanceObservabilityConfig {
    /// Observability feature status for an instance.
    #[builder(into, default)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
    /// Query string length. The default value is 10240. Any integer between 1024 and 100000 is considered valid.
    #[builder(into, default)]
    #[serde(rename = "maxQueryStringLength")]
    pub r#max_query_string_length: Box<Option<i32>>,
    /// Preserve comments in the query string.
    #[builder(into, default)]
    #[serde(rename = "preserveComments")]
    pub r#preserve_comments: Box<Option<bool>>,
    /// Number of query execution plans captured by Insights per minute for all queries combined. The default value is 5. Any integer between 0 and 200 is considered valid.
    #[builder(into, default)]
    #[serde(rename = "queryPlansPerMinute")]
    pub r#query_plans_per_minute: Box<Option<i32>>,
    /// Record application tags for an instance. This flag is turned "on" by default.
    #[builder(into, default)]
    #[serde(rename = "recordApplicationTags")]
    pub r#record_application_tags: Box<Option<bool>>,
    /// Track actively running queries. If not set, default value is "off".
    #[builder(into, default)]
    #[serde(rename = "trackActiveQueries")]
    pub r#track_active_queries: Box<Option<bool>>,
    /// Record wait event types during query execution for an instance.
    #[builder(into, default)]
    #[serde(rename = "trackWaitEventTypes")]
    pub r#track_wait_event_types: Box<Option<bool>>,
    /// Record wait events during query execution for an instance.
    #[builder(into, default)]
    #[serde(rename = "trackWaitEvents")]
    pub r#track_wait_events: Box<Option<bool>>,
}
