#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ContactsRotationRecurrence {
    #[builder(into, default)]
    #[serde(rename = "dailySettings")]
    pub r#daily_settings: Box<Option<Vec<super::super::types::ssm::ContactsRotationRecurrenceDailySetting>>>,
    /// (Optional) Information about on-call rotations that recur monthly. See Monthly Settings for more details.
    #[builder(into, default)]
    #[serde(rename = "monthlySettings")]
    pub r#monthly_settings: Box<Option<Vec<super::super::types::ssm::ContactsRotationRecurrenceMonthlySetting>>>,
    /// (Required) The number of contacts, or shift team members designated to be on call concurrently during a shift.
    #[builder(into)]
    #[serde(rename = "numberOfOnCalls")]
    pub r#number_of_on_calls: Box<i32>,
    /// (Required) The number of days, weeks, or months a single rotation lasts.
    #[builder(into)]
    #[serde(rename = "recurrenceMultiplier")]
    pub r#recurrence_multiplier: Box<i32>,
    /// (Optional) Information about the days of the week that the on-call rotation coverage includes. See Shift Coverages for more details.
    #[builder(into, default)]
    #[serde(rename = "shiftCoverages")]
    pub r#shift_coverages: Box<Option<Vec<super::super::types::ssm::ContactsRotationRecurrenceShiftCoverage>>>,
    /// (Optional) Information about on-call rotations that recur weekly. See Weekly Settings for more details.
    #[builder(into, default)]
    #[serde(rename = "weeklySettings")]
    pub r#weekly_settings: Box<Option<Vec<super::super::types::ssm::ContactsRotationRecurrenceWeeklySetting>>>,
}
