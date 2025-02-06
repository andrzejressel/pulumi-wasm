#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetKubernetesClusterOmsAgent {
    /// The ID of the Log Analytics Workspace to which the OMS Agent should send data.
    #[builder(into)]
    #[serde(rename = "logAnalyticsWorkspaceId")]
    pub r#log_analytics_workspace_id: Box<String>,
    /// Is managed identity authentication for monitoring enabled?
    #[builder(into)]
    #[serde(rename = "msiAuthForMonitoringEnabled")]
    pub r#msi_auth_for_monitoring_enabled: Box<bool>,
    /// An `oms_agent_identity` block as defined below.
    #[builder(into)]
    #[serde(rename = "omsAgentIdentities")]
    pub r#oms_agent_identities: Box<Vec<super::super::types::containerservice::GetKubernetesClusterOmsAgentOmsAgentIdentity>>,
}
