#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AgentDataSourceVectorIngestionConfigurationCustomTransformationConfigurationTransformation {
    /// When the service applies the transformation. Currently only `POST_CHUNKING` is supported.
    #[builder(into)]
    #[serde(rename = "stepToApply")]
    pub r#step_to_apply: Box<String>,
    /// The lambda function that processes documents.
    #[builder(into, default)]
    #[serde(rename = "transformationFunction")]
    pub r#transformation_function: Box<Option<super::super::types::bedrock::AgentDataSourceVectorIngestionConfigurationCustomTransformationConfigurationTransformationTransformationFunction>>,
}