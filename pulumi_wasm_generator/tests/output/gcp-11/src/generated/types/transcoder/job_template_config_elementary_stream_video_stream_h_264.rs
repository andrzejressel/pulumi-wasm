#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct JobTemplateConfigElementaryStreamVideoStreamH264 {
    /// The video bitrate in bits per second.
    #[builder(into)]
    #[serde(rename = "bitrateBps")]
    pub r#bitrate_bps: Box<i32>,
    /// Target CRF level. The default is '21'.
    #[builder(into, default)]
    #[serde(rename = "crfLevel")]
    pub r#crf_level: Box<Option<i32>>,
    /// The entropy coder to use. The default is 'cabac'.
    #[builder(into, default)]
    #[serde(rename = "entropyCoder")]
    pub r#entropy_coder: Box<Option<String>>,
    /// The target video frame rate in frames per second (FPS).
    #[builder(into)]
    #[serde(rename = "frameRate")]
    pub r#frame_rate: Box<i32>,
    /// Select the GOP size based on the specified duration. The default is '3s'.
    #[builder(into, default)]
    #[serde(rename = "gopDuration")]
    pub r#gop_duration: Box<Option<String>>,
    /// The height of the video in pixels.
    #[builder(into, default)]
    #[serde(rename = "heightPixels")]
    pub r#height_pixels: Box<Option<i32>>,
    /// HLG color format setting for H264.
    #[builder(into, default)]
    #[serde(rename = "hlg")]
    pub r#hlg: Box<Option<super::super::types::transcoder::JobTemplateConfigElementaryStreamVideoStreamH264Hlg>>,
    /// Pixel format to use. The default is 'yuv420p'.
    #[builder(into, default)]
    #[serde(rename = "pixelFormat")]
    pub r#pixel_format: Box<Option<String>>,
    /// Enforces the specified codec preset. The default is 'veryfast'.
    #[builder(into, default)]
    #[serde(rename = "preset")]
    pub r#preset: Box<Option<String>>,
    /// Enforces the specified codec profile.
    #[builder(into, default)]
    #[serde(rename = "profile")]
    pub r#profile: Box<Option<String>>,
    /// Specify the mode. The default is 'vbr'.
    #[builder(into, default)]
    #[serde(rename = "rateControlMode")]
    pub r#rate_control_mode: Box<Option<String>>,
    /// SDR color format setting for H264.
    #[builder(into, default)]
    #[serde(rename = "sdr")]
    pub r#sdr: Box<Option<super::super::types::transcoder::JobTemplateConfigElementaryStreamVideoStreamH264Sdr>>,
    /// Initial fullness of the Video Buffering Verifier (VBV) buffer in bits.
    #[builder(into, default)]
    #[serde(rename = "vbvFullnessBits")]
    pub r#vbv_fullness_bits: Box<Option<i32>>,
    /// Size of the Video Buffering Verifier (VBV) buffer in bits.
    #[builder(into, default)]
    #[serde(rename = "vbvSizeBits")]
    pub r#vbv_size_bits: Box<Option<i32>>,
    /// The width of the video in pixels.
    #[builder(into, default)]
    #[serde(rename = "widthPixels")]
    pub r#width_pixels: Box<Option<i32>>,
}
