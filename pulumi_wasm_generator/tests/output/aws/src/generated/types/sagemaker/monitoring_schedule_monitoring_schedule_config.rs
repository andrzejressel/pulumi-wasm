#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct MonitoringScheduleMonitoringScheduleConfig {
    /// The name of the monitoring job definition to schedule.
    #[builder(into)]
    #[serde(rename = "monitoringJobDefinitionName")]
    pub r#monitoring_job_definition_name: Box<String>,
    /// The type of the monitoring job definition to schedule. Valid values are `DataQuality`, `ModelQuality`, `ModelBias` or `ModelExplainability`
    #[builder(into)]
    #[serde(rename = "monitoringType")]
    pub r#monitoring_type: Box<String>,
    /// Configures the monitoring schedule. Fields are documented below.
    #[builder(into, default)]
    #[serde(rename = "scheduleConfig")]
    pub r#schedule_config: Box<Option<super::super::types::sagemaker::MonitoringScheduleMonitoringScheduleConfigScheduleConfig>>,
}
