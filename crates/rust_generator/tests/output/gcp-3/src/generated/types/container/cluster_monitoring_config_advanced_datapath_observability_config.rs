#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ClusterMonitoringConfigAdvancedDatapathObservabilityConfig {
    /// Whether or not to enable advanced datapath metrics.
    #[builder(into)]
    #[serde(rename = "enableMetrics")]
    pub r#enable_metrics: Box<bool>,
    /// Whether or not Relay is enabled.
    #[builder(into)]
    #[serde(rename = "enableRelay")]
    pub r#enable_relay: Box<bool>,
}
