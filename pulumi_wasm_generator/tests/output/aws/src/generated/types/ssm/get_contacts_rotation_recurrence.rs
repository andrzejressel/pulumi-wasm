#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetContactsRotationRecurrence {
    #[builder(into)]
    #[serde(rename = "dailySettings")]
    pub r#daily_settings: Box<Vec<super::super::types::ssm::GetContactsRotationRecurrenceDailySetting>>,
    #[builder(into)]
    #[serde(rename = "monthlySettings")]
    pub r#monthly_settings: Box<Vec<super::super::types::ssm::GetContactsRotationRecurrenceMonthlySetting>>,
    #[builder(into)]
    #[serde(rename = "numberOfOnCalls")]
    pub r#number_of_on_calls: Box<i32>,
    #[builder(into)]
    #[serde(rename = "recurrenceMultiplier")]
    pub r#recurrence_multiplier: Box<i32>,
    #[builder(into)]
    #[serde(rename = "shiftCoverages")]
    pub r#shift_coverages: Box<Vec<super::super::types::ssm::GetContactsRotationRecurrenceShiftCoverage>>,
    #[builder(into)]
    #[serde(rename = "weeklySettings")]
    pub r#weekly_settings: Box<Vec<super::super::types::ssm::GetContactsRotationRecurrenceWeeklySetting>>,
}
