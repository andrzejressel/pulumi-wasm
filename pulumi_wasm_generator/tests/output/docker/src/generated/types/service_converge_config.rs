#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ServiceConvergeConfig {
    /// The interval to check if the desired state is reached `(ms|s)`. Defaults to `7s`.
    #[builder(into, default)]
    #[serde(rename = "delay")]
    pub r#delay: Box<Option<String>>,
    /// The timeout of the service to reach the desired state `(s|m)`. Defaults to `3m`
    #[builder(into, default)]
    #[serde(rename = "timeout")]
    pub r#timeout: Box<Option<String>>,
}
