#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetThemeConfigurationSheetTileLayout {
    /// The gutter settings that apply between tiles. See gutter.
    #[builder(into)]
    #[serde(rename = "gutters")]
    pub r#gutters: Box<Vec<super::super::types::quicksight::GetThemeConfigurationSheetTileLayoutGutter>>,
    /// The margin settings that apply around the outside edge of sheets. See margin.
    #[builder(into)]
    #[serde(rename = "margins")]
    pub r#margins: Box<Vec<super::super::types::quicksight::GetThemeConfigurationSheetTileLayoutMargin>>,
}