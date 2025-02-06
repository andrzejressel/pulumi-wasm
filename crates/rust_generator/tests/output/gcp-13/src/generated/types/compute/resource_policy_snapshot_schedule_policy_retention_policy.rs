#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ResourcePolicySnapshotSchedulePolicyRetentionPolicy {
    /// Maximum age of the snapshot that is allowed to be kept.
    #[builder(into)]
    #[serde(rename = "maxRetentionDays")]
    pub r#max_retention_days: Box<i32>,
    /// Specifies the behavior to apply to scheduled snapshots when
    /// the source disk is deleted.
    /// Default value is `KEEP_AUTO_SNAPSHOTS`.
    /// Possible values are: `KEEP_AUTO_SNAPSHOTS`, `APPLY_RETENTION_POLICY`.
    #[builder(into, default)]
    #[serde(rename = "onSourceDiskDelete")]
    pub r#on_source_disk_delete: Box<Option<String>>,
}
