#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetInstanceMaintenancePolicyWeeklyMaintenanceWindow {
    /// Required. The day of week that maintenance updates occur.
    /// 
    /// - DAY_OF_WEEK_UNSPECIFIED: The day of the week is unspecified.
    /// - MONDAY: Monday
    /// - TUESDAY: Tuesday
    /// - WEDNESDAY: Wednesday
    /// - THURSDAY: Thursday
    /// - FRIDAY: Friday
    /// - SATURDAY: Saturday
    /// - SUNDAY: Sunday Possible values: ["DAY_OF_WEEK_UNSPECIFIED", "MONDAY", "TUESDAY", "WEDNESDAY", "THURSDAY", "FRIDAY", "SATURDAY", "SUNDAY"]
    #[builder(into)]
    #[serde(rename = "day")]
    pub r#day: Box<String>,
    /// Output only. Duration of the maintenance window.
    /// The current window is fixed at 1 hour.
    /// A duration in seconds with up to nine fractional digits,
    /// terminated by 's'. Example: "3.5s".
    #[builder(into)]
    #[serde(rename = "duration")]
    pub r#duration: Box<String>,
    /// Required. Start time of the window in UTC time.
    #[builder(into)]
    #[serde(rename = "startTimes")]
    pub r#start_times: Box<Vec<super::super::types::redis::GetInstanceMaintenancePolicyWeeklyMaintenanceWindowStartTime>>,
}
