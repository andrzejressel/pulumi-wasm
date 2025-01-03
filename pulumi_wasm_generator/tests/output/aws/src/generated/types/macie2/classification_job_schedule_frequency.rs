#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ClassificationJobScheduleFrequency {
    /// Specifies a daily recurrence pattern for running the job.
    #[builder(into, default)]
    #[serde(rename = "dailySchedule")]
    pub r#daily_schedule: Box<Option<bool>>,
    /// Specifies a monthly recurrence pattern for running the job.
    #[builder(into, default)]
    #[serde(rename = "monthlySchedule")]
    pub r#monthly_schedule: Box<Option<i32>>,
    /// Specifies a weekly recurrence pattern for running the job.
    #[builder(into, default)]
    #[serde(rename = "weeklySchedule")]
    pub r#weekly_schedule: Box<Option<String>>,
}
