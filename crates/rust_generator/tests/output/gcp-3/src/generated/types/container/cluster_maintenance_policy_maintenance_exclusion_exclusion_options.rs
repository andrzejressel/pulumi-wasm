#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ClusterMaintenancePolicyMaintenanceExclusionExclusionOptions {
    /// The scope of automatic upgrades to restrict in the exclusion window. One of: **NO_UPGRADES | NO_MINOR_UPGRADES | NO_MINOR_OR_NODE_UPGRADES**
    /// 
    /// Specify `start_time` and `end_time` in [RFC3339](https://www.ietf.org/rfc/rfc3339.txt) "Zulu" date format.  The start time's date is
    /// the initial date that the window starts, and the end time is used for calculating duration.Specify `recurrence` in
    /// [RFC5545](https://tools.ietf.org/html/rfc5545#section-3.8.5.3) RRULE format, to specify when this recurs.
    /// Note that GKE may accept other formats, but will return values in UTC, causing a permanent diff.
    /// 
    /// Examples:
    /// 
    /// ```sh
    /// maintenance_policy {
    /// recurring_window {
    /// start_time = "2019-01-01T00:00:00Z"
    /// end_time = "2019-01-02T00:00:00Z"
    /// recurrence = "FREQ=DAILY"
    /// }
    /// maintenance_exclusion{
    /// exclusion_name = "batch job"
    /// start_time = "2019-01-01T00:00:00Z"
    /// end_time = "2019-01-02T00:00:00Z"
    /// exclusion_options {
    /// scope = "NO_UPGRADES"
    /// }
    /// }
    /// maintenance_exclusion{
    /// exclusion_name = "holiday data load"
    /// start_time = "2019-05-01T00:00:00Z"
    /// end_time = "2019-05-02T00:00:00Z"
    /// exclusion_options {
    /// scope = "NO_MINOR_UPGRADES"
    /// }
    /// }
    /// }
    /// ```
    #[builder(into)]
    #[serde(rename = "scope")]
    pub r#scope: Box<String>,
}
