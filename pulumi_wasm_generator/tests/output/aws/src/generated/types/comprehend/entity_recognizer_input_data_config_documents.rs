#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct EntityRecognizerInputDataConfigDocuments {
    /// Specifies how the input files should be processed.
    /// One of `ONE_DOC_PER_LINE` or `ONE_DOC_PER_FILE`.
    #[builder(into, default)]
    #[serde(rename = "inputFormat")]
    pub r#input_format: Box<Option<String>>,
    /// Location of training documents.
    #[builder(into)]
    #[serde(rename = "s3Uri")]
    pub r#s_3_uri: Box<String>,
    #[builder(into, default)]
    #[serde(rename = "testS3Uri")]
    pub r#test_s_3_uri: Box<Option<String>>,
}