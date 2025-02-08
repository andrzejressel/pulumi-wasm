#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ChannelEncoderSettingsVideoDescriptionCodecSettingsH264Settings {
    /// Enables or disables adaptive quantization.
    #[builder(into, default)]
    #[serde(rename = "adaptiveQuantization")]
    pub r#adaptive_quantization: Box<Option<String>>,
    /// Indicates that AFD values will be written into the output stream.
    #[builder(into, default)]
    #[serde(rename = "afdSignaling")]
    pub r#afd_signaling: Box<Option<String>>,
    /// Average bitrate in bits/second.
    #[builder(into, default)]
    #[serde(rename = "bitrate")]
    pub r#bitrate: Box<Option<i32>>,
    #[builder(into, default)]
    #[serde(rename = "bufFillPct")]
    pub r#buf_fill_pct: Box<Option<i32>>,
    /// Size of buffer in bits.
    #[builder(into, default)]
    #[serde(rename = "bufSize")]
    pub r#buf_size: Box<Option<i32>>,
    /// Includes color space metadata in the output.
    #[builder(into, default)]
    #[serde(rename = "colorMetadata")]
    pub r#color_metadata: Box<Option<String>>,
    /// Entropy encoding mode.
    #[builder(into, default)]
    #[serde(rename = "entropyEncoding")]
    pub r#entropy_encoding: Box<Option<String>>,
    /// Filters to apply to an encode. See H264 Filter Settings for more details.
    #[builder(into, default)]
    #[serde(rename = "filterSettings")]
    pub r#filter_settings: Box<Option<super::super::types::medialive::ChannelEncoderSettingsVideoDescriptionCodecSettingsH264SettingsFilterSettings>>,
    /// Four bit AFD value to write on all frames of video in the output stream.
    #[builder(into, default)]
    #[serde(rename = "fixedAfd")]
    pub r#fixed_afd: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "flickerAq")]
    pub r#flicker_aq: Box<Option<String>>,
    /// Controls whether coding is performed on a field basis or on a frame basis.
    #[builder(into, default)]
    #[serde(rename = "forceFieldPictures")]
    pub r#force_field_pictures: Box<Option<String>>,
    /// Indicates how the output video frame rate is specified.
    #[builder(into, default)]
    #[serde(rename = "framerateControl")]
    pub r#framerate_control: Box<Option<String>>,
    /// Framerate denominator.
    #[builder(into, default)]
    #[serde(rename = "framerateDenominator")]
    pub r#framerate_denominator: Box<Option<i32>>,
    /// Framerate numerator.
    #[builder(into, default)]
    #[serde(rename = "framerateNumerator")]
    pub r#framerate_numerator: Box<Option<i32>>,
    /// GOP-B reference.
    #[builder(into, default)]
    #[serde(rename = "gopBReference")]
    pub r#gop_b_reference: Box<Option<String>>,
    /// Frequency of closed GOPs.
    #[builder(into, default)]
    #[serde(rename = "gopClosedCadence")]
    pub r#gop_closed_cadence: Box<Option<i32>>,
    /// Number of B-frames between reference frames.
    #[builder(into, default)]
    #[serde(rename = "gopNumBFrames")]
    pub r#gop_num_b_frames: Box<Option<i32>>,
    /// GOP size in units of either frames of seconds per `gop_size_units`.
    #[builder(into, default)]
    #[serde(rename = "gopSize")]
    pub r#gop_size: Box<Option<f64>>,
    /// Indicates if the `gop_size` is specified in frames or seconds.
    #[builder(into, default)]
    #[serde(rename = "gopSizeUnits")]
    pub r#gop_size_units: Box<Option<String>>,
    /// H264 level.
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
    /// Number of reference frames to use.
    #[builder(into, default)]
    #[serde(rename = "numRefFrames")]
    pub r#num_ref_frames: Box<Option<i32>>,
    /// Indicates how the output pixel aspect ratio is specified.
    #[builder(into, default)]
    #[serde(rename = "parControl")]
    pub r#par_control: Box<Option<String>>,
    /// Pixel Aspect Ratio denominator.
    #[builder(into, default)]
    #[serde(rename = "parDenominator")]
    pub r#par_denominator: Box<Option<i32>>,
    /// Pixel Aspect Ratio numerator.
    #[builder(into, default)]
    #[serde(rename = "parNumerator")]
    pub r#par_numerator: Box<Option<i32>>,
    /// H264 profile.
    #[builder(into, default)]
    #[serde(rename = "profile")]
    pub r#profile: Box<Option<String>>,
    /// Quality level.
    #[builder(into, default)]
    #[serde(rename = "qualityLevel")]
    pub r#quality_level: Box<Option<String>>,
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
    /// Softness.
    #[builder(into, default)]
    #[serde(rename = "softness")]
    pub r#softness: Box<Option<i32>>,
    /// Makes adjustments within each frame based on spatial variation of content complexity.
    #[builder(into, default)]
    #[serde(rename = "spatialAq")]
    pub r#spatial_aq: Box<Option<String>>,
    /// Subgop length.
    #[builder(into, default)]
    #[serde(rename = "subgopLength")]
    pub r#subgop_length: Box<Option<String>>,
    /// Produces a bitstream compliant with SMPTE RP-2027.
    #[builder(into, default)]
    #[serde(rename = "syntax")]
    pub r#syntax: Box<Option<String>>,
    /// Makes adjustments within each frame based on temporal variation of content complexity.
    #[builder(into, default)]
    #[serde(rename = "temporalAq")]
    pub r#temporal_aq: Box<Option<String>>,
    /// Determines how timecodes should be inserted into the video elementary stream.
    #[builder(into, default)]
    #[serde(rename = "timecodeInsertion")]
    pub r#timecode_insertion: Box<Option<String>>,
}
