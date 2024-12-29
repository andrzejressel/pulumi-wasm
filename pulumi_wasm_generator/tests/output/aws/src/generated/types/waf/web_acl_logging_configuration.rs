#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct WebAclLoggingConfiguration {
    /// Amazon Resource Name (ARN) of Kinesis Firehose Delivery Stream
    #[builder(into)]
    #[serde(rename = "logDestination")]
    pub r#log_destination: Box<String>,
    /// Configuration block containing parts of the request that you want redacted from the logs. Detailed below.
    #[builder(into, default)]
    #[serde(rename = "redactedFields")]
    pub r#redacted_fields: Box<Option<super::super::types::waf::WebAclLoggingConfigurationRedactedFields>>,
}
