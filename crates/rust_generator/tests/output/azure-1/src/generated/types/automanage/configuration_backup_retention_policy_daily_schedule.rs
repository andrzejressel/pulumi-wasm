#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ConfigurationBackupRetentionPolicyDailySchedule {
    /// A `retention_duration` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "retentionDuration")]
    pub r#retention_duration: Box<Option<super::super::types::automanage::ConfigurationBackupRetentionPolicyDailyScheduleRetentionDuration>>,
    /// The retention times of the backup policy.
    #[builder(into, default)]
    #[serde(rename = "retentionTimes")]
    pub r#retention_times: Box<Option<Vec<String>>>,
}
