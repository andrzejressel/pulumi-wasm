#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ContactsRotationRecurrenceMonthlySetting {
    /// (Required) The day of the month when monthly recurring on-call rotations begin.
    #[builder(into)]
    #[serde(rename = "dayOfMonth")]
    pub r#day_of_month: Box<i32>,
    /// (Required) The hand off time. See Hand Off Time for more details.
    #[builder(into, default)]
    #[serde(rename = "handOffTime")]
    pub r#hand_off_time: Box<Option<super::super::types::ssm::ContactsRotationRecurrenceMonthlySettingHandOffTime>>,
}
