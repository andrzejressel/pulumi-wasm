#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DataStoreDocumentProcessingConfig {
    /// Whether chunking mode is enabled.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "chunkingConfig")]
    pub r#chunking_config: Box<Option<super::super::types::discoveryengine::DataStoreDocumentProcessingConfigChunkingConfig>>,
    /// Configurations for default Document parser. If not specified, this resource
    /// will be configured to use a default DigitalParsingConfig, and the default parsing
    /// config will be applied to all file types for Document parsing.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "defaultParsingConfig")]
    pub r#default_parsing_config: Box<Option<super::super::types::discoveryengine::DataStoreDocumentProcessingConfigDefaultParsingConfig>>,
    /// (Output)
    /// The full resource name of the Document Processing Config. Format:
    /// `projects/{project}/locations/{location}/collections/{collection_id}/dataStores/{data_store_id}/documentProcessingConfig`.
    #[builder(into, default)]
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    /// Map from file type to override the default parsing configuration based on the file type. Supported keys:
    #[builder(into, default)]
    #[serde(rename = "parsingConfigOverrides")]
    pub r#parsing_config_overrides: Box<Option<Vec<super::super::types::discoveryengine::DataStoreDocumentProcessingConfigParsingConfigOverride>>>,
}
