#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ChannelEncoderSettingsGlobalConfigurationInputLossBehavior {
    #[builder(into, default)]
    #[serde(rename = "blackFrameMsec")]
    pub r#black_frame_msec: Box<Option<i32>>,
    #[builder(into, default)]
    #[serde(rename = "inputLossImageColor")]
    pub r#input_loss_image_color: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "inputLossImageSlate")]
    pub r#input_loss_image_slate: Box<Option<super::super::types::medialive::ChannelEncoderSettingsGlobalConfigurationInputLossBehaviorInputLossImageSlate>>,
    #[builder(into, default)]
    #[serde(rename = "inputLossImageType")]
    pub r#input_loss_image_type: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "repeatFrameMsec")]
    pub r#repeat_frame_msec: Box<Option<i32>>,
}