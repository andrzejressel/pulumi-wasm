#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct OsPolicyAssignmentOsPolicyResourceGroupResourceFileFileGcs {
    /// Bucket of the Cloud Storage object.
    #[builder(into)]
    #[serde(rename = "bucket")]
    pub r#bucket: Box<String>,
    /// Generation number of the Cloud Storage object.
    #[builder(into, default)]
    #[serde(rename = "generation")]
    pub r#generation: Box<Option<i32>>,
    /// Name of the Cloud Storage object.
    #[builder(into)]
    #[serde(rename = "object")]
    pub r#object: Box<String>,
}
