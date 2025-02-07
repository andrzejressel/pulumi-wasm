#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FleetUpdateStrategyStage {
    /// Specifies the time in seconds to wait at the end of this stage before starting the next one.
    #[builder(into, default)]
    #[serde(rename = "afterStageWaitInSeconds")]
    pub r#after_stage_wait_in_seconds: Box<Option<i32>>,
    /// One or more `group` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "groups")]
    pub r#groups: Box<Vec<super::super::types::containerservice::FleetUpdateStrategyStageGroup>>,
    /// The name which should be used for this stage.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}
