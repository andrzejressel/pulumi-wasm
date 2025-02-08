#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct DatabaseInstanceRestoreBackupContext {
    /// The ID of the backup run to restore from.
    #[builder(into)]
    #[serde(rename = "backupRunId")]
    pub r#backup_run_id: Box<i32>,
    /// The ID of the instance that the backup was taken from. If left empty,
    /// this instance's ID will be used.
    #[builder(into, default)]
    #[serde(rename = "instanceId")]
    pub r#instance_id: Box<Option<String>>,
    /// The full project ID of the source instance.`
    #[builder(into, default)]
    #[serde(rename = "project")]
    pub r#project: Box<Option<String>>,
}
