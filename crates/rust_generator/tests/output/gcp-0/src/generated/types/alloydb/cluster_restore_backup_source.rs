#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ClusterRestoreBackupSource {
    /// The name of the backup that this cluster is restored from.
    #[builder(into)]
    #[serde(rename = "backupName")]
    pub r#backup_name: Box<String>,
}
