#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GremlinGraphIndexPolicySpatialIndex {
    /// Path for which the indexing behaviour applies to. According to the service design, all spatial types including `LineString`, `MultiPolygon`, `Point`, and `Polygon` will be applied to the path.
    #[builder(into)]
    #[serde(rename = "path")]
    pub r#path: Box<String>,
    #[builder(into, default)]
    #[serde(rename = "types")]
    pub r#types: Box<Option<Vec<String>>>,
}
