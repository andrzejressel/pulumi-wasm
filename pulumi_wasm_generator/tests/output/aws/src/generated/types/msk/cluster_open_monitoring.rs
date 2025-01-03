#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ClusterOpenMonitoring {
    /// Configuration block for Prometheus settings for open monitoring. See below.
    #[builder(into)]
    #[serde(rename = "prometheus")]
    pub r#prometheus: Box<super::super::types::msk::ClusterOpenMonitoringPrometheus>,
}
