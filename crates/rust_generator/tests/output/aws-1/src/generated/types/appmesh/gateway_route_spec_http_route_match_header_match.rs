#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GatewayRouteSpecHttpRouteMatchHeaderMatch {
    /// Header value sent by the client must match the specified value exactly.
    #[builder(into, default)]
    #[serde(rename = "exact")]
    pub r#exact: Box<Option<String>>,
    /// Header value sent by the client must begin with the specified characters.
    #[builder(into, default)]
    #[serde(rename = "prefix")]
    pub r#prefix: Box<Option<String>>,
    /// Object that specifies the range of numbers that the header value sent by the client must be included in.
    #[builder(into, default)]
    #[serde(rename = "range")]
    pub r#range: Box<Option<super::super::types::appmesh::GatewayRouteSpecHttpRouteMatchHeaderMatchRange>>,
    /// Header value sent by the client must include the specified characters.
    #[builder(into, default)]
    #[serde(rename = "regex")]
    pub r#regex: Box<Option<String>>,
    /// Header value sent by the client must end with the specified characters.
    #[builder(into, default)]
    #[serde(rename = "suffix")]
    pub r#suffix: Box<Option<String>>,
}
