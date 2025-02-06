#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct SnapshotImportClientData {
    /// A user-defined comment about the disk upload.
    #[builder(into, default)]
    #[serde(rename = "comment")]
    pub r#comment: Box<Option<String>>,
    /// The time that the disk upload ends.
    #[builder(into, default)]
    #[serde(rename = "uploadEnd")]
    pub r#upload_end: Box<Option<String>>,
    /// The size of the uploaded disk image, in GiB.
    #[builder(into, default)]
    #[serde(rename = "uploadSize")]
    pub r#upload_size: Box<Option<f64>>,
    /// The time that the disk upload starts.
    #[builder(into, default)]
    #[serde(rename = "uploadStart")]
    pub r#upload_start: Box<Option<String>>,
}
