#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct TopicRuleErrorActionLambda {
    /// The ARN of the Lambda function.
    #[builder(into)]
    #[serde(rename = "functionArn")]
    pub r#function_arn: Box<String>,
}
