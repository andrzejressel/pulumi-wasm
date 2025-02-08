#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetContactsRotationRecurrenceMonthlySetting {
    #[builder(into)]
    #[serde(rename = "dayOfMonth")]
    pub r#day_of_month: Box<i32>,
    #[builder(into)]
    #[serde(rename = "handOffTimes")]
    pub r#hand_off_times: Box<Vec<super::super::types::ssm::GetContactsRotationRecurrenceMonthlySettingHandOffTime>>,
}
