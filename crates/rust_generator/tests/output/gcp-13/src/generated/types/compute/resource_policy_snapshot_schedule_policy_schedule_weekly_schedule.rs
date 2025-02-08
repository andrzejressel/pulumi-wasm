#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ResourcePolicySnapshotSchedulePolicyScheduleWeeklySchedule {
    /// May contain up to seven (one for each day of the week) snapshot times.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "dayOfWeeks")]
    pub r#day_of_weeks: Box<Vec<super::super::types::compute::ResourcePolicySnapshotSchedulePolicyScheduleWeeklyScheduleDayOfWeek>>,
}
