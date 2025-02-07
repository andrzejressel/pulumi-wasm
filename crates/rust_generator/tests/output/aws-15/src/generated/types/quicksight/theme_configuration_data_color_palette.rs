#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ThemeConfigurationDataColorPalette {
    /// List of hexadecimal codes for the colors. Minimum of 8 items and maximum of 20 items.
    #[builder(into, default)]
    #[serde(rename = "colors")]
    pub r#colors: Box<Option<Vec<String>>>,
    /// The hexadecimal code of a color that applies to charts where a lack of data is highlighted.
    #[builder(into, default)]
    #[serde(rename = "emptyFillColor")]
    pub r#empty_fill_color: Box<Option<String>>,
    /// The minimum and maximum hexadecimal codes that describe a color gradient. List of exactly 2 items.
    #[builder(into, default)]
    #[serde(rename = "minMaxGradients")]
    pub r#min_max_gradients: Box<Option<Vec<String>>>,
}
