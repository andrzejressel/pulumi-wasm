#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetVirtualNodeSpecListenerTimeoutHttp2 {
    #[builder(into)]
    #[serde(rename = "idles")]
    pub r#idles: Box<Vec<super::super::types::appmesh::GetVirtualNodeSpecListenerTimeoutHttp2Idle>>,
    #[builder(into)]
    #[serde(rename = "perRequests")]
    pub r#per_requests: Box<Vec<super::super::types::appmesh::GetVirtualNodeSpecListenerTimeoutHttp2PerRequest>>,
}