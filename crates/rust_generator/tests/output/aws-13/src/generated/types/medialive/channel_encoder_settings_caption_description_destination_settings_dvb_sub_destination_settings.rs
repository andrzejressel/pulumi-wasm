#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ChannelEncoderSettingsCaptionDescriptionDestinationSettingsDvbSubDestinationSettings {
    /// If no explicit xPosition or yPosition is provided, setting alignment to centered will place the captions at the bottom center of the output. Similarly, setting a left alignment will align captions to the bottom left of the output. If x and y positions are given in conjunction with the alignment parameter, the font will be justified (either left or centered) relative to those coordinates. Selecting “smart” justification will left-justify live subtitles and center-justify pre-recorded subtitles. This option is not valid for source captions that are STL or 608/embedded. These source settings are already pre-defined by the caption stream. All burn-in and DVB-Sub font settings must match.
    #[builder(into, default)]
    #[serde(rename = "alignment")]
    pub r#alignment: Box<Option<String>>,
    /// Specifies the color of the rectangle behind the captions. All burn-in and DVB-Sub font settings must match.
    #[builder(into, default)]
    #[serde(rename = "backgroundColor")]
    pub r#background_color: Box<Option<String>>,
    /// Specifies the opacity of the background rectangle. 255 is opaque; 0 is transparent. Leaving this parameter blank is equivalent to setting it to 0 (transparent). All burn-in and DVB-Sub font settings must match.
    #[builder(into, default)]
    #[serde(rename = "backgroundOpacity")]
    pub r#background_opacity: Box<Option<i32>>,
    /// External font file used for caption burn-in. File extension must be ‘ttf’ or ‘tte’. Although the user can select output fonts for many different types of input captions, embedded, STL and teletext sources use a strict grid system. Using external fonts with these caption sources could cause unexpected display of proportional fonts. All burn-in and DVB-Sub font settings must match. See Font for more details.
    #[builder(into, default)]
    #[serde(rename = "font")]
    pub r#font: Box<Option<super::super::types::medialive::ChannelEncoderSettingsCaptionDescriptionDestinationSettingsDvbSubDestinationSettingsFont>>,
    /// Specifies the color of the burned-in captions. This option is not valid for source captions that are STL, 608/embedded or teletext. These source settings are already pre-defined by the caption stream. All burn-in and DVB-Sub font settings must match.
    #[builder(into, default)]
    #[serde(rename = "fontColor")]
    pub r#font_color: Box<Option<String>>,
    /// Specifies the opacity of the burned-in captions. 255 is opaque; 0 is transparent. All burn-in and DVB-Sub font settings must match.
    #[builder(into, default)]
    #[serde(rename = "fontOpacity")]
    pub r#font_opacity: Box<Option<i32>>,
    /// Font resolution in DPI (dots per inch); default is 96 dpi. All burn-in and DVB-Sub font settings must match.
    #[builder(into, default)]
    #[serde(rename = "fontResolution")]
    pub r#font_resolution: Box<Option<i32>>,
    /// When set to auto fontSize will scale depending on the size of the output. Giving a positive integer will specify the exact font size in points. All burn-in and DVB-Sub font settings must match.
    #[builder(into, default)]
    #[serde(rename = "fontSize")]
    pub r#font_size: Box<Option<String>>,
    /// Specifies font outline color. This option is not valid for source captions that are either 608/embedded or teletext. These source settings are already pre-defined by the caption stream. All burn-in and DVB-Sub font settings must match.
    #[builder(into, default)]
    #[serde(rename = "outlineColor")]
    pub r#outline_color: Box<Option<String>>,
    /// Specifies font outline size in pixels. This option is not valid for source captions that are either 608/embedded or teletext. These source settings are already pre-defined by the caption stream. All burn-in and DVB-Sub font settings must match.
    #[builder(into, default)]
    #[serde(rename = "outlineSize")]
    pub r#outline_size: Box<Option<i32>>,
    /// Specifies the color of the shadow cast by the captions. All burn-in and DVB-Sub font settings must match.
    #[builder(into, default)]
    #[serde(rename = "shadowColor")]
    pub r#shadow_color: Box<Option<String>>,
    /// Specifies the opacity of the shadow. 255 is opaque; 0 is transparent. Leaving this parameter blank is equivalent to setting it to 0 (transparent). All burn-in and DVB-Sub font settings must match.
    #[builder(into, default)]
    #[serde(rename = "shadowOpacity")]
    pub r#shadow_opacity: Box<Option<i32>>,
    /// Specifies the horizontal offset of the shadow relative to the captions in pixels. A value of -2 would result in a shadow offset 2 pixels to the left. All burn-in and DVB-Sub font settings must match.
    #[builder(into, default)]
    #[serde(rename = "shadowXOffset")]
    pub r#shadow_x_offset: Box<Option<i32>>,
    /// Specifies the vertical offset of the shadow relative to the captions in pixels. A value of -2 would result in a shadow offset 2 pixels above the text. All burn-in and DVB-Sub font settings must match.
    #[builder(into, default)]
    #[serde(rename = "shadowYOffset")]
    pub r#shadow_y_offset: Box<Option<i32>>,
    /// Controls whether a fixed grid size will be used to generate the output subtitles bitmap. Only applicable for Teletext inputs and DVB-Sub/Burn-in outputs.
    #[builder(into, default)]
    #[serde(rename = "teletextGridControl")]
    pub r#teletext_grid_control: Box<Option<String>>,
    /// Specifies the horizontal position of the caption relative to the left side of the output in pixels. A value of 10 would result in the captions starting 10 pixels from the left of the output. If no explicit xPosition is provided, the horizontal caption position will be determined by the alignment parameter. This option is not valid for source captions that are STL, 608/embedded or teletext. These source settings are already pre-defined by the caption stream. All burn-in and DVB-Sub font settings must match.
    #[builder(into, default)]
    #[serde(rename = "xPosition")]
    pub r#x_position: Box<Option<i32>>,
    /// Specifies the vertical position of the caption relative to the top of the output in pixels. A value of 10 would result in the captions starting 10 pixels from the top of the output. If no explicit yPosition is provided, the caption will be positioned towards the bottom of the output. This option is not valid for source captions that are STL, 608/embedded or teletext. These source settings are already pre-defined by the caption stream. All burn-in and DVB-Sub font settings must match.
    #[builder(into, default)]
    #[serde(rename = "yPosition")]
    pub r#y_position: Box<Option<i32>>,
}
