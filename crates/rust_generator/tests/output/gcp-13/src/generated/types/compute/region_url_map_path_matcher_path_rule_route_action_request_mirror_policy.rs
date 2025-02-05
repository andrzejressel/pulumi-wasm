#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RegionUrlMapPathMatcherPathRuleRouteActionRequestMirrorPolicy {
    /// The full or partial URL to the RegionBackendService resource being mirrored to.
    /// The backend service configured for a mirroring policy must reference backends that are of the same type as the original backend service matched in the URL map.
    /// Serverless NEG backends are not currently supported as a mirrored backend service.
    #[builder(into)]
    #[serde(rename = "backendService")]
    pub r#backend_service: Box<String>,
}
