#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ApiConfigGatewayConfig {
    /// Backend settings that are applied to all backends of the Gateway.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "backendConfig")]
    pub r#backend_config: Box<super::super::types::apigateway::ApiConfigGatewayConfigBackendConfig>,
}
