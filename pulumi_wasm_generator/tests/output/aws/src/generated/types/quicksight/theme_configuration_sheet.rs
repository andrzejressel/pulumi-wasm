#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ThemeConfigurationSheet {
    /// The display options for tiles. See tile.
    #[builder(into, default)]
    #[serde(rename = "tile")]
    pub r#tile: Box<Option<super::super::types::quicksight::ThemeConfigurationSheetTile>>,
    /// The layout options for tiles. See tile_layout.
    #[builder(into, default)]
    #[serde(rename = "tileLayout")]
    pub r#tile_layout: Box<Option<super::super::types::quicksight::ThemeConfigurationSheetTileLayout>>,
}