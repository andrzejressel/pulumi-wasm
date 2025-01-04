#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct VirtualNodeSpecListenerTimeoutHttp2 {
    /// Idle timeout. An idle timeout bounds the amount of time that a connection may be idle.
    #[builder(into, default)]
    #[serde(rename = "idle")]
    pub r#idle: Box<Option<super::super::types::appmesh::VirtualNodeSpecListenerTimeoutHttp2Idle>>,
    /// Per request timeout.
    #[builder(into, default)]
    #[serde(rename = "perRequest")]
    pub r#per_request: Box<Option<super::super::types::appmesh::VirtualNodeSpecListenerTimeoutHttp2PerRequest>>,
}
