#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetProfilingGroupProfilingStatus {
    #[builder(into)]
    #[serde(rename = "latestAgentOrchestratedAt")]
    pub r#latest_agent_orchestrated_at: Box<String>,
    #[builder(into)]
    #[serde(rename = "latestAgentProfileReportedAt")]
    pub r#latest_agent_profile_reported_at: Box<String>,
    #[builder(into)]
    #[serde(rename = "latestAggregatedProfiles")]
    pub r#latest_aggregated_profiles: Box<Vec<super::super::types::codeguruprofiler::GetProfilingGroupProfilingStatusLatestAggregatedProfile>>,
}
