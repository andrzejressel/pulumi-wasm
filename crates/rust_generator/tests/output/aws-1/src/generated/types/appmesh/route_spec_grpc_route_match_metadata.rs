#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct RouteSpecGrpcRouteMatchMetadata {
    /// If `true`, the match is on the opposite of the `match` criteria. Default is `false`.
    #[builder(into, default)]
    #[serde(rename = "invert")]
    pub r#invert: Box<Option<bool>>,
    /// Data to match from the request.
    #[builder(into, default)]
    #[serde(rename = "match")]
    pub r#match_: Box<Option<super::super::types::appmesh::RouteSpecGrpcRouteMatchMetadataMatch>>,
    /// Name of the route. Must be between 1 and 50 characters in length.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}
