#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct EntityRecognizerInputDataConfigAnnotations {
    /// Location of training annotations.
    #[builder(into)]
    #[serde(rename = "s3Uri")]
    pub r#s_3_uri: Box<String>,
    #[builder(into, default)]
    #[serde(rename = "testS3Uri")]
    pub r#test_s_3_uri: Box<Option<String>>,
}