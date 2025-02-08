#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ClusterContinuousBackupInfo {
    /// (Output)
    /// The earliest restorable time that can be restored to. Output only field.
    #[builder(into, default)]
    #[serde(rename = "earliestRestorableTime")]
    pub r#earliest_restorable_time: Box<Option<String>>,
    /// (Output)
    /// When ContinuousBackup was most recently enabled. Set to null if ContinuousBackup is not enabled.
    #[builder(into, default)]
    #[serde(rename = "enabledTime")]
    pub r#enabled_time: Box<Option<String>>,
    /// (Output)
    /// Output only. The encryption information for the WALs and backups required for ContinuousBackup.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "encryptionInfos")]
    pub r#encryption_infos: Box<Option<Vec<super::super::types::alloydb::ClusterContinuousBackupInfoEncryptionInfo>>>,
    /// (Output)
    /// Days of the week on which a continuous backup is taken. Output only field. Ignored if passed into the request.
    #[builder(into, default)]
    #[serde(rename = "schedules")]
    pub r#schedules: Box<Option<Vec<String>>>,
}
