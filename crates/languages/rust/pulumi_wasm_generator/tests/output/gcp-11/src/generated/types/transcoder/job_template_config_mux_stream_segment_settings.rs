#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct JobTemplateConfigMuxStreamSegmentSettings {
    /// Duration of the segments in seconds. The default is `6.0s`.
    #[builder(into, default)]
    #[serde(rename = "segmentDuration")]
    pub r#segment_duration: Box<Option<String>>,
}
