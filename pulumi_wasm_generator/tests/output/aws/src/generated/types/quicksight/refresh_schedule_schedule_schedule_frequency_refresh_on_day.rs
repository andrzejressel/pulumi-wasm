#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RefreshScheduleScheduleScheduleFrequencyRefreshOnDay {
    /// The day of the month that you want to schedule refresh on.
    #[builder(into, default)]
    #[serde(rename = "dayOfMonth")]
    pub r#day_of_month: Box<Option<String>>,
    /// The day of the week that you want to schedule a refresh on. Valid values are `SUNDAY`, `MONDAY`, `TUESDAY`, `WEDNESDAY`, `THURSDAY`, `FRIDAY` and `SATURDAY`.
    #[builder(into, default)]
    #[serde(rename = "dayOfWeek")]
    pub r#day_of_week: Box<Option<String>>,
}