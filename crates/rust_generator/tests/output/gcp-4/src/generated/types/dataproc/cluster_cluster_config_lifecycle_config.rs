#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ClusterClusterConfigLifecycleConfig {
    /// The time when cluster will be auto-deleted.
    /// A timestamp in RFC3339 UTC "Zulu" format, accurate to nanoseconds.
    /// Example: "2014-10-02T15:01:23.045123456Z".
    /// 
    /// - - -
    #[builder(into, default)]
    #[serde(rename = "autoDeleteTime")]
    pub r#auto_delete_time: Box<Option<String>>,
    /// The duration to keep the cluster alive while idling
    /// (no jobs running). After this TTL, the cluster will be deleted. Valid range: [10m, 14d].
    #[builder(into, default)]
    #[serde(rename = "idleDeleteTtl")]
    pub r#idle_delete_ttl: Box<Option<String>>,
    /// Time when the cluster became idle
    /// (most recent job finished) and became eligible for deletion due to idleness.
    #[builder(into, default)]
    #[serde(rename = "idleStartTime")]
    pub r#idle_start_time: Box<Option<String>>,
}
