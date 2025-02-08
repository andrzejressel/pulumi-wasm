#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct EntryGcsFilesetSpecSampleGcsFileSpec {
    /// The full file path
    #[builder(into, default)]
    #[serde(rename = "filePath")]
    pub r#file_path: Box<Option<String>>,
    /// The size of the file, in bytes.
    #[builder(into, default)]
    #[serde(rename = "sizeBytes")]
    pub r#size_bytes: Box<Option<i32>>,
}
