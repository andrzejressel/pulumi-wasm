#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ResourcePolicySnapshotSchedulePolicy {
    /// Retention policy applied to snapshots created by this resource policy.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "retentionPolicy")]
    pub r#retention_policy: Box<Option<super::super::types::compute::ResourcePolicySnapshotSchedulePolicyRetentionPolicy>>,
    /// Contains one of an `hourlySchedule`, `dailySchedule`, or `weeklySchedule`.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "schedule")]
    pub r#schedule: Box<super::super::types::compute::ResourcePolicySnapshotSchedulePolicySchedule>,
    /// Properties with which the snapshots are created, such as labels.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "snapshotProperties")]
    pub r#snapshot_properties: Box<Option<super::super::types::compute::ResourcePolicySnapshotSchedulePolicySnapshotProperties>>,
}
