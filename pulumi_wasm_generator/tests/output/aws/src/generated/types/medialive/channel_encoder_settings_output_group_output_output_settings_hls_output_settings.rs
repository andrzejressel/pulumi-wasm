#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ChannelEncoderSettingsOutputGroupOutputOutputSettingsHlsOutputSettings {
    #[builder(into, default)]
    #[serde(rename = "h265PackagingType")]
    pub r#h_265_packaging_type: Box<Option<String>>,
    #[builder(into)]
    #[serde(rename = "hlsSettings")]
    pub r#hls_settings: Box<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputOutputSettingsHlsOutputSettingsHlsSettings>,
    /// String concatenated to the end of the destination filename. Required for multiple outputs of the same type.
    #[builder(into, default)]
    #[serde(rename = "nameModifier")]
    pub r#name_modifier: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "segmentModifier")]
    pub r#segment_modifier: Box<Option<String>>,
}
