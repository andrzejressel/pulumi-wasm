#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetBackupPlanBackupRuleStandardScheduleBackupWindow {
    /// The hour of the day (1-24) when the window ends, for example, if the value of end hour of the day is 10, that means the backup window end time is 10:00.
    /// The end hour of the day should be greater than the start
    #[builder(into)]
    #[serde(rename = "endHourOfDay")]
    pub r#end_hour_of_day: Box<i32>,
    /// The hour of the day (0-23) when the window starts, for example, if the value of the start hour of the day is 6, that means the backup window starts at 6:00.
    #[builder(into)]
    #[serde(rename = "startHourOfDay")]
    pub r#start_hour_of_day: Box<i32>,
}
