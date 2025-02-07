#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetThemeConfigurationSheetTile {
    /// The border around a tile. See border.
    #[builder(into)]
    #[serde(rename = "borders")]
    pub r#borders: Box<Vec<super::super::types::quicksight::GetThemeConfigurationSheetTileBorder>>,
}
