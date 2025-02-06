#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct HBaseClusterMonitor {
    /// The Operations Management Suite (OMS) workspace ID.
    #[builder(into)]
    #[serde(rename = "logAnalyticsWorkspaceId")]
    pub r#log_analytics_workspace_id: Box<String>,
    /// The Operations Management Suite (OMS) workspace key.
    #[builder(into)]
    #[serde(rename = "primaryKey")]
    pub r#primary_key: Box<String>,
}
