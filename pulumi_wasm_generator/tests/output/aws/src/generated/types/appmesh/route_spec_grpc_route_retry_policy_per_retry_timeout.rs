#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RouteSpecGrpcRouteRetryPolicyPerRetryTimeout {
    /// Retry unit. Valid values: `ms`, `s`.
    #[builder(into)]
    #[serde(rename = "unit")]
    pub r#unit: Box<String>,
    /// Retry value.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Box<i32>,
}