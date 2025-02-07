#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ThemeConfigurationUiColorPalette {
    /// Color (hexadecimal) that applies to selected states and buttons.
    #[builder(into, default)]
    #[serde(rename = "accent")]
    pub r#accent: Box<Option<String>>,
    /// Color (hexadecimal) that applies to any text or other elements that appear over the accent color.
    #[builder(into, default)]
    #[serde(rename = "accentForeground")]
    pub r#accent_foreground: Box<Option<String>>,
    /// Color (hexadecimal) that applies to error messages.
    #[builder(into, default)]
    #[serde(rename = "danger")]
    pub r#danger: Box<Option<String>>,
    /// Color (hexadecimal) that applies to any text or other elements that appear over the error color.
    #[builder(into, default)]
    #[serde(rename = "dangerForeground")]
    pub r#danger_foreground: Box<Option<String>>,
    /// Color (hexadecimal) that applies to the names of fields that are identified as dimensions.
    #[builder(into, default)]
    #[serde(rename = "dimension")]
    pub r#dimension: Box<Option<String>>,
    /// Color (hexadecimal) that applies to any text or other elements that appear over the dimension color.
    #[builder(into, default)]
    #[serde(rename = "dimensionForeground")]
    pub r#dimension_foreground: Box<Option<String>>,
    /// Color (hexadecimal) that applies to the names of fields that are identified as measures.
    #[builder(into, default)]
    #[serde(rename = "measure")]
    pub r#measure: Box<Option<String>>,
    /// Color (hexadecimal) that applies to any text or other elements that appear over the measure color.
    #[builder(into, default)]
    #[serde(rename = "measureForeground")]
    pub r#measure_foreground: Box<Option<String>>,
    /// Color (hexadecimal) that applies to visuals and other high emphasis UI.
    #[builder(into, default)]
    #[serde(rename = "primaryBackground")]
    pub r#primary_background: Box<Option<String>>,
    /// Color (hexadecimal) of text and other foreground elements that appear over the primary background regions, such as grid lines, borders, table banding, icons, and so on.
    #[builder(into, default)]
    #[serde(rename = "primaryForeground")]
    pub r#primary_foreground: Box<Option<String>>,
    /// Color (hexadecimal) that applies to the sheet background and sheet controls.
    #[builder(into, default)]
    #[serde(rename = "secondaryBackground")]
    pub r#secondary_background: Box<Option<String>>,
    /// Color (hexadecimal) that applies to any sheet title, sheet control text, or UI that appears over the secondary background.
    #[builder(into, default)]
    #[serde(rename = "secondaryForeground")]
    pub r#secondary_foreground: Box<Option<String>>,
    /// Color (hexadecimal) that applies to success messages, for example the check mark for a successful download.
    #[builder(into, default)]
    #[serde(rename = "success")]
    pub r#success: Box<Option<String>>,
    /// Color (hexadecimal) that applies to any text or other elements that appear over the success color.
    #[builder(into, default)]
    #[serde(rename = "successForeground")]
    pub r#success_foreground: Box<Option<String>>,
    /// Color (hexadecimal) that applies to warning and informational messages.
    #[builder(into, default)]
    #[serde(rename = "warning")]
    pub r#warning: Box<Option<String>>,
    /// Color (hexadecimal) that applies to any text or other elements that appear over the warning color.
    #[builder(into, default)]
    #[serde(rename = "warningForeground")]
    pub r#warning_foreground: Box<Option<String>>,
}
