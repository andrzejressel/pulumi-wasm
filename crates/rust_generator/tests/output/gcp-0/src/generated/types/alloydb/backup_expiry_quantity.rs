#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct BackupExpiryQuantity {
    /// (Output)
    /// Output only. The backup's position among its backups with the same source cluster and type, by descending chronological order create time (i.e. newest first).
    #[builder(into, default)]
    #[serde(rename = "retentionCount")]
    pub r#retention_count: Box<Option<i32>>,
    /// (Output)
    /// Output only. The length of the quantity-based queue, specified by the backup's retention policy.
    #[builder(into, default)]
    #[serde(rename = "totalRetentionCount")]
    pub r#total_retention_count: Box<Option<i32>>,
}
