#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DataSetColumnGroup {
    /// Geospatial column group that denotes a hierarchy. See geo_spatial_column_group.
    #[builder(into, default)]
    #[serde(rename = "geoSpatialColumnGroup")]
    pub r#geo_spatial_column_group: Box<Option<super::super::types::quicksight::DataSetColumnGroupGeoSpatialColumnGroup>>,
}