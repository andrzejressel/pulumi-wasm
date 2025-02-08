#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetContactsRotationRecurrenceWeeklySetting {
    #[builder(into)]
    #[serde(rename = "dayOfWeek")]
    pub r#day_of_week: Box<String>,
    #[builder(into)]
    #[serde(rename = "handOffTimes")]
    pub r#hand_off_times: Box<Vec<super::super::types::ssm::GetContactsRotationRecurrenceWeeklySettingHandOffTime>>,
}
