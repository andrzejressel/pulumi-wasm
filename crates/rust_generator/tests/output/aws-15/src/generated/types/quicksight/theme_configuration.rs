#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ThemeConfiguration {
    /// Color properties that apply to chart data colors. See data_color_palette.
    #[builder(into, default)]
    #[serde(rename = "dataColorPalette")]
    pub r#data_color_palette: Box<Option<super::super::types::quicksight::ThemeConfigurationDataColorPalette>>,
    /// Display options related to sheets. See sheet.
    #[builder(into, default)]
    #[serde(rename = "sheet")]
    pub r#sheet: Box<Option<super::super::types::quicksight::ThemeConfigurationSheet>>,
    /// Determines the typography options. See typography.
    #[builder(into, default)]
    #[serde(rename = "typography")]
    pub r#typography: Box<Option<super::super::types::quicksight::ThemeConfigurationTypography>>,
    /// Color properties that apply to the UI and to charts, excluding the colors that apply to data. See ui_color_palette.
    #[builder(into, default)]
    #[serde(rename = "uiColorPalette")]
    pub r#ui_color_palette: Box<Option<super::super::types::quicksight::ThemeConfigurationUiColorPalette>>,
}
