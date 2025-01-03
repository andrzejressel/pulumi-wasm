#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct EndpointClientConnectOptions {
    /// Indicates whether client connect options are enabled. The default is `false` (not enabled).
    #[builder(into, default)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
    /// The Amazon Resource Name (ARN) of the Lambda function used for connection authorization.
    #[builder(into, default)]
    #[serde(rename = "lambdaFunctionArn")]
    pub r#lambda_function_arn: Box<Option<String>>,
}
