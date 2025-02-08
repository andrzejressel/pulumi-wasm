#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ClusterMaintenanceUpdatePolicyMaintenanceWindow {
    /// Preferred day of the week for maintenance, e.g. MONDAY, TUESDAY, etc.
    /// Possible values are: `MONDAY`, `TUESDAY`, `WEDNESDAY`, `THURSDAY`, `FRIDAY`, `SATURDAY`, `SUNDAY`.
    #[builder(into)]
    #[serde(rename = "day")]
    pub r#day: Box<String>,
    /// Preferred time to start the maintenance operation on the specified day. Maintenance will start within 1 hour of this time.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "startTime")]
    pub r#start_time: Box<super::super::types::alloydb::ClusterMaintenanceUpdatePolicyMaintenanceWindowStartTime>,
}
