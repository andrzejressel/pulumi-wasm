#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct SdkvoiceSipMediaApplicationEndpoints {
    /// Valid Amazon Resource Name (ARN) of the Lambda function, version, or alias. The function must be created in the same AWS Region as the SIP media application.
    #[builder(into)]
    #[serde(rename = "lambdaArn")]
    pub r#lambda_arn: Box<String>,
}
