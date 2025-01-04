#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct BrokerMaintenanceWindowStartTime {
    /// Day of the week, e.g., `MONDAY`, `TUESDAY`, or `WEDNESDAY`.
    #[builder(into)]
    #[serde(rename = "dayOfWeek")]
    pub r#day_of_week: Box<String>,
    /// Time, in 24-hour format, e.g., `02:00`.
    #[builder(into)]
    #[serde(rename = "timeOfDay")]
    pub r#time_of_day: Box<String>,
    /// Time zone in either the Country/City format or the UTC offset format, e.g., `CET`.
    #[builder(into)]
    #[serde(rename = "timeZone")]
    pub r#time_zone: Box<String>,
}
