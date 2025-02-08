#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetConfigurationSetVdmOptionDashboardOption {
    /// Specifies the status of your VDM engagement metrics collection.
    #[builder(into)]
    #[serde(rename = "engagementMetrics")]
    pub r#engagement_metrics: Box<String>,
}
