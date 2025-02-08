#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ClusterMaintenanceUpdatePolicy {
    /// Preferred windows to perform maintenance. Currently limited to 1.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "maintenanceWindows")]
    pub r#maintenance_windows: Box<Option<Vec<super::super::types::alloydb::ClusterMaintenanceUpdatePolicyMaintenanceWindow>>>,
}
