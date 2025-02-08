#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DataStoreDocumentProcessingConfigChunkingConfigLayoutBasedChunkingConfig {
    /// The token size limit for each chunk.
    /// Supported values: 100-500 (inclusive). Default value: 500.
    #[builder(into, default)]
    #[serde(rename = "chunkSize")]
    pub r#chunk_size: Box<Option<i32>>,
    /// Whether to include appending different levels of headings to chunks from the middle of the document to prevent context loss.
    /// Default value: False.
    #[builder(into, default)]
    #[serde(rename = "includeAncestorHeadings")]
    pub r#include_ancestor_headings: Box<Option<bool>>,
}
