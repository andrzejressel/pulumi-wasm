#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetBrokerMaintenanceWindowStartTime {
    #[builder(into)]
    #[serde(rename = "dayOfWeek")]
    pub r#day_of_week: Box<String>,
    #[builder(into)]
    #[serde(rename = "timeOfDay")]
    pub r#time_of_day: Box<String>,
    #[builder(into)]
    #[serde(rename = "timeZone")]
    pub r#time_zone: Box<String>,
}
