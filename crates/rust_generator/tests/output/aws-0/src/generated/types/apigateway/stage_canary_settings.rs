#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct StageCanarySettings {
    /// ID of the deployment that the canary points to.
    #[builder(into)]
    #[serde(rename = "deploymentId")]
    pub r#deployment_id: Box<String>,
    /// Percent `0.0` - `100.0` of traffic to divert to the canary deployment.
    #[builder(into, default)]
    #[serde(rename = "percentTraffic")]
    pub r#percent_traffic: Box<Option<f64>>,
    /// Map of overridden stage `variables` (including new variables) for the canary deployment.
    #[builder(into, default)]
    #[serde(rename = "stageVariableOverrides")]
    pub r#stage_variable_overrides: Box<Option<std::collections::HashMap<String, String>>>,
    /// Whether the canary deployment uses the stage cache. Defaults to false.
    #[builder(into, default)]
    #[serde(rename = "useStageCache")]
    pub r#use_stage_cache: Box<Option<bool>>,
}
