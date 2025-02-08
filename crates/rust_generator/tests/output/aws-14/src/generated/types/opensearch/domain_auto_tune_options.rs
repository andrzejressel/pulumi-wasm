#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct DomainAutoTuneOptions {
    /// Auto-Tune desired state for the domain. Valid values: `ENABLED` or `DISABLED`.
    #[builder(into)]
    #[serde(rename = "desiredState")]
    pub r#desired_state: Box<String>,
    /// Configuration block for Auto-Tune maintenance windows. Can be specified multiple times for each maintenance window. Detailed below.
    /// 
    /// **NOTE:** Maintenance windows are deprecated and have been replaced with [off-peak windows](https://docs.aws.amazon.com/opensearch-service/latest/developerguide/off-peak.html). Consequently, `maintenance_schedule` configuration blocks cannot be specified when `use_off_peak_window` is set to `true`.
    #[builder(into, default)]
    #[serde(rename = "maintenanceSchedules")]
    pub r#maintenance_schedules: Box<Option<Vec<super::super::types::opensearch::DomainAutoTuneOptionsMaintenanceSchedule>>>,
    /// Whether to roll back to default Auto-Tune settings when disabling Auto-Tune. Valid values: `DEFAULT_ROLLBACK` or `NO_ROLLBACK`.
    #[builder(into, default)]
    #[serde(rename = "rollbackOnDisable")]
    pub r#rollback_on_disable: Box<Option<String>>,
    /// Whether to schedule Auto-Tune optimizations that require blue/green deployments during the domain's configured daily off-peak window. Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "useOffPeakWindow")]
    pub r#use_off_peak_window: Box<Option<bool>>,
}
