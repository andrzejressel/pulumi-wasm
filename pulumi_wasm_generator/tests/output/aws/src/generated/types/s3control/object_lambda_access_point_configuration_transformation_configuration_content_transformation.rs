#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ObjectLambdaAccessPointConfigurationTransformationConfigurationContentTransformation {
    /// Configuration for an AWS Lambda function. See AWS Lambda below for more details.
    #[builder(into)]
    #[serde(rename = "awsLambda")]
    pub r#aws_lambda: Box<super::super::types::s3control::ObjectLambdaAccessPointConfigurationTransformationConfigurationContentTransformationAwsLambda>,
}