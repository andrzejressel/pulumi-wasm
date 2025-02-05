#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FolderBucketConfigIndexConfig {
    /// The LogEntry field path to index.
    /// Note that some paths are automatically indexed, and other paths are not eligible for indexing. See [indexing documentation](https://cloud.google.com/logging/docs/analyze/custom-index) for details.
    #[builder(into)]
    #[serde(rename = "fieldPath")]
    pub r#field_path: Box<String>,
    /// The type of data in this index. Allowed types include `INDEX_TYPE_UNSPECIFIED`, `INDEX_TYPE_STRING` and `INDEX_TYPE_INTEGER`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
