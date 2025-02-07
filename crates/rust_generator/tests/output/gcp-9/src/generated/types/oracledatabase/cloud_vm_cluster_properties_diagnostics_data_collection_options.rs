#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct CloudVmClusterPropertiesDiagnosticsDataCollectionOptions {
    /// Indicates whether diagnostic collection is enabled for the VM cluster
    #[builder(into, default)]
    #[serde(rename = "diagnosticsEventsEnabled")]
    pub r#diagnostics_events_enabled: Box<Option<bool>>,
    /// Indicates whether health monitoring is enabled for the VM cluster
    #[builder(into, default)]
    #[serde(rename = "healthMonitoringEnabled")]
    pub r#health_monitoring_enabled: Box<Option<bool>>,
    /// Indicates whether incident logs and trace collection are enabled for the VM
    /// cluster
    #[builder(into, default)]
    #[serde(rename = "incidentLogsEnabled")]
    pub r#incident_logs_enabled: Box<Option<bool>>,
}
