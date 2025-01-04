#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetConfigurationProfileValidator {
    /// Either the JSON Schema content or the ARN of an AWS Lambda function.
    #[builder(into)]
    #[serde(rename = "content")]
    pub r#content: Box<String>,
    /// Type of validator. Valid values: JSON_SCHEMA and LAMBDA.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
