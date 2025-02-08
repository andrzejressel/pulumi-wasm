#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct FirewallPolicyInsights {
    /// The ID of the default Log Analytics Workspace that the Firewalls associated with this Firewall Policy will send their logs to, when there is no location matches in the `log_analytics_workspace`.
    #[builder(into)]
    #[serde(rename = "defaultLogAnalyticsWorkspaceId")]
    pub r#default_log_analytics_workspace_id: Box<String>,
    /// Whether the insights functionality is enabled for this Firewall Policy.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<bool>,
    /// A list of `log_analytics_workspace` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "logAnalyticsWorkspaces")]
    pub r#log_analytics_workspaces: Box<Option<Vec<super::super::types::network::FirewallPolicyInsightsLogAnalyticsWorkspace>>>,
    /// The log retention period in days.
    #[builder(into, default)]
    #[serde(rename = "retentionInDays")]
    pub r#retention_in_days: Box<Option<i32>>,
}
