#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PipeSourceParametersDynamodbStreamParametersDeadLetterConfig {
    /// ARN of this pipe.
    #[builder(into, default)]
    #[serde(rename = "arn")]
    pub r#arn: Box<Option<String>>,
}