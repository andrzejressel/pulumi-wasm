#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DomainOffPeakWindowOptions {
    /// Enabled disabled toggle for off-peak update window.
    #[builder(into, default)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
    #[builder(into, default)]
    #[serde(rename = "offPeakWindow")]
    pub r#off_peak_window: Box<Option<super::super::types::opensearch::DomainOffPeakWindowOptionsOffPeakWindow>>,
}