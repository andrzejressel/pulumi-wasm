#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetThemeConfigurationSheet {
    /// The layout options for tiles. See tile_layout.
    #[builder(into)]
    #[serde(rename = "tileLayouts")]
    pub r#tile_layouts: Box<Vec<super::super::types::quicksight::GetThemeConfigurationSheetTileLayout>>,
    /// The display options for tiles. See tile.
    #[builder(into)]
    #[serde(rename = "tiles")]
    pub r#tiles: Box<Vec<super::super::types::quicksight::GetThemeConfigurationSheetTile>>,
}
