#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct EndpointGlobalDeliveryRuleCacheExpirationAction {
    /// The behavior of the cache. Valid values are `BypassCache`, `Override` and `SetIfMissing`.
    #[builder(into)]
    #[serde(rename = "behavior")]
    pub r#behavior: Box<String>,
    /// Duration of the cache. Only allowed when `behavior` is set to `Override` or `SetIfMissing`. Format: `[d.]hh:mm:ss`
    #[builder(into, default)]
    #[serde(rename = "duration")]
    pub r#duration: Box<Option<String>>,
}