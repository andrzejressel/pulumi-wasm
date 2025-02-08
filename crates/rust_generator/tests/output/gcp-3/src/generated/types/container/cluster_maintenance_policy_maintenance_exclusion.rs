#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ClusterMaintenancePolicyMaintenanceExclusion {
    #[builder(into)]
    #[serde(rename = "endTime")]
    pub r#end_time: Box<String>,
    #[builder(into)]
    #[serde(rename = "exclusionName")]
    pub r#exclusion_name: Box<String>,
    /// MaintenanceExclusionOptions provides maintenance exclusion related options.
    #[builder(into, default)]
    #[serde(rename = "exclusionOptions")]
    pub r#exclusion_options: Box<Option<super::super::types::container::ClusterMaintenancePolicyMaintenanceExclusionExclusionOptions>>,
    #[builder(into)]
    #[serde(rename = "startTime")]
    pub r#start_time: Box<String>,
}
