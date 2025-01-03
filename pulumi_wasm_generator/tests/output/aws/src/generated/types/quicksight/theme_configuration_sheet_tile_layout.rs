#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ThemeConfigurationSheetTileLayout {
    /// The gutter settings that apply between tiles. See gutter.
    #[builder(into, default)]
    #[serde(rename = "gutter")]
    pub r#gutter: Box<Option<super::super::types::quicksight::ThemeConfigurationSheetTileLayoutGutter>>,
    /// The margin settings that apply around the outside edge of sheets. See margin.
    #[builder(into, default)]
    #[serde(rename = "margin")]
    pub r#margin: Box<Option<super::super::types::quicksight::ThemeConfigurationSheetTileLayoutMargin>>,
}
