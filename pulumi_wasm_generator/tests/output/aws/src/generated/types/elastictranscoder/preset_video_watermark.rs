#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PresetVideoWatermark {
    /// The horizontal position of the watermark unless you specify a nonzero value for `horzontal_offset`.
    #[builder(into, default)]
    #[serde(rename = "horizontalAlign")]
    pub r#horizontal_align: Box<Option<String>>,
    /// The amount by which you want the horizontal position of the watermark to be offset from the position specified by `horizontal_align`.
    #[builder(into, default)]
    #[serde(rename = "horizontalOffset")]
    pub r#horizontal_offset: Box<Option<String>>,
    /// A unique identifier for the settings for one watermark. The value of Id can be up to 40 characters long. You can specify settings for up to four watermarks.
    #[builder(into, default)]
    #[serde(rename = "id")]
    pub r#id: Box<Option<String>>,
    /// The maximum height of the watermark.
    #[builder(into, default)]
    #[serde(rename = "maxHeight")]
    pub r#max_height: Box<Option<String>>,
    /// The maximum width of the watermark.
    #[builder(into, default)]
    #[serde(rename = "maxWidth")]
    pub r#max_width: Box<Option<String>>,
    /// A percentage that indicates how much you want a watermark to obscure the video in the location where it appears.
    #[builder(into, default)]
    #[serde(rename = "opacity")]
    pub r#opacity: Box<Option<String>>,
    /// A value that controls scaling of the watermark. Valid values are: `Fit`, `Stretch`, `ShrinkToFit`
    #[builder(into, default)]
    #[serde(rename = "sizingPolicy")]
    pub r#sizing_policy: Box<Option<String>>,
    /// A value that determines how Elastic Transcoder interprets values that you specified for `video_watermarks.horizontal_offset`, `video_watermarks.vertical_offset`, `video_watermarks.max_width`, and `video_watermarks.max_height`. Valid values are `Content` and `Frame`.
    #[builder(into, default)]
    #[serde(rename = "target")]
    pub r#target: Box<Option<String>>,
    /// The vertical position of the watermark unless you specify a nonzero value for `vertical_align`. Valid values are `Top`, `Bottom`, `Center`.
    #[builder(into, default)]
    #[serde(rename = "verticalAlign")]
    pub r#vertical_align: Box<Option<String>>,
    /// The amount by which you want the vertical position of the watermark to be offset from the position specified by `vertical_align`
    #[builder(into, default)]
    #[serde(rename = "verticalOffset")]
    pub r#vertical_offset: Box<Option<String>>,
}
