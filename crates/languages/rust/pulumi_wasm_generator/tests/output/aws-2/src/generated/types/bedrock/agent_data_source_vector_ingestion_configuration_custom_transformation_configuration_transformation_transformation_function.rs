#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AgentDataSourceVectorIngestionConfigurationCustomTransformationConfigurationTransformationTransformationFunction {
    /// The configuration of the lambda function.
    #[builder(into, default)]
    #[serde(rename = "transformationLambdaConfiguration")]
    pub r#transformation_lambda_configuration: Box<Option<super::super::types::bedrock::AgentDataSourceVectorIngestionConfigurationCustomTransformationConfigurationTransformationTransformationFunctionTransformationLambdaConfiguration>>,
}
