#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DataStoreDocumentProcessingConfigParsingConfigOverrideOcrParsingConfig {
    /// If true, will use native text instead of OCR text on pages containing native text.
    #[builder(into, default)]
    #[serde(rename = "useNativeText")]
    pub r#use_native_text: Box<Option<bool>>,
}
