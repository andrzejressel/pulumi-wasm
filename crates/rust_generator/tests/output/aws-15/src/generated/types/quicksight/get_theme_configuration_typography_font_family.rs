#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetThemeConfigurationTypographyFontFamily {
    /// Font family name.
    #[builder(into)]
    #[serde(rename = "fontFamily")]
    pub r#font_family: Box<String>,
}
