#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct HostPoolScheduledAgentUpdatesSchedule {
    /// The day of the week on which agent updates should be performed. Possible values are `Monday`, `Tuesday`, `Wednesday`, `Thursday`, `Friday`, `Saturday`, and `Sunday`
    #[builder(into)]
    #[serde(rename = "dayOfWeek")]
    pub r#day_of_week: Box<String>,
    /// The hour of day the update window should start. The update is a 2 hour period following the hour provided. The value should be provided as a number between 0 and 23, with 0 being midnight and 23 being 11pm. A leading zero should not be used.
    #[builder(into)]
    #[serde(rename = "hourOfDay")]
    pub r#hour_of_day: Box<i32>,
}
