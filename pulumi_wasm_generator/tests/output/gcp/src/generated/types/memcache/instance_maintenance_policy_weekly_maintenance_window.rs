#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct InstanceMaintenancePolicyWeeklyMaintenanceWindow {
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
    /// Required. The length of the maintenance window, ranging from 3 hours to 8 hours.
    /// A duration in seconds with up to nine fractional digits,
    /// terminated by 's'. Example: "3.5s".
    #[builder(into)]
    #[serde(rename = "duration")]
    pub r#duration: Box<String>,
    /// Required. Start time of the window in UTC time.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "startTime")]
    pub r#start_time: Box<super::super::types::memcache::InstanceMaintenancePolicyWeeklyMaintenanceWindowStartTime>,
}
