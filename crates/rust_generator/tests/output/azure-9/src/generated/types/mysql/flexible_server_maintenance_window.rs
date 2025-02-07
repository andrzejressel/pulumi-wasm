#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FlexibleServerMaintenanceWindow {
    /// The day of week for maintenance window. Defaults to `0`.
    #[builder(into, default)]
    #[serde(rename = "dayOfWeek")]
    pub r#day_of_week: Box<Option<i32>>,
    /// The start hour for maintenance window. Defaults to `0`.
    #[builder(into, default)]
    #[serde(rename = "startHour")]
    pub r#start_hour: Box<Option<i32>>,
    /// The start minute for maintenance window. Defaults to `0`.
    #[builder(into, default)]
    #[serde(rename = "startMinute")]
    pub r#start_minute: Box<Option<i32>>,
}
