#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FeatureSpecFleetobservabilityLoggingConfigDefaultConfig {
    /// Specified if fleet logging feature is enabled.
    /// Possible values are: `MODE_UNSPECIFIED`, `COPY`, `MOVE`.
    #[builder(into, default)]
    #[serde(rename = "mode")]
    pub r#mode: Box<Option<String>>,
}
