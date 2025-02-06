#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetThemeConfigurationTypography {
    /// Determines the list of font families. Maximum number of 5 items. See font_families.
    #[builder(into)]
    #[serde(rename = "fontFamilies")]
    pub r#font_families: Box<Vec<super::super::types::quicksight::GetThemeConfigurationTypographyFontFamily>>,
}
