#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetVirtualNodeSpecListenerOutlierDetection {
    #[builder(into)]
    #[serde(rename = "baseEjectionDurations")]
    pub r#base_ejection_durations: Box<Vec<super::super::types::appmesh::GetVirtualNodeSpecListenerOutlierDetectionBaseEjectionDuration>>,
    #[builder(into)]
    #[serde(rename = "intervals")]
    pub r#intervals: Box<Vec<super::super::types::appmesh::GetVirtualNodeSpecListenerOutlierDetectionInterval>>,
    #[builder(into)]
    #[serde(rename = "maxEjectionPercent")]
    pub r#max_ejection_percent: Box<i32>,
    #[builder(into)]
    #[serde(rename = "maxServerErrors")]
    pub r#max_server_errors: Box<i32>,
}
