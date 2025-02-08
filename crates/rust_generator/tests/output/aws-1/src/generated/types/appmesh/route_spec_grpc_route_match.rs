#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct RouteSpecGrpcRouteMatch {
    /// Data to match from the gRPC request.
    #[builder(into, default)]
    #[serde(rename = "metadatas")]
    pub r#metadatas: Box<Option<Vec<super::super::types::appmesh::RouteSpecGrpcRouteMatchMetadata>>>,
    /// Method name to match from the request. If you specify a name, you must also specify a `service_name`.
    #[builder(into, default)]
    #[serde(rename = "methodName")]
    pub r#method_name: Box<Option<String>>,
    /// The port number to match from the request.
    #[builder(into, default)]
    #[serde(rename = "port")]
    pub r#port: Box<Option<i32>>,
    #[builder(into, default)]
    #[serde(rename = "prefix")]
    pub r#prefix: Box<Option<String>>,
    /// Fully qualified domain name for the service to match from the request.
    #[builder(into, default)]
    #[serde(rename = "serviceName")]
    pub r#service_name: Box<Option<String>>,
}
