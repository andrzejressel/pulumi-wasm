#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetClusterNodeConfigEffectiveTaint {
    /// Effect for taint.
    #[builder(into)]
    #[serde(rename = "effect")]
    pub r#effect: Box<String>,
    /// Key for taint.
    #[builder(into)]
    #[serde(rename = "key")]
    pub r#key: Box<String>,
    /// Value for taint.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Box<String>,
}
