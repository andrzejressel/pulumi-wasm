#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetDomainAutoTuneOption {
    /// The Auto-Tune desired state for the domain.
    #[builder(into)]
    #[serde(rename = "desiredState")]
    pub r#desired_state: Box<String>,
    /// A list of the nested configurations for the Auto-Tune maintenance windows of the domain.
    #[builder(into)]
    #[serde(rename = "maintenanceSchedules")]
    pub r#maintenance_schedules: Box<Vec<super::super::types::elasticsearch::GetDomainAutoTuneOptionMaintenanceSchedule>>,
    /// Whether the domain is set to roll back to default Auto-Tune settings when disabling Auto-Tune.
    #[builder(into)]
    #[serde(rename = "rollbackOnDisable")]
    pub r#rollback_on_disable: Box<String>,
}
