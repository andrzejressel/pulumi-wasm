#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct DeploymentCanarySettings {
    /// Percentage (0.0-100.0) of traffic routed to the canary deployment.
    #[builder(into, default)]
    #[serde(rename = "percentTraffic")]
    pub r#percent_traffic: Box<Option<f64>>,
    /// Stage variable overrides used for the canary release deployment. They can override existing stage variables or add new stage variables for the canary release deployment. These stage variables are represented as a string-to-string map between stage variable names and their values.
    #[builder(into, default)]
    #[serde(rename = "stageVariableOverrides")]
    pub r#stage_variable_overrides: Box<Option<std::collections::HashMap<String, String>>>,
    /// Boolean flag to indicate whether the canary release deployment uses the stage cache or not.
    #[builder(into, default)]
    #[serde(rename = "useStageCache")]
    pub r#use_stage_cache: Box<Option<bool>>,
}
