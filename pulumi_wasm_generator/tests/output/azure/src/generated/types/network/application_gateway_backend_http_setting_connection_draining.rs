#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ApplicationGatewayBackendHttpSettingConnectionDraining {
    /// The number of seconds connection draining is active. Acceptable values are from `1` second to `3600` seconds.
    #[builder(into)]
    #[serde(rename = "drainTimeoutSec")]
    pub r#drain_timeout_sec: Box<i32>,
    /// If connection draining is enabled or not.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<bool>,
}