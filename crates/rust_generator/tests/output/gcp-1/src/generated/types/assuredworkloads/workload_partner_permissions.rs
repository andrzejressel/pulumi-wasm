#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct WorkloadPartnerPermissions {
    /// Optional. Allow partner to view violation alerts.
    #[builder(into, default)]
    #[serde(rename = "assuredWorkloadsMonitoring")]
    pub r#assured_workloads_monitoring: Box<Option<bool>>,
    /// Allow the partner to view inspectability logs and monitoring violations.
    #[builder(into, default)]
    #[serde(rename = "dataLogsViewer")]
    pub r#data_logs_viewer: Box<Option<bool>>,
    /// Optional. Allow partner to view access approval logs.
    #[builder(into, default)]
    #[serde(rename = "serviceAccessApprover")]
    pub r#service_access_approver: Box<Option<bool>>,
}
