#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetDataCollectionRuleDataSource {
    /// A `data_import` block as defined above.
    #[builder(into)]
    #[serde(rename = "dataImports")]
    pub r#data_imports: Box<Vec<super::super::types::monitoring::GetDataCollectionRuleDataSourceDataImport>>,
    /// One or more `extension` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "extensions")]
    pub r#extensions: Box<Vec<super::super::types::monitoring::GetDataCollectionRuleDataSourceExtension>>,
    /// One or more `iis_log` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "iisLogs")]
    pub r#iis_logs: Box<Vec<super::super::types::monitoring::GetDataCollectionRuleDataSourceIisLog>>,
    /// One or more `log_file` blocks as defined below.
    #[builder(into, default)]
    #[serde(rename = "logFiles")]
    pub r#log_files: Box<Option<Vec<super::super::types::monitoring::GetDataCollectionRuleDataSourceLogFile>>>,
    /// One or more `performance_counter` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "performanceCounters")]
    pub r#performance_counters: Box<Vec<super::super::types::monitoring::GetDataCollectionRuleDataSourcePerformanceCounter>>,
    /// One or more `platform_telemetry` blocks as defined below.
    #[builder(into, default)]
    #[serde(rename = "platformTelemetries")]
    pub r#platform_telemetries: Box<Option<Vec<super::super::types::monitoring::GetDataCollectionRuleDataSourcePlatformTelemetry>>>,
    /// One or more `prometheus_forwarder` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "prometheusForwarders")]
    pub r#prometheus_forwarders: Box<Vec<super::super::types::monitoring::GetDataCollectionRuleDataSourcePrometheusForwarder>>,
    /// One or more `syslog` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "syslogs")]
    pub r#syslogs: Box<Vec<super::super::types::monitoring::GetDataCollectionRuleDataSourceSyslog>>,
    /// One or more `windows_event_log` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "windowsEventLogs")]
    pub r#windows_event_logs: Box<Vec<super::super::types::monitoring::GetDataCollectionRuleDataSourceWindowsEventLog>>,
    /// One or more `windows_firewall_log` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "windowsFirewallLogs")]
    pub r#windows_firewall_logs: Box<Vec<super::super::types::monitoring::GetDataCollectionRuleDataSourceWindowsFirewallLog>>,
}
