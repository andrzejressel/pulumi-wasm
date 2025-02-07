#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ClusterOpenMonitoringPrometheusJmxExporter {
    /// Indicates whether you want to enable or disable the Node Exporter.
    #[builder(into)]
    #[serde(rename = "enabledInBroker")]
    pub r#enabled_in_broker: Box<bool>,
}
