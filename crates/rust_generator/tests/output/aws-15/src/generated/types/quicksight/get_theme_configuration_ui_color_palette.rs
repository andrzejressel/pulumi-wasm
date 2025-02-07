#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetThemeConfigurationUiColorPalette {
    /// Color (hexadecimal) that applies to selected states and buttons.
    #[builder(into)]
    #[serde(rename = "accent")]
    pub r#accent: Box<String>,
    /// Color (hexadecimal) that applies to any text or other elements that appear over the accent color.
    #[builder(into)]
    #[serde(rename = "accentForeground")]
    pub r#accent_foreground: Box<String>,
    /// Color (hexadecimal) that applies to error messages.
    #[builder(into)]
    #[serde(rename = "danger")]
    pub r#danger: Box<String>,
    /// Color (hexadecimal) that applies to any text or other elements that appear over the error color.
    #[builder(into)]
    #[serde(rename = "dangerForeground")]
    pub r#danger_foreground: Box<String>,
    /// Color (hexadecimal) that applies to the names of fields that are identified as dimensions.
    #[builder(into)]
    #[serde(rename = "dimension")]
    pub r#dimension: Box<String>,
    /// Color (hexadecimal) that applies to any text or other elements that appear over the dimension color.
    #[builder(into)]
    #[serde(rename = "dimensionForeground")]
    pub r#dimension_foreground: Box<String>,
    /// Color (hexadecimal) that applies to the names of fields that are identified as measures.
    #[builder(into)]
    #[serde(rename = "measure")]
    pub r#measure: Box<String>,
    /// Color (hexadecimal) that applies to any text or other elements that appear over the measure color.
    #[builder(into)]
    #[serde(rename = "measureForeground")]
    pub r#measure_foreground: Box<String>,
    /// Color (hexadecimal) that applies to visuals and other high emphasis UI.
    #[builder(into)]
    #[serde(rename = "primaryBackground")]
    pub r#primary_background: Box<String>,
    /// Color (hexadecimal) of text and other foreground elements that appear over the primary background regions, such as grid lines, borders, table banding, icons, and so on.
    #[builder(into)]
    #[serde(rename = "primaryForeground")]
    pub r#primary_foreground: Box<String>,
    /// Color (hexadecimal) that applies to the sheet background and sheet controls.
    #[builder(into)]
    #[serde(rename = "secondaryBackground")]
    pub r#secondary_background: Box<String>,
    /// Color (hexadecimal) that applies to any sheet title, sheet control text, or UI that appears over the secondary background.
    #[builder(into)]
    #[serde(rename = "secondaryForeground")]
    pub r#secondary_foreground: Box<String>,
    /// Color (hexadecimal) that applies to success messages, for example the check mark for a successful download.
    #[builder(into)]
    #[serde(rename = "success")]
    pub r#success: Box<String>,
    /// Color (hexadecimal) that applies to any text or other elements that appear over the success color.
    #[builder(into)]
    #[serde(rename = "successForeground")]
    pub r#success_foreground: Box<String>,
    /// Color (hexadecimal) that applies to warning and informational messages.
    #[builder(into)]
    #[serde(rename = "warning")]
    pub r#warning: Box<String>,
    /// Color (hexadecimal) that applies to any text or other elements that appear over the warning color.
    #[builder(into)]
    #[serde(rename = "warningForeground")]
    pub r#warning_foreground: Box<String>,
}
