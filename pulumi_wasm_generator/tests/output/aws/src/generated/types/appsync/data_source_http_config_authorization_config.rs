#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DataSourceHttpConfigAuthorizationConfig {
    /// Authorization type that the HTTP endpoint requires. Default values is `AWS_IAM`.
    #[builder(into, default)]
    #[serde(rename = "authorizationType")]
    pub r#authorization_type: Box<Option<String>>,
    /// Identity and Access Management (IAM) settings. See `aws_iam_config` Block for details.
    #[builder(into, default)]
    #[serde(rename = "awsIamConfig")]
    pub r#aws_iam_config: Box<Option<super::super::types::appsync::DataSourceHttpConfigAuthorizationConfigAwsIamConfig>>,
}
