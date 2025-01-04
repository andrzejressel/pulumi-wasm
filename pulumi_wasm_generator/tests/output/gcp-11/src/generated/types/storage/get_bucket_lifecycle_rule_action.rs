#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetBucketLifecycleRuleAction {
    /// The target Storage Class of objects affected by this Lifecycle Rule. Supported values include: MULTI_REGIONAL, REGIONAL, NEARLINE, COLDLINE, ARCHIVE.
    #[builder(into)]
    #[serde(rename = "storageClass")]
    pub r#storage_class: Box<String>,
    /// The type of the action of this Lifecycle Rule. Supported values include: Delete, SetStorageClass and AbortIncompleteMultipartUpload.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
