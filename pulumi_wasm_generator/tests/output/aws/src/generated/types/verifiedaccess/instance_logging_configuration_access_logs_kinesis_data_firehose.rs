#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct InstanceLoggingConfigurationAccessLogsKinesisDataFirehose {
    /// The name of the delivery stream.
    #[builder(into, default)]
    #[serde(rename = "deliveryStream")]
    pub r#delivery_stream: Box<Option<String>>,
    /// Indicates whether logging is enabled.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<bool>,
}
