#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetClusterMaintenancePolicyDailyMaintenanceWindow {
    #[builder(into)]
    #[serde(rename = "duration")]
    pub r#duration: Box<String>,
    #[builder(into)]
    #[serde(rename = "startTime")]
    pub r#start_time: Box<String>,
}
