#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DomainAutoTuneOptions {
    /// The Auto-Tune desired state for the domain. Valid values: `ENABLED` or `DISABLED`.
    #[builder(into)]
    #[serde(rename = "desiredState")]
    pub r#desired_state: Box<String>,
    /// Configuration block for Auto-Tune maintenance windows. Can be specified multiple times for each maintenance window. Detailed below.
    #[builder(into, default)]
    #[serde(rename = "maintenanceSchedules")]
    pub r#maintenance_schedules: Box<Option<Vec<super::super::types::elasticsearch::DomainAutoTuneOptionsMaintenanceSchedule>>>,
    /// Whether to roll back to default Auto-Tune settings when disabling Auto-Tune. Valid values: `DEFAULT_ROLLBACK` or `NO_ROLLBACK`.
    #[builder(into, default)]
    #[serde(rename = "rollbackOnDisable")]
    pub r#rollback_on_disable: Box<Option<String>>,
}
