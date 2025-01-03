#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DatascanExecutionSpecTriggerSchedule {
    /// Cron schedule for running scans periodically. This field is required for Schedule scans.
    /// 
    /// - - -
    #[builder(into)]
    #[serde(rename = "cron")]
    pub r#cron: Box<String>,
}
