#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct VirtualNodeSpecListenerOutlierDetection {
    /// Base amount of time for which a host is ejected.
    #[builder(into)]
    #[serde(rename = "baseEjectionDuration")]
    pub r#base_ejection_duration: Box<super::super::types::appmesh::VirtualNodeSpecListenerOutlierDetectionBaseEjectionDuration>,
    /// Time interval between ejection sweep analysis.
    #[builder(into)]
    #[serde(rename = "interval")]
    pub r#interval: Box<super::super::types::appmesh::VirtualNodeSpecListenerOutlierDetectionInterval>,
    /// Maximum percentage of hosts in load balancing pool for upstream service that can be ejected. Will eject at least one host regardless of the value.
    /// Minimum value of `0`. Maximum value of `100`.
    #[builder(into)]
    #[serde(rename = "maxEjectionPercent")]
    pub r#max_ejection_percent: Box<i32>,
    /// Number of consecutive `5xx` errors required for ejection. Minimum value of `1`.
    #[builder(into)]
    #[serde(rename = "maxServerErrors")]
    pub r#max_server_errors: Box<i32>,
}
