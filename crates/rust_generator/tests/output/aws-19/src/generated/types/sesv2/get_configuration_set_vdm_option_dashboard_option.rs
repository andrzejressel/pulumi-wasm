#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetConfigurationSetVdmOptionDashboardOption {
    /// Specifies the status of your VDM engagement metrics collection.
    #[builder(into)]
    #[serde(rename = "engagementMetrics")]
    pub r#engagement_metrics: Box<String>,
}
