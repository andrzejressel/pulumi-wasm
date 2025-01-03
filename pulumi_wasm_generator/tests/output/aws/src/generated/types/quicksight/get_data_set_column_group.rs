#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetDataSetColumnGroup {
    #[builder(into)]
    #[serde(rename = "geoSpatialColumnGroups")]
    pub r#geo_spatial_column_groups: Box<Vec<super::super::types::quicksight::GetDataSetColumnGroupGeoSpatialColumnGroup>>,
}
