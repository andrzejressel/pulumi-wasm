#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct MetastoreServiceMaintenanceWindow {
    /// The day of week, when the window starts.
    /// Possible values are: `MONDAY`, `TUESDAY`, `WEDNESDAY`, `THURSDAY`, `FRIDAY`, `SATURDAY`, `SUNDAY`.
    #[builder(into)]
    #[serde(rename = "dayOfWeek")]
    pub r#day_of_week: Box<String>,
    /// The hour of day (0-23) when the window starts.
    #[builder(into)]
    #[serde(rename = "hourOfDay")]
    pub r#hour_of_day: Box<i32>,
}
