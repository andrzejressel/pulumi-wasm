#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ChannelEncoderSettingsVideoDescriptionCodecSettingsH265Settings {
    /// Enables or disables adaptive quantization.
    #[builder(into, default)]
    #[serde(rename = "adaptiveQuantization")]
    pub r#adaptive_quantization: Box<Option<String>>,
    /// Indicates that AFD values will be written into the output stream.
    #[builder(into, default)]
    #[serde(rename = "afdSignaling")]
    pub r#afd_signaling: Box<Option<String>>,
    /// Whether or not EML should insert an Alternative Transfer Function SEI message.
    #[builder(into, default)]
    #[serde(rename = "alternativeTransferFunction")]
    pub r#alternative_transfer_function: Box<Option<String>>,
    /// Average bitrate in bits/second.
    #[builder(into)]
    #[serde(rename = "bitrate")]
    pub r#bitrate: Box<i32>,
    /// Size of buffer in bits.
    #[builder(into, default)]
    #[serde(rename = "bufSize")]
    pub r#buf_size: Box<Option<i32>>,
    /// Includes color space metadata in the output.
    #[builder(into, default)]
    #[serde(rename = "colorMetadata")]
    pub r#color_metadata: Box<Option<String>>,
    /// Define the color metadata for the output. H265 Color Space Settings for more details.
    #[builder(into, default)]
    #[serde(rename = "colorSpaceSettings")]
    pub r#color_space_settings: Box<Option<super::super::types::medialive::ChannelEncoderSettingsVideoDescriptionCodecSettingsH265SettingsColorSpaceSettings>>,
    /// Filters to apply to an encode. See H265 Filter Settings for more details.
    #[builder(into, default)]
    #[serde(rename = "filterSettings")]
    pub r#filter_settings: Box<Option<super::super::types::medialive::ChannelEncoderSettingsVideoDescriptionCodecSettingsH265SettingsFilterSettings>>,
    /// Four bit AFD value to write on all frames of video in the output stream.
    #[builder(into, default)]
    #[serde(rename = "fixedAfd")]
    pub r#fixed_afd: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "flickerAq")]
    pub r#flicker_aq: Box<Option<String>>,
    /// Framerate denominator.
    #[builder(into)]
    #[serde(rename = "framerateDenominator")]
    pub r#framerate_denominator: Box<i32>,
    /// Framerate numerator.
    #[builder(into)]
    #[serde(rename = "framerateNumerator")]
    pub r#framerate_numerator: Box<i32>,
    /// Frequency of closed GOPs.
    #[builder(into, default)]
    #[serde(rename = "gopClosedCadence")]
    pub r#gop_closed_cadence: Box<Option<i32>>,
    /// GOP size in units of either frames of seconds per `gop_size_units`.
    #[builder(into, default)]
    #[serde(rename = "gopSize")]
    pub r#gop_size: Box<Option<f64>>,
    /// Indicates if the `gop_size` is specified in frames or seconds.
    #[builder(into, default)]
    #[serde(rename = "gopSizeUnits")]
    pub r#gop_size_units: Box<Option<String>>,
    /// H265 level.
    #[builder(into, default)]
    #[serde(rename = "level")]
    pub r#level: Box<Option<String>>,
    /// Amount of lookahead.
    #[builder(into, default)]
    #[serde(rename = "lookAheadRateControl")]
    pub r#look_ahead_rate_control: Box<Option<String>>,
    /// Set the maximum bitrate in order to accommodate expected spikes in the complexity of the video.
    #[builder(into, default)]
    #[serde(rename = "maxBitrate")]
    pub r#max_bitrate: Box<Option<i32>>,
    /// Min interval.
    #[builder(into, default)]
    #[serde(rename = "minIInterval")]
    pub r#min_i_interval: Box<Option<i32>>,
    /// Set the minimum QP.
    #[builder(into, default)]
    #[serde(rename = "minQp")]
    pub r#min_qp: Box<Option<i32>>,
    /// Enables or disables motion vector over picture boundaries.
    #[builder(into, default)]
    #[serde(rename = "mvOverPictureBoundaries")]
    pub r#mv_over_picture_boundaries: Box<Option<String>>,
    /// Enables or disables the motion vector temporal predictor.
    #[builder(into, default)]
    #[serde(rename = "mvTemporalPredictor")]
    pub r#mv_temporal_predictor: Box<Option<String>>,
    /// Pixel Aspect Ratio denominator.
    #[builder(into, default)]
    #[serde(rename = "parDenominator")]
    pub r#par_denominator: Box<Option<i32>>,
    /// Pixel Aspect Ratio numerator.
    #[builder(into, default)]
    #[serde(rename = "parNumerator")]
    pub r#par_numerator: Box<Option<i32>>,
    /// H265 profile.
    #[builder(into, default)]
    #[serde(rename = "profile")]
    pub r#profile: Box<Option<String>>,
    /// Controls the target quality for the video encode.
    #[builder(into, default)]
    #[serde(rename = "qvbrQualityLevel")]
    pub r#qvbr_quality_level: Box<Option<i32>>,
    /// Rate control mode.
    #[builder(into, default)]
    #[serde(rename = "rateControlMode")]
    pub r#rate_control_mode: Box<Option<String>>,
    /// Sets the scan type of the output.
    #[builder(into, default)]
    #[serde(rename = "scanType")]
    pub r#scan_type: Box<Option<String>>,
    /// Scene change detection.
    #[builder(into, default)]
    #[serde(rename = "sceneChangeDetect")]
    pub r#scene_change_detect: Box<Option<String>>,
    /// Number of slices per picture.
    #[builder(into, default)]
    #[serde(rename = "slices")]
    pub r#slices: Box<Option<i32>>,
    /// Set the H265 tier in the output.
    #[builder(into, default)]
    #[serde(rename = "tier")]
    pub r#tier: Box<Option<String>>,
    /// Sets the height of tiles.
    #[builder(into, default)]
    #[serde(rename = "tileHeight")]
    pub r#tile_height: Box<Option<i32>>,
    /// Enables or disables padding of tiles.
    #[builder(into, default)]
    #[serde(rename = "tilePadding")]
    pub r#tile_padding: Box<Option<String>>,
    /// Sets the width of tiles.
    #[builder(into, default)]
    #[serde(rename = "tileWidth")]
    pub r#tile_width: Box<Option<i32>>,
    /// Apply a burned in timecode. See H265 Timecode Burnin Settings for more details.
    #[builder(into, default)]
    #[serde(rename = "timecodeBurninSettings")]
    pub r#timecode_burnin_settings: Box<Option<super::super::types::medialive::ChannelEncoderSettingsVideoDescriptionCodecSettingsH265SettingsTimecodeBurninSettings>>,
    /// Determines how timecodes should be inserted into the video elementary stream.
    #[builder(into, default)]
    #[serde(rename = "timecodeInsertion")]
    pub r#timecode_insertion: Box<Option<String>>,
    /// Sets the size of the treeblock.
    #[builder(into, default)]
    #[serde(rename = "treeblockSize")]
    pub r#treeblock_size: Box<Option<String>>,
}
