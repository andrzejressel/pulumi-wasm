#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetVirtualGatewaySpecListenerConnectionPool {
    #[builder(into)]
    #[serde(rename = "grpcs")]
    pub r#grpcs: Box<Vec<super::super::types::appmesh::GetVirtualGatewaySpecListenerConnectionPoolGrpc>>,
    #[builder(into)]
    #[serde(rename = "http2s")]
    pub r#http_2_s: Box<Vec<super::super::types::appmesh::GetVirtualGatewaySpecListenerConnectionPoolHttp2>>,
    #[builder(into)]
    #[serde(rename = "https")]
    pub r#https: Box<Vec<super::super::types::appmesh::GetVirtualGatewaySpecListenerConnectionPoolHttp>>,
}
