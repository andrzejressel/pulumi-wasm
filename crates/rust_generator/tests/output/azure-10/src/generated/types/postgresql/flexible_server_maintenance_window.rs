#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FlexibleServerMaintenanceWindow {
    /// The day of week for maintenance window, where the week starts on a Sunday, i.e. Sunday = `0`, Monday = `1`. Defaults to `0`.
    #[builder(into, default)]
    #[serde(rename = "dayOfWeek")]
    pub r#day_of_week: Box<Option<i32>>,
    /// The start hour for maintenance window. Defaults to `0`.
    #[builder(into, default)]
    #[serde(rename = "startHour")]
    pub r#start_hour: Box<Option<i32>>,
    /// The start minute for maintenance window. Defaults to `0`.
    /// 
    /// > **NOTE** The specified `maintenance_window` is always defined in UTC time. When unspecified, the maintenance window falls back to the default [system-managed](https://learn.microsoft.com/en-us/azure/postgresql/flexible-server/how-to-maintenance-portal#specify-maintenance-schedule-options).
    #[builder(into, default)]
    #[serde(rename = "startMinute")]
    pub r#start_minute: Box<Option<i32>>,
}
