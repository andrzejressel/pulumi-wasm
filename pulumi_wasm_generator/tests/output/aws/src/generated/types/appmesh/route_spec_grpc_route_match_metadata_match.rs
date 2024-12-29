#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RouteSpecGrpcRouteMatchMetadataMatch {
    /// Value sent by the client must match the specified value exactly. Must be between 1 and 255 characters in length.
    #[builder(into, default)]
    #[serde(rename = "exact")]
    pub r#exact: Box<Option<String>>,
    /// Value sent by the client must begin with the specified characters. Must be between 1 and 255 characters in length.
    #[builder(into, default)]
    #[serde(rename = "prefix")]
    pub r#prefix: Box<Option<String>>,
    /// Object that specifies the range of numbers that the value sent by the client must be included in.
    #[builder(into, default)]
    #[serde(rename = "range")]
    pub r#range: Box<Option<super::super::types::appmesh::RouteSpecGrpcRouteMatchMetadataMatchRange>>,
    /// Value sent by the client must include the specified characters. Must be between 1 and 255 characters in length.
    #[builder(into, default)]
    #[serde(rename = "regex")]
    pub r#regex: Box<Option<String>>,
    /// Value sent by the client must end with the specified characters. Must be between 1 and 255 characters in length.
    #[builder(into, default)]
    #[serde(rename = "suffix")]
    pub r#suffix: Box<Option<String>>,
}
