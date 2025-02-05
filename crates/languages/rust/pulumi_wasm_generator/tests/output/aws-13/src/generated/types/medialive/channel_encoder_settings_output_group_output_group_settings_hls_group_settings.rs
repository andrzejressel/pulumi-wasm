#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ChannelEncoderSettingsOutputGroupOutputGroupSettingsHlsGroupSettings {
    /// The ad marker type for this output group.
    #[builder(into, default)]
    #[serde(rename = "adMarkers")]
    pub r#ad_markers: Box<Option<Vec<String>>>,
    #[builder(into, default)]
    #[serde(rename = "baseUrlContent")]
    pub r#base_url_content: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "baseUrlContent1")]
    pub r#base_url_content_1: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "baseUrlManifest")]
    pub r#base_url_manifest: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "baseUrlManifest1")]
    pub r#base_url_manifest_1: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "captionLanguageMappings")]
    pub r#caption_language_mappings: Box<Option<Vec<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputGroupSettingsHlsGroupSettingsCaptionLanguageMapping>>>,
    #[builder(into, default)]
    #[serde(rename = "captionLanguageSetting")]
    pub r#caption_language_setting: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "clientCache")]
    pub r#client_cache: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "codecSpecification")]
    pub r#codec_specification: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "constantIv")]
    pub r#constant_iv: Box<Option<String>>,
    #[builder(into)]
    #[serde(rename = "destination")]
    pub r#destination: Box<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputGroupSettingsHlsGroupSettingsDestination>,
    #[builder(into, default)]
    #[serde(rename = "directoryStructure")]
    pub r#directory_structure: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "discontinuityTags")]
    pub r#discontinuity_tags: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "encryptionType")]
    pub r#encryption_type: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "hlsCdnSettings")]
    pub r#hls_cdn_settings: Box<Option<Vec<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputGroupSettingsHlsGroupSettingsHlsCdnSetting>>>,
    #[builder(into, default)]
    #[serde(rename = "hlsId3SegmentTagging")]
    pub r#hls_id_3_segment_tagging: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "iframeOnlyPlaylists")]
    pub r#iframe_only_playlists: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "incompleteSegmentBehavior")]
    pub r#incomplete_segment_behavior: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "indexNSegments")]
    pub r#index_n_segments: Box<Option<i32>>,
    #[builder(into, default)]
    #[serde(rename = "inputLossAction")]
    pub r#input_loss_action: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "ivInManifest")]
    pub r#iv_in_manifest: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "ivSource")]
    pub r#iv_source: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "keepSegments")]
    pub r#keep_segments: Box<Option<i32>>,
    #[builder(into, default)]
    #[serde(rename = "keyFormat")]
    pub r#key_format: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "keyFormatVersions")]
    pub r#key_format_versions: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "keyProviderSettings")]
    pub r#key_provider_settings: Box<Option<super::super::types::medialive::ChannelEncoderSettingsOutputGroupOutputGroupSettingsHlsGroupSettingsKeyProviderSettings>>,
    #[builder(into, default)]
    #[serde(rename = "manifestCompression")]
    pub r#manifest_compression: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "manifestDurationFormat")]
    pub r#manifest_duration_format: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "minSegmentLength")]
    pub r#min_segment_length: Box<Option<i32>>,
    #[builder(into, default)]
    #[serde(rename = "mode")]
    pub r#mode: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "outputSelection")]
    pub r#output_selection: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "programDateTime")]
    pub r#program_date_time: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "programDateTimeClock")]
    pub r#program_date_time_clock: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "programDateTimePeriod")]
    pub r#program_date_time_period: Box<Option<i32>>,
    #[builder(into, default)]
    #[serde(rename = "redundantManifest")]
    pub r#redundant_manifest: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "segmentLength")]
    pub r#segment_length: Box<Option<i32>>,
    #[builder(into, default)]
    #[serde(rename = "segmentsPerSubdirectory")]
    pub r#segments_per_subdirectory: Box<Option<i32>>,
    #[builder(into, default)]
    #[serde(rename = "streamInfResolution")]
    pub r#stream_inf_resolution: Box<Option<String>>,
    /// Indicates ID3 frame that has the timecode.
    #[builder(into, default)]
    #[serde(rename = "timedMetadataId3Frame")]
    pub r#timed_metadata_id_3_frame: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "timedMetadataId3Period")]
    pub r#timed_metadata_id_3_period: Box<Option<i32>>,
    #[builder(into, default)]
    #[serde(rename = "timestampDeltaMilliseconds")]
    pub r#timestamp_delta_milliseconds: Box<Option<i32>>,
    #[builder(into, default)]
    #[serde(rename = "tsFileMode")]
    pub r#ts_file_mode: Box<Option<String>>,
}
