#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetProfilingGroupAgentOrchestrationConfig {
    #[builder(into)]
    #[serde(rename = "profilingEnabled")]
    pub r#profiling_enabled: Box<bool>,
}
