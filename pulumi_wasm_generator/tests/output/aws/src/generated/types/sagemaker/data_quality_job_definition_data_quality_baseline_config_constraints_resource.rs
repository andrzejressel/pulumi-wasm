#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DataQualityJobDefinitionDataQualityBaselineConfigConstraintsResource {
    /// The Amazon S3 URI for the constraints resource.
    #[builder(into, default)]
    #[serde(rename = "s3Uri")]
    pub r#s_3_uri: Box<Option<String>>,
}
