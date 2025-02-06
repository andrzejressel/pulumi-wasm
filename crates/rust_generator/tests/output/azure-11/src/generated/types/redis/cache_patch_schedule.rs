#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct CachePatchSchedule {
    /// the Weekday name - possible values include `Monday`, `Tuesday`, `Wednesday` etc.
    #[builder(into)]
    #[serde(rename = "dayOfWeek")]
    pub r#day_of_week: Box<String>,
    /// The ISO 8601 timespan which specifies the amount of time the Redis Cache can be updated. Defaults to `PT5H`.
    #[builder(into, default)]
    #[serde(rename = "maintenanceWindow")]
    pub r#maintenance_window: Box<Option<String>>,
    /// the Start Hour for maintenance in UTC - possible values range from `0 - 23`.
    /// 
    /// > **Note:** The Patch Window lasts for `5` hours from the `start_hour_utc`.
    #[builder(into, default)]
    #[serde(rename = "startHourUtc")]
    pub r#start_hour_utc: Box<Option<i32>>,
}
