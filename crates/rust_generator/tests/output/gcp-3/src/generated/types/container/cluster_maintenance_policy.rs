#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ClusterMaintenancePolicy {
    /// Time window specified for daily maintenance operations.
    /// Specify `start_time` in [RFC3339](https://www.ietf.org/rfc/rfc3339.txt) format "HH:MM”,
    /// where HH : \[00-23\] and MM : \[00-59\] GMT. For example:
    /// 
    /// Examples:
    #[builder(into, default)]
    #[serde(rename = "dailyMaintenanceWindow")]
    pub r#daily_maintenance_window: Box<Option<super::super::types::container::ClusterMaintenancePolicyDailyMaintenanceWindow>>,
    /// Exceptions to maintenance window. Non-emergency maintenance should not occur in these windows. A cluster can have up to 20 maintenance exclusions at a time [Maintenance Window and Exclusions](https://cloud.google.com/kubernetes-engine/docs/concepts/maintenance-windows-and-exclusions)
    #[builder(into, default)]
    #[serde(rename = "maintenanceExclusions")]
    pub r#maintenance_exclusions: Box<Option<Vec<super::super::types::container::ClusterMaintenancePolicyMaintenanceExclusion>>>,
    /// Time window for recurring maintenance operations.
    /// 
    /// Specify `start_time` and `end_time` in [RFC3339](https://www.ietf.org/rfc/rfc3339.txt) "Zulu" date format.  The start time's date is
    /// the initial date that the window starts, and the end time is used for calculating duration.  Specify `recurrence` in
    /// [RFC5545](https://tools.ietf.org/html/rfc5545#section-3.8.5.3) RRULE format, to specify when this recurs.
    /// Note that GKE may accept other formats, but will return values in UTC, causing a permanent diff.
    /// 
    /// Examples:
    /// ```sh
    /// maintenance_policy {
    /// recurring_window {
    /// start_time = "2019-08-01T02:00:00Z"
    /// end_time = "2019-08-01T06:00:00Z"
    /// recurrence = "FREQ=DAILY"
    /// }
    /// }
    /// ```
    /// 
    /// ```sh
    /// maintenance_policy {
    /// recurring_window {
    /// start_time = "2019-01-01T09:00:00Z"
    /// end_time = "2019-01-01T17:00:00Z"
    /// recurrence = "FREQ=WEEKLY;BYDAY=MO,TU,WE,TH,FR"
    /// }
    /// }
    /// ```
    #[builder(into, default)]
    #[serde(rename = "recurringWindow")]
    pub r#recurring_window: Box<Option<super::super::types::container::ClusterMaintenancePolicyRecurringWindow>>,
}
