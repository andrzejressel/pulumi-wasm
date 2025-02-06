#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct UserPoolLambdaConfigPreTokenGenerationConfig {
    #[builder(into)]
    #[serde(rename = "lambdaArn")]
    pub r#lambda_arn: Box<String>,
    #[builder(into)]
    #[serde(rename = "lambdaVersion")]
    pub r#lambda_version: Box<String>,
}
