#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetDataSetColumnGroupGeoSpatialColumnGroup {
    #[builder(into)]
    #[serde(rename = "columns")]
    pub r#columns: Box<Vec<String>>,
    #[builder(into)]
    #[serde(rename = "countryCode")]
    pub r#country_code: Box<String>,
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}