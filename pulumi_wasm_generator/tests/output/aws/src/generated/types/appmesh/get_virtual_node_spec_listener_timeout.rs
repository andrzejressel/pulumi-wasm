#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetVirtualNodeSpecListenerTimeout {
    #[builder(into)]
    #[serde(rename = "grpcs")]
    pub r#grpcs: Box<Vec<super::super::types::appmesh::GetVirtualNodeSpecListenerTimeoutGrpc>>,
    #[builder(into)]
    #[serde(rename = "http2s")]
    pub r#http_2_s: Box<Vec<super::super::types::appmesh::GetVirtualNodeSpecListenerTimeoutHttp2>>,
    #[builder(into)]
    #[serde(rename = "https")]
    pub r#https: Box<Vec<super::super::types::appmesh::GetVirtualNodeSpecListenerTimeoutHttp>>,
    #[builder(into)]
    #[serde(rename = "tcps")]
    pub r#tcps: Box<Vec<super::super::types::appmesh::GetVirtualNodeSpecListenerTimeoutTcp>>,
}