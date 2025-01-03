#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ChannelInputAttachmentInputSettings {
    /// Used to select the audio stream to decode for inputs that have multiple. See Audio Selectors for more details.
    #[builder(into, default)]
    #[serde(rename = "audioSelectors")]
    pub r#audio_selectors: Box<Option<Vec<super::super::types::medialive::ChannelInputAttachmentInputSettingsAudioSelector>>>,
    /// Used to select the caption input to use for inputs that have multiple available. See Caption Selectors for more details.
    #[builder(into, default)]
    #[serde(rename = "captionSelectors")]
    pub r#caption_selectors: Box<Option<Vec<super::super::types::medialive::ChannelInputAttachmentInputSettingsCaptionSelector>>>,
    /// Enable or disable the deblock filter when filtering.
    #[builder(into, default)]
    #[serde(rename = "deblockFilter")]
    pub r#deblock_filter: Box<Option<String>>,
    /// Enable or disable the denoise filter when filtering.
    #[builder(into, default)]
    #[serde(rename = "denoiseFilter")]
    pub r#denoise_filter: Box<Option<String>>,
    /// Adjusts the magnitude of filtering from 1 (minimal) to 5 (strongest).
    #[builder(into, default)]
    #[serde(rename = "filterStrength")]
    pub r#filter_strength: Box<Option<i32>>,
    /// Turns on the filter for the input.
    #[builder(into, default)]
    #[serde(rename = "inputFilter")]
    pub r#input_filter: Box<Option<String>>,
    /// Input settings. See Network Input Settings for more details.
    #[builder(into, default)]
    #[serde(rename = "networkInputSettings")]
    pub r#network_input_settings: Box<Option<super::super::types::medialive::ChannelInputAttachmentInputSettingsNetworkInputSettings>>,
    /// PID from which to read SCTE-35 messages.
    #[builder(into, default)]
    #[serde(rename = "scte35Pid")]
    pub r#scte_35_pid: Box<Option<i32>>,
    /// Specifies whether to extract applicable ancillary data from a SMPTE-2038 source in the input.
    #[builder(into, default)]
    #[serde(rename = "smpte2038DataPreference")]
    pub r#smpte_2038_data_preference: Box<Option<String>>,
    /// Loop input if it is a file.
    #[builder(into, default)]
    #[serde(rename = "sourceEndBehavior")]
    pub r#source_end_behavior: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "videoSelector")]
    pub r#video_selector: Box<Option<super::super::types::medialive::ChannelInputAttachmentInputSettingsVideoSelector>>,
}
