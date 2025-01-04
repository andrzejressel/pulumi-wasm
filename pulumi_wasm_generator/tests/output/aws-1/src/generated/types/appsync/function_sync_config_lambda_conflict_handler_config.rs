#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FunctionSyncConfigLambdaConflictHandlerConfig {
    /// ARN for the Lambda function to use as the Conflict Handler.
    #[builder(into, default)]
    #[serde(rename = "lambdaConflictHandlerArn")]
    pub r#lambda_conflict_handler_arn: Box<Option<String>>,
}
