#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct BackupPlanBackupScheduleRpoConfig {
    /// User specified time windows during which backup can NOT happen for this BackupPlan.
    /// Backups should start and finish outside of any given exclusion window. Note: backup
    /// jobs will be scheduled to start and finish outside the duration of the window as
    /// much as possible, but running jobs will not get canceled when it runs into the window.
    /// All the time and date values in exclusionWindows entry in the API are in UTC. We
    /// only allow <=1 recurrence (daily or weekly) exclusion window for a BackupPlan while no
    /// restriction on number of single occurrence windows.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "exclusionWindows")]
    pub r#exclusion_windows: Box<Option<Vec<super::super::types::gkebackup::BackupPlanBackupScheduleRpoConfigExclusionWindow>>>,
    /// Defines the target RPO for the BackupPlan in minutes, which means the target
    /// maximum data loss in time that is acceptable for this BackupPlan. This must be
    /// at least 60, i.e., 1 hour, and at most 86400, i.e., 60 days.
    #[builder(into)]
    #[serde(rename = "targetRpoMinutes")]
    pub r#target_rpo_minutes: Box<i32>,
}
