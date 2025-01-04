#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ChannelEncoderSettingsAudioDescriptionAudioWatermarkSettingsNielsenWatermarksSettings {
    /// Used to insert watermarks of type Nielsen CBET. See Nielsen CBET Settings for more details.
    #[builder(into, default)]
    #[serde(rename = "nielsenCbetSettings")]
    pub r#nielsen_cbet_settings: Box<Option<super::super::types::medialive::ChannelEncoderSettingsAudioDescriptionAudioWatermarkSettingsNielsenWatermarksSettingsNielsenCbetSettings>>,
    /// Distribution types to assign to the watermarks. Options are `PROGRAM_CONTENT` and `FINAL_DISTRIBUTOR`.
    #[builder(into, default)]
    #[serde(rename = "nielsenDistributionType")]
    pub r#nielsen_distribution_type: Box<Option<String>>,
    /// Used to insert watermarks of type Nielsen NAES, II (N2) and Nielsen NAES VI (NW). See Nielsen NAES II NW Settings for more details.
    #[builder(into, default)]
    #[serde(rename = "nielsenNaesIiNwSettings")]
    pub r#nielsen_naes_ii_nw_settings: Box<Option<Vec<super::super::types::medialive::ChannelEncoderSettingsAudioDescriptionAudioWatermarkSettingsNielsenWatermarksSettingsNielsenNaesIiNwSetting>>>,
}
