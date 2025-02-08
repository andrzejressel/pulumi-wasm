#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetCachePatchSchedule {
    /// the Weekday name for the patch item
    #[builder(into)]
    #[serde(rename = "dayOfWeek")]
    pub r#day_of_week: Box<String>,
    /// The ISO 8601 timespan which specifies the amount of time the Redis Cache can be updated.
    #[builder(into)]
    #[serde(rename = "maintenanceWindow")]
    pub r#maintenance_window: Box<String>,
    /// The Start Hour for maintenance in UTC
    #[builder(into)]
    #[serde(rename = "startHourUtc")]
    pub r#start_hour_utc: Box<i32>,
}
