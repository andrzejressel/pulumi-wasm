#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PolicyFileShareBackup {
    /// Sets the backup frequency. Possible values are `Daily` and `Hourly`. 
    /// 
    /// > **NOTE:** This argument is made available for consistency with VM backup policies and to allow for potential future support of weekly backups
    #[builder(into)]
    #[serde(rename = "frequency")]
    pub r#frequency: Box<String>,
    /// A `hourly` block defined as below. This is required when `frequency` is set to `Hourly`.
    #[builder(into, default)]
    #[serde(rename = "hourly")]
    pub r#hourly: Box<Option<super::super::types::backup::PolicyFileShareBackupHourly>>,
    /// The time of day to perform the backup in 24-hour format. Times must be either on the hour or half hour (e.g. 12:00, 12:30, 13:00, etc.)
    /// 
    /// > **NOTE:** `time` is required when `frequency` is set to `Daily`.
    #[builder(into, default)]
    #[serde(rename = "time")]
    pub r#time: Box<Option<String>>,
}
