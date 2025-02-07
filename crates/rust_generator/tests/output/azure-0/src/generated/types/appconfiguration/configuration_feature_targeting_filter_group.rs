#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ConfigurationFeatureTargetingFilterGroup {
    /// The name of the group.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Rollout percentage of the group.
    #[builder(into)]
    #[serde(rename = "rolloutPercentage")]
    pub r#rollout_percentage: Box<i32>,
}
