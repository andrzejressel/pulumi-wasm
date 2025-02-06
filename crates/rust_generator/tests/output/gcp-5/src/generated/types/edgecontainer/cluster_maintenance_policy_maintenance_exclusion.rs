#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ClusterMaintenancePolicyMaintenanceExclusion {
    /// A unique (per cluster) id for the window.
    #[builder(into, default)]
    #[serde(rename = "id")]
    pub r#id: Box<Option<String>>,
    /// Represents an arbitrary window of time.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "window")]
    pub r#window: Box<Option<super::super::types::edgecontainer::ClusterMaintenancePolicyMaintenanceExclusionWindow>>,
}
