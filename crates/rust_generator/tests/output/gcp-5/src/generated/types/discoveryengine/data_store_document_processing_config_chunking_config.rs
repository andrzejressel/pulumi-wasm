#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DataStoreDocumentProcessingConfigChunkingConfig {
    /// Configuration for the layout based chunking.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "layoutBasedChunkingConfig")]
    pub r#layout_based_chunking_config: Box<Option<super::super::types::discoveryengine::DataStoreDocumentProcessingConfigChunkingConfigLayoutBasedChunkingConfig>>,
}
