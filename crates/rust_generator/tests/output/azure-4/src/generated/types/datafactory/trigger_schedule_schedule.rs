#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct TriggerScheduleSchedule {
    /// Day(s) of the month on which the trigger is scheduled. This value can be specified with a monthly frequency only.
    #[builder(into, default)]
    #[serde(rename = "daysOfMonths")]
    pub r#days_of_months: Box<Option<Vec<i32>>>,
    /// Days of the week on which the trigger is scheduled. This value can be specified only with a weekly frequency.
    #[builder(into, default)]
    #[serde(rename = "daysOfWeeks")]
    pub r#days_of_weeks: Box<Option<Vec<String>>>,
    /// Hours of the day on which the trigger is scheduled.
    #[builder(into, default)]
    #[serde(rename = "hours")]
    pub r#hours: Box<Option<Vec<i32>>>,
    /// Minutes of the hour on which the trigger is scheduled.
    #[builder(into, default)]
    #[serde(rename = "minutes")]
    pub r#minutes: Box<Option<Vec<i32>>>,
    /// A `monthly` block as documented below, which specifies the days of the month on which the trigger is scheduled. The value can be specified only with a monthly frequency.
    #[builder(into, default)]
    #[serde(rename = "monthlies")]
    pub r#monthlies: Box<Option<Vec<super::super::types::datafactory::TriggerScheduleScheduleMonthly>>>,
}
