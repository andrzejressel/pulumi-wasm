#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ChannelInputAttachmentInputSettingsCaptionSelectorSelectorSettingsEmbeddedSourceSettings {
    /// If upconvert, 608 data is both passed through via the “608 compatibility bytes” fields of the 708 wrapper as well as translated into 708. 708 data present in the source content will be discarded.
    #[builder(into, default)]
    #[serde(rename = "convert608To708")]
    pub r#convert_608_to_708: Box<Option<String>>,
    /// Set to “auto” to handle streams with intermittent and/or non-aligned SCTE-20 and Embedded captions.
    #[builder(into, default)]
    #[serde(rename = "scte20Detection")]
    pub r#scte_20_detection: Box<Option<String>>,
    /// Specifies the 608/708 channel number within the video track from which to extract captions. Unused for passthrough.
    #[builder(into, default)]
    #[serde(rename = "source608ChannelNumber")]
    pub r#source_608_channel_number: Box<Option<i32>>,
}
