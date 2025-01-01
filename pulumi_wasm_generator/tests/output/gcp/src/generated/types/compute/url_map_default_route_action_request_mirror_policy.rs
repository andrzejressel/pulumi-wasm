#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct UrlMapDefaultRouteActionRequestMirrorPolicy {
    /// The full or partial URL to the BackendService resource being mirrored to.
    #[builder(into)]
    #[serde(rename = "backendService")]
    pub r#backend_service: Box<String>,
}
