#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ContactsRotationRecurrenceWeeklySetting {
    /// (Required) The day of the week when the shift coverage occurs.
    #[builder(into)]
    #[serde(rename = "dayOfWeek")]
    pub r#day_of_week: Box<String>,
    /// (Required) The hand off time. See Hand Off Time for more details.
    #[builder(into, default)]
    #[serde(rename = "handOffTime")]
    pub r#hand_off_time: Box<Option<super::super::types::ssm::ContactsRotationRecurrenceWeeklySettingHandOffTime>>,
}