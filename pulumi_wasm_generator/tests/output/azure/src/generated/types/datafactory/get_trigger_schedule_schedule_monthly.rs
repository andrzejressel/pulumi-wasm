#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetTriggerScheduleScheduleMonthly {
    /// The occurrence of the specified day during the month.
    #[builder(into)]
    #[serde(rename = "week")]
    pub r#week: Box<i32>,
    /// The day of the week on which the trigger runs.
    #[builder(into)]
    #[serde(rename = "weekday")]
    pub r#weekday: Box<String>,
}