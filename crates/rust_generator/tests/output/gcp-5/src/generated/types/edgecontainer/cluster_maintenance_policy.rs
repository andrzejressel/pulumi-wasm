#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ClusterMaintenancePolicy {
    /// Exclusions to automatic maintenance. Non-emergency maintenance should not occur
    /// in these windows. Each exclusion has a unique name and may be active or expired.
    /// The max number of maintenance exclusions allowed at a given time is 3.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "maintenanceExclusions")]
    pub r#maintenance_exclusions: Box<Option<Vec<super::super::types::edgecontainer::ClusterMaintenancePolicyMaintenanceExclusion>>>,
    /// Specifies the maintenance window in which maintenance may be performed.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "window")]
    pub r#window: Box<super::super::types::edgecontainer::ClusterMaintenancePolicyWindow>,
}
