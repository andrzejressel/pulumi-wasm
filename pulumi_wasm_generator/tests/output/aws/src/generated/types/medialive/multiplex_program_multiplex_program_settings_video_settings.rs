#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct MultiplexProgramMultiplexProgramSettingsVideoSettings {
    /// Constant bitrate value.
    #[builder(into, default)]
    #[serde(rename = "constantBitrate")]
    pub r#constant_bitrate: Box<Option<i32>>,
    /// Statmux settings. See Statmux Settings for more details.
    #[builder(into, default)]
    #[serde(rename = "statmuxSettings")]
    pub r#statmux_settings: Box<Option<super::super::types::medialive::MultiplexProgramMultiplexProgramSettingsVideoSettingsStatmuxSettings>>,
}