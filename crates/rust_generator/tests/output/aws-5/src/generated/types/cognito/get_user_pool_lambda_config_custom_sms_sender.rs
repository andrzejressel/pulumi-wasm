#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetUserPoolLambdaConfigCustomSmsSender {
    /// - ARN of the Lambda function.
    #[builder(into)]
    #[serde(rename = "lambdaArn")]
    pub r#lambda_arn: Box<String>,
    /// - Version of the Lambda function.
    #[builder(into)]
    #[serde(rename = "lambdaVersion")]
    pub r#lambda_version: Box<String>,
}
