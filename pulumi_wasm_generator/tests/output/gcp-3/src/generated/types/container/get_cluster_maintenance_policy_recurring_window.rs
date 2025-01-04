#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetClusterMaintenancePolicyRecurringWindow {
    #[builder(into)]
    #[serde(rename = "endTime")]
    pub r#end_time: Box<String>,
    #[builder(into)]
    #[serde(rename = "recurrence")]
    pub r#recurrence: Box<String>,
    #[builder(into)]
    #[serde(rename = "startTime")]
    pub r#start_time: Box<String>,
}
