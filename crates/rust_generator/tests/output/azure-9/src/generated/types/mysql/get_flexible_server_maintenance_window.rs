#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetFlexibleServerMaintenanceWindow {
    /// The day of week of the maintenance window.
    #[builder(into)]
    #[serde(rename = "dayOfWeek")]
    pub r#day_of_week: Box<i32>,
    /// The start hour of the maintenance window.
    #[builder(into)]
    #[serde(rename = "startHour")]
    pub r#start_hour: Box<i32>,
    /// The start minute of the maintenance window.
    #[builder(into)]
    #[serde(rename = "startMinute")]
    pub r#start_minute: Box<i32>,
}
