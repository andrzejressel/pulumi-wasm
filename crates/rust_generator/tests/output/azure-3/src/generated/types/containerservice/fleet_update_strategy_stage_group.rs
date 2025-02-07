#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FleetUpdateStrategyStageGroup {
    /// The name which should be used for this group.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}
