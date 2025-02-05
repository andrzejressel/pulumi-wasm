#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct MonitoringScheduleMonitoringScheduleConfigScheduleConfig {
    /// A cron expression that describes details about the monitoring schedule. For example, and hourly schedule would be `cron(0 * ? * * *)`.
    #[builder(into)]
    #[serde(rename = "scheduleExpression")]
    pub r#schedule_expression: Box<String>,
}
