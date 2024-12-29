#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ChannelEncoderSettingsMotionGraphicsConfiguration {
    /// Motion Graphics Insertion.
    #[builder(into, default)]
    #[serde(rename = "motionGraphicsInsertion")]
    pub r#motion_graphics_insertion: Box<Option<String>>,
    /// Motion Graphics Settings. See Motion Graphics Settings for more details.
    #[builder(into)]
    #[serde(rename = "motionGraphicsSettings")]
    pub r#motion_graphics_settings: Box<super::super::types::medialive::ChannelEncoderSettingsMotionGraphicsConfigurationMotionGraphicsSettings>,
}
