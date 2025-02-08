#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RouteSpecHttpRouteMatch {
    /// Client request headers to match on.
    #[builder(into, default)]
    #[serde(rename = "headers")]
    pub r#headers: Box<Option<Vec<super::super::types::appmesh::RouteSpecHttpRouteMatchHeader>>>,
    /// Client request header method to match on. Valid values: `GET`, `HEAD`, `POST`, `PUT`, `DELETE`, `CONNECT`, `OPTIONS`, `TRACE`, `PATCH`.
    #[builder(into, default)]
    #[serde(rename = "method")]
    pub r#method: Box<Option<String>>,
    /// Client request path to match on.
    #[builder(into, default)]
    #[serde(rename = "path")]
    pub r#path: Box<Option<super::super::types::appmesh::RouteSpecHttpRouteMatchPath>>,
    /// The port number to match from the request.
    #[builder(into, default)]
    #[serde(rename = "port")]
    pub r#port: Box<Option<i32>>,
    /// Path with which to match requests.
    /// This parameter must always start with /, which by itself matches all requests to the virtual router service name.
    #[builder(into, default)]
    #[serde(rename = "prefix")]
    pub r#prefix: Box<Option<String>>,
    /// Client request query parameters to match on.
    #[builder(into, default)]
    #[serde(rename = "queryParameters")]
    pub r#query_parameters: Box<Option<Vec<super::super::types::appmesh::RouteSpecHttpRouteMatchQueryParameter>>>,
    /// Client request header scheme to match on. Valid values: `http`, `https`.
    #[builder(into, default)]
    #[serde(rename = "scheme")]
    pub r#scheme: Box<Option<String>>,
}
