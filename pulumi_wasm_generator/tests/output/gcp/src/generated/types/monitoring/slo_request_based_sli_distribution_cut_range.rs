#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct SloRequestBasedSliDistributionCutRange {
    /// max value for the range (inclusive). If not given,
    /// will be set to "infinity", defining an open range
    /// ">= range.min"
    #[builder(into, default)]
    #[serde(rename = "max")]
    pub r#max: Box<Option<f64>>,
    /// Min value for the range (inclusive). If not given,
    /// will be set to "-infinity", defining an open range
    /// "< range.max"
    #[builder(into, default)]
    #[serde(rename = "min")]
    pub r#min: Box<Option<f64>>,
}
