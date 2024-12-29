#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ChannelEncoderSettingsOutputGroupOutputGroupSettingsUdpGroupSettings {
    /// Specifies behavior of last resort when input video os lost.
    #[builder(into, default)]
    #[serde(rename = "inputLossAction")]
    pub r#input_loss_action: Box<Option<String>>,
    /// Indicates ID3 frame that has the timecode.
    #[builder(into, default)]
    #[serde(rename = "timedMetadataId3Frame")]
    pub r#timed_metadata_id_3_frame: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "timedMetadataId3Period")]
    pub r#timed_metadata_id_3_period: Box<Option<i32>>,
}
