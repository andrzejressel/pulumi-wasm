#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetDetectorFeatureAdditionalConfiguration {
    /// The name of the detector feature.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Current status of the detector.
    #[builder(into)]
    #[serde(rename = "status")]
    pub r#status: Box<String>,
}