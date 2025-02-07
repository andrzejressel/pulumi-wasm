#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetResourcePolicySnapshotSchedulePolicy {
    /// Retention policy applied to snapshots created by this resource policy.
    #[builder(into)]
    #[serde(rename = "retentionPolicies")]
    pub r#retention_policies: Box<Vec<super::super::types::compute::GetResourcePolicySnapshotSchedulePolicyRetentionPolicy>>,
    /// Contains one of an 'hourlySchedule', 'dailySchedule', or 'weeklySchedule'.
    #[builder(into)]
    #[serde(rename = "schedules")]
    pub r#schedules: Box<Vec<super::super::types::compute::GetResourcePolicySnapshotSchedulePolicySchedule>>,
    /// Properties with which the snapshots are created, such as labels.
    #[builder(into)]
    #[serde(rename = "snapshotProperties")]
    pub r#snapshot_properties: Box<Vec<super::super::types::compute::GetResourcePolicySnapshotSchedulePolicySnapshotProperty>>,
}
