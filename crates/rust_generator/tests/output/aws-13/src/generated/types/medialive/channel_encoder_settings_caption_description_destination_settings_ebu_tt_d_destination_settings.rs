#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ChannelEncoderSettingsCaptionDescriptionDestinationSettingsEbuTtDDestinationSettings {
    /// Complete this field if you want to include the name of the copyright holder in the copyright tag in the captions metadata.
    #[builder(into, default)]
    #[serde(rename = "copyrightHolder")]
    pub r#copyright_holder: Box<Option<String>>,
    /// Specifies how to handle the gap between the lines (in multi-line captions). - enabled: Fill with the captions background color (as specified in the input captions). - disabled: Leave the gap unfilled.
    #[builder(into, default)]
    #[serde(rename = "fillLineGap")]
    pub r#fill_line_gap: Box<Option<String>>,
    /// Specifies the font family to include in the font data attached to the EBU-TT captions. Valid only if styleControl is set to include. If you leave this field empty, the font family is set to “monospaced”. (If styleControl is set to exclude, the font family is always set to “monospaced”.) You specify only the font family. All other style information (color, bold, position and so on) is copied from the input captions. The size is always set to 100% to allow the downstream player to choose the size. - Enter a list of font families, as a comma-separated list of font names, in order of preference. The name can be a font family (such as “Arial”), or a generic font family (such as “serif”), or “default” (to let the downstream player choose the font). - Leave blank to set the family to “monospace”.
    #[builder(into, default)]
    #[serde(rename = "fontFamily")]
    pub r#font_family: Box<Option<String>>,
    /// Specifies the style information (font color, font position, and so on) to include in the font data that is attached to the EBU-TT captions. - include: Take the style information (font color, font position, and so on) from the source captions and include that information in the font data attached to the EBU-TT captions. This option is valid only if the source captions are Embedded or Teletext. - exclude: In the font data attached to the EBU-TT captions, set the font family to “monospaced”. Do not include any other style information.
    #[builder(into, default)]
    #[serde(rename = "styleControl")]
    pub r#style_control: Box<Option<String>>,
}
