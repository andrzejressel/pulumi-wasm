#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DataCollectionRuleDataSources {
    /// A `data_import` block as defined above.
    #[builder(into, default)]
    #[serde(rename = "dataImport")]
    pub r#data_import: Box<Option<super::super::types::monitoring::DataCollectionRuleDataSourcesDataImport>>,
    /// One or more `extension` blocks as defined below.
    #[builder(into, default)]
    #[serde(rename = "extensions")]
    pub r#extensions: Box<Option<Vec<super::super::types::monitoring::DataCollectionRuleDataSourcesExtension>>>,
    /// One or more `iis_log` blocks as defined below.
    #[builder(into, default)]
    #[serde(rename = "iisLogs")]
    pub r#iis_logs: Box<Option<Vec<super::super::types::monitoring::DataCollectionRuleDataSourcesIisLog>>>,
    /// One or more `log_file` blocks as defined below.
    #[builder(into, default)]
    #[serde(rename = "logFiles")]
    pub r#log_files: Box<Option<Vec<super::super::types::monitoring::DataCollectionRuleDataSourcesLogFile>>>,
    /// One or more `performance_counter` blocks as defined below.
    #[builder(into, default)]
    #[serde(rename = "performanceCounters")]
    pub r#performance_counters: Box<Option<Vec<super::super::types::monitoring::DataCollectionRuleDataSourcesPerformanceCounter>>>,
    /// One or more `platform_telemetry` blocks as defined below.
    #[builder(into, default)]
    #[serde(rename = "platformTelemetries")]
    pub r#platform_telemetries: Box<Option<Vec<super::super::types::monitoring::DataCollectionRuleDataSourcesPlatformTelemetry>>>,
    /// One or more `prometheus_forwarder` blocks as defined below.
    #[builder(into, default)]
    #[serde(rename = "prometheusForwarders")]
    pub r#prometheus_forwarders: Box<Option<Vec<super::super::types::monitoring::DataCollectionRuleDataSourcesPrometheusForwarder>>>,
    /// One or more `syslog` blocks as defined below.
    #[builder(into, default)]
    #[serde(rename = "syslogs")]
    pub r#syslogs: Box<Option<Vec<super::super::types::monitoring::DataCollectionRuleDataSourcesSyslog>>>,
    /// One or more `windows_event_log` blocks as defined below.
    #[builder(into, default)]
    #[serde(rename = "windowsEventLogs")]
    pub r#windows_event_logs: Box<Option<Vec<super::super::types::monitoring::DataCollectionRuleDataSourcesWindowsEventLog>>>,
    /// One or more `windows_firewall_log` blocks as defined below.
    #[builder(into, default)]
    #[serde(rename = "windowsFirewallLogs")]
    pub r#windows_firewall_logs: Box<Option<Vec<super::super::types::monitoring::DataCollectionRuleDataSourcesWindowsFirewallLog>>>,
}
