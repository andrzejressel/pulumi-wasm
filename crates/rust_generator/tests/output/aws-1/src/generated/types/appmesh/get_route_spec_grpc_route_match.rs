#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetRouteSpecGrpcRouteMatch {
    #[builder(into)]
    #[serde(rename = "metadatas")]
    pub r#metadatas: Box<Vec<super::super::types::appmesh::GetRouteSpecGrpcRouteMatchMetadata>>,
    #[builder(into)]
    #[serde(rename = "methodName")]
    pub r#method_name: Box<String>,
    #[builder(into)]
    #[serde(rename = "port")]
    pub r#port: Box<i32>,
    #[builder(into)]
    #[serde(rename = "prefix")]
    pub r#prefix: Box<String>,
    #[builder(into)]
    #[serde(rename = "serviceName")]
    pub r#service_name: Box<String>,
}
