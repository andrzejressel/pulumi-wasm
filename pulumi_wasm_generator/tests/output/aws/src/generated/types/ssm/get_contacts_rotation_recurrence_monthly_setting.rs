#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetContactsRotationRecurrenceMonthlySetting {
    #[builder(into)]
    #[serde(rename = "dayOfMonth")]
    pub r#day_of_month: Box<i32>,
    #[builder(into)]
    #[serde(rename = "handOffTimes")]
    pub r#hand_off_times: Box<Vec<super::super::types::ssm::GetContactsRotationRecurrenceMonthlySettingHandOffTime>>,
}