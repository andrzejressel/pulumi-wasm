#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DeliveryPipelineConditionTargetsPresentCondition {
    /// The list of Target names that are missing. For example, projects/{project_id}/locations/{location_name}/targets/{target_name}.
    #[builder(into, default)]
    #[serde(rename = "missingTargets")]
    pub r#missing_targets: Box<Option<Vec<String>>>,
    /// True if there aren't any missing Targets.
    #[builder(into, default)]
    #[serde(rename = "status")]
    pub r#status: Box<Option<bool>>,
    /// Output only. Most recent time at which the pipeline was updated.
    #[builder(into, default)]
    #[serde(rename = "updateTime")]
    pub r#update_time: Box<Option<String>>,
}
