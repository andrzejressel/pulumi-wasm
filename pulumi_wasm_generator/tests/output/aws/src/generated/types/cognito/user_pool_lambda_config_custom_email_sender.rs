#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct UserPoolLambdaConfigCustomEmailSender {
    /// The Lambda Amazon Resource Name of the Lambda function that Amazon Cognito triggers to send email notifications to users.
    #[builder(into)]
    #[serde(rename = "lambdaArn")]
    pub r#lambda_arn: Box<String>,
    /// The Lambda version represents the signature of the "request" attribute in the "event" information Amazon Cognito passes to your custom email Lambda function. The only supported value is `V1_0`.
    #[builder(into)]
    #[serde(rename = "lambdaVersion")]
    pub r#lambda_version: Box<String>,
}
