#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct VaultMonitoring {
    /// Enabling/Disabling built-in Azure Monitor alerts for security scenarios and job failure scenarios. Defaults to `true`.
    #[builder(into, default)]
    #[serde(rename = "alertsForAllJobFailuresEnabled")]
    pub r#alerts_for_all_job_failures_enabled: Box<Option<bool>>,
    /// Enabling/Disabling alerts from the older (classic alerts) solution. Defaults to `true`. More details could be found [here](https://learn.microsoft.com/en-us/azure/backup/monitoring-and-alerts-overview).
    #[builder(into, default)]
    #[serde(rename = "alertsForCriticalOperationFailuresEnabled")]
    pub r#alerts_for_critical_operation_failures_enabled: Box<Option<bool>>,
}
