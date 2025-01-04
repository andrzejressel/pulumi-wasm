#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetApplicationGatewayBackendHttpSettingConnectionDraining {
    /// The number of seconds connection draining is active.
    #[builder(into)]
    #[serde(rename = "drainTimeoutSec")]
    pub r#drain_timeout_sec: Box<i32>,
    /// Is the Web Application Firewall enabled?
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<bool>,
}
