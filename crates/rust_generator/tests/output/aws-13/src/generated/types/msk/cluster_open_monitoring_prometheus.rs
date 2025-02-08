#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ClusterOpenMonitoringPrometheus {
    /// Configuration block for JMX Exporter. See below.
    #[builder(into, default)]
    #[serde(rename = "jmxExporter")]
    pub r#jmx_exporter: Box<Option<super::super::types::msk::ClusterOpenMonitoringPrometheusJmxExporter>>,
    /// Configuration block for Node Exporter. See below.
    #[builder(into, default)]
    #[serde(rename = "nodeExporter")]
    pub r#node_exporter: Box<Option<super::super::types::msk::ClusterOpenMonitoringPrometheusNodeExporter>>,
}
