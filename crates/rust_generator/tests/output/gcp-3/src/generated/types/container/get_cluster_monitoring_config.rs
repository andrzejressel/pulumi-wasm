#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetClusterMonitoringConfig {
    /// Configuration of Advanced Datapath Observability features.
    #[builder(into)]
    #[serde(rename = "advancedDatapathObservabilityConfigs")]
    pub r#advanced_datapath_observability_configs: Box<Vec<super::super::types::container::GetClusterMonitoringConfigAdvancedDatapathObservabilityConfig>>,
    /// GKE components exposing metrics. Valid values include SYSTEM_COMPONENTS, APISERVER, SCHEDULER, CONTROLLER_MANAGER, STORAGE, HPA, POD, DAEMONSET, DEPLOYMENT, STATEFULSET, WORKLOADS, KUBELET, CADVISOR and DCGM.
    #[builder(into)]
    #[serde(rename = "enableComponents")]
    pub r#enable_components: Box<Vec<String>>,
    /// Configuration for Google Cloud Managed Services for Prometheus.
    #[builder(into)]
    #[serde(rename = "managedPrometheuses")]
    pub r#managed_prometheuses: Box<Vec<super::super::types::container::GetClusterMonitoringConfigManagedPrometheus>>,
}
