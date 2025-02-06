#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ContinuousDeploymentPolicyTrafficConfigSingleWeightConfigSessionStickinessConfig {
    /// The amount of time in seconds after which sessions will cease if no requests are received. Valid values are `300` – `3600` (5–60 minutes). The value must be less than or equal to `maximum_ttl`.
    #[builder(into)]
    #[serde(rename = "idleTtl")]
    pub r#idle_ttl: Box<i32>,
    /// The maximum amount of time in seconds to consider requests from the viewer as being part of the same session. Valid values are `300` – `3600` (5–60 minutes). The value must be greater than or equal to `idle_ttl`.
    #[builder(into)]
    #[serde(rename = "maximumTtl")]
    pub r#maximum_ttl: Box<i32>,
}
