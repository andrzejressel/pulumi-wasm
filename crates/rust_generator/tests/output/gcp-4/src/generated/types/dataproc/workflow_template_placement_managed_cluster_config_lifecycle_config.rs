#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct WorkflowTemplatePlacementManagedClusterConfigLifecycleConfig {
    /// The time when cluster will be auto-deleted (see JSON representation of [JSON Mapping - Language Guide (proto 3)](https://developers.google.com/protocol-buffers/docs/proto3#json)).
    #[builder(into, default)]
    #[serde(rename = "autoDeleteTime")]
    pub r#auto_delete_time: Box<Option<String>>,
    /// The lifetime duration of cluster. The cluster will be auto-deleted at the end of this period. Minimum value is 10 minutes; maximum value is 14 days (see JSON representation of [JSON Mapping - Language Guide (proto 3)](https://developers.google.com/protocol-buffers/docs/proto3#json)).
    #[builder(into, default)]
    #[serde(rename = "autoDeleteTtl")]
    pub r#auto_delete_ttl: Box<Option<String>>,
    /// The duration to keep the cluster alive while idling (when no jobs are running). Passing this threshold will cause the cluster to be deleted. Minimum value is 5 minutes; maximum value is 14 days (see JSON representation of [JSON Mapping - Language Guide (proto 3)](https://developers.google.com/protocol-buffers/docs/proto3#json).
    #[builder(into, default)]
    #[serde(rename = "idleDeleteTtl")]
    pub r#idle_delete_ttl: Box<Option<String>>,
    /// Output only. The time when cluster became idle (most recent job finished) and became eligible for deletion due to idleness (see JSON representation of [JSON Mapping - Language Guide (proto 3)](https://developers.google.com/protocol-buffers/docs/proto3#json)).
    #[builder(into, default)]
    #[serde(rename = "idleStartTime")]
    pub r#idle_start_time: Box<Option<String>>,
}
