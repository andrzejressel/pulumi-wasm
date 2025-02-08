#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ClusterMaintenancePolicyWeeklyMaintenanceWindow {
    /// Required. The day of week that maintenance updates occur.
    /// - DAY_OF_WEEK_UNSPECIFIED: The day of the week is unspecified.
    /// - MONDAY: Monday
    /// - TUESDAY: Tuesday
    /// - WEDNESDAY: Wednesday
    /// - THURSDAY: Thursday
    /// - FRIDAY: Friday
    /// - SATURDAY: Saturday
    /// - SUNDAY: Sunday
    /// Possible values are: `DAY_OF_WEEK_UNSPECIFIED`, `MONDAY`, `TUESDAY`, `WEDNESDAY`, `THURSDAY`, `FRIDAY`, `SATURDAY`, `SUNDAY`.
    #[builder(into)]
    #[serde(rename = "day")]
    pub r#day: Box<String>,
    /// (Output)
    /// Output only. Duration of the maintenance window.
    /// The current window is fixed at 1 hour.
    /// A duration in seconds with up to nine fractional digits,
    /// terminated by 's'. Example: "3.5s".
    #[builder(into, default)]
    #[serde(rename = "duration")]
    pub r#duration: Box<Option<String>>,
    /// Required. Start time of the window in UTC time.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "startTime")]
    pub r#start_time: Box<super::super::types::redis::ClusterMaintenancePolicyWeeklyMaintenanceWindowStartTime>,
}
