#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DataSourceEventBridgeConfig {
    /// ARN for the EventBridge bus.
    #[builder(into)]
    #[serde(rename = "eventBusArn")]
    pub r#event_bus_arn: Box<String>,
}