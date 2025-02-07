#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PresetVideo {
    /// The display aspect ratio of the video in the output file. Valid values are: `auto`, `1:1`, `4:3`, `3:2`, `16:9`. (Note; to better control resolution and aspect ratio of output videos, we recommend that you use the values `max_width`, `max_height`, `sizing_policy`, `padding_policy`, and `display_aspect_ratio` instead of `resolution` and `aspect_ratio`.)
    #[builder(into, default)]
    #[serde(rename = "aspectRatio")]
    pub r#aspect_ratio: Box<Option<String>>,
    /// The bit rate of the video stream in the output file, in kilobits/second. You can configure variable bit rate or constant bit rate encoding.
    #[builder(into, default)]
    #[serde(rename = "bitRate")]
    pub r#bit_rate: Box<Option<String>>,
    /// The video codec for the output file. Valid values are `gif`, `H.264`, `mpeg2`, `vp8`, and `vp9`.
    #[builder(into, default)]
    #[serde(rename = "codec")]
    pub r#codec: Box<Option<String>>,
    /// The value that Elastic Transcoder adds to the metadata in the output file. If you set DisplayAspectRatio to auto, Elastic Transcoder chooses an aspect ratio that ensures square pixels. If you specify another option, Elastic Transcoder sets that value in the output file.
    #[builder(into, default)]
    #[serde(rename = "displayAspectRatio")]
    pub r#display_aspect_ratio: Box<Option<String>>,
    /// Whether to use a fixed value for Video:FixedGOP. Not applicable for containers of type gif. Valid values are true and false. Also known as, Fixed Number of Frames Between Keyframes.
    #[builder(into, default)]
    #[serde(rename = "fixedGop")]
    pub r#fixed_gop: Box<Option<String>>,
    /// The frames per second for the video stream in the output file. The following values are valid: `auto`, `10`, `15`, `23.97`, `24`, `25`, `29.97`, `30`, `50`, `60`.
    #[builder(into, default)]
    #[serde(rename = "frameRate")]
    pub r#frame_rate: Box<Option<String>>,
    /// The maximum number of frames between key frames. Not applicable for containers of type gif.
    #[builder(into, default)]
    #[serde(rename = "keyframesMaxDist")]
    pub r#keyframes_max_dist: Box<Option<String>>,
    /// If you specify auto for FrameRate, Elastic Transcoder uses the frame rate of the input video for the frame rate of the output video, up to the maximum frame rate. If you do not specify a MaxFrameRate, Elastic Transcoder will use a default of 30.
    #[builder(into, default)]
    #[serde(rename = "maxFrameRate")]
    pub r#max_frame_rate: Box<Option<String>>,
    /// The maximum height of the output video in pixels. If you specify auto, Elastic Transcoder uses 1080 (Full HD) as the default value. If you specify a numeric value, enter an even integer between 96 and 3072, inclusive.
    #[builder(into, default)]
    #[serde(rename = "maxHeight")]
    pub r#max_height: Box<Option<String>>,
    /// The maximum width of the output video in pixels. If you specify auto, Elastic Transcoder uses 1920 (Full HD) as the default value. If you specify a numeric value, enter an even integer between 128 and 4096, inclusive.
    #[builder(into, default)]
    #[serde(rename = "maxWidth")]
    pub r#max_width: Box<Option<String>>,
    /// When you set PaddingPolicy to Pad, Elastic Transcoder might add black bars to the top and bottom and/or left and right sides of the output video to make the total size of the output video match the values that you specified for `max_width` and `max_height`.
    #[builder(into, default)]
    #[serde(rename = "paddingPolicy")]
    pub r#padding_policy: Box<Option<String>>,
    /// The width and height of the video in the output file, in pixels. Valid values are `auto` and `widthxheight`. (see note for `aspect_ratio`)
    #[builder(into, default)]
    #[serde(rename = "resolution")]
    pub r#resolution: Box<Option<String>>,
    /// A value that controls scaling of the output video. Valid values are: `Fit`, `Fill`, `Stretch`, `Keep`, `ShrinkToFit`, `ShrinkToFill`.
    #[builder(into, default)]
    #[serde(rename = "sizingPolicy")]
    pub r#sizing_policy: Box<Option<String>>,
}
