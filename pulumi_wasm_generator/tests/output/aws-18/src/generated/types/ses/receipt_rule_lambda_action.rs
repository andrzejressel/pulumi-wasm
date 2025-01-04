#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ReceiptRuleLambdaAction {
    /// The ARN of the Lambda function to invoke
    #[builder(into)]
    #[serde(rename = "functionArn")]
    pub r#function_arn: Box<String>,
    /// `Event` or `RequestResponse`
    #[builder(into, default)]
    #[serde(rename = "invocationType")]
    pub r#invocation_type: Box<Option<String>>,
    /// The position of the action in the receipt rule
    #[builder(into)]
    #[serde(rename = "position")]
    pub r#position: Box<i32>,
    /// The ARN of an SNS topic to notify
    #[builder(into, default)]
    #[serde(rename = "topicArn")]
    pub r#topic_arn: Box<Option<String>>,
}
