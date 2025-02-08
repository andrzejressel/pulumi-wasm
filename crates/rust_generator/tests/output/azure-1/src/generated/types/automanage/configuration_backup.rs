#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ConfigurationBackup {
    /// The retention range in days of the backup policy. Defaults to `5`.
    #[builder(into, default)]
    #[serde(rename = "instantRpRetentionRangeInDays")]
    pub r#instant_rp_retention_range_in_days: Box<Option<i32>>,
    /// The name of the backup policy.
    #[builder(into, default)]
    #[serde(rename = "policyName")]
    pub r#policy_name: Box<Option<String>>,
    /// A `retention_policy` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "retentionPolicy")]
    pub r#retention_policy: Box<Option<super::super::types::automanage::ConfigurationBackupRetentionPolicy>>,
    /// A `schedule_policy` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "schedulePolicy")]
    pub r#schedule_policy: Box<Option<super::super::types::automanage::ConfigurationBackupSchedulePolicy>>,
    /// The timezone of the backup policy. Defaults to `UTC`.
    #[builder(into, default)]
    #[serde(rename = "timeZone")]
    pub r#time_zone: Box<Option<String>>,
}
