#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct AttachedClusterMonitoringConfig {
    /// Enable Google Cloud Managed Service for Prometheus in the cluster.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "managedPrometheusConfig")]
    pub r#managed_prometheus_config: Box<Option<super::super::types::container::AttachedClusterMonitoringConfigManagedPrometheusConfig>>,
}
