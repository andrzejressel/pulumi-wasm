#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct DataStoreDocumentProcessingConfigDefaultParsingConfig {
    /// Configurations applied to digital parser.
    #[builder(into, default)]
    #[serde(rename = "digitalParsingConfig")]
    pub r#digital_parsing_config: Box<Option<super::super::types::discoveryengine::DataStoreDocumentProcessingConfigDefaultParsingConfigDigitalParsingConfig>>,
    /// Configurations applied to layout parser.
    #[builder(into, default)]
    #[serde(rename = "layoutParsingConfig")]
    pub r#layout_parsing_config: Box<Option<super::super::types::discoveryengine::DataStoreDocumentProcessingConfigDefaultParsingConfigLayoutParsingConfig>>,
    /// Configurations applied to OCR parser. Currently it only applies to PDFs.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "ocrParsingConfig")]
    pub r#ocr_parsing_config: Box<Option<super::super::types::discoveryengine::DataStoreDocumentProcessingConfigDefaultParsingConfigOcrParsingConfig>>,
}
