#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FlowSourceFlowConfigSourceConnectorPropertiesVeeva {
    /// Document type specified in the Veeva document extract flow.
    #[builder(into, default)]
    #[serde(rename = "documentType")]
    pub r#document_type: Box<Option<String>>,
    /// Boolean value to include All Versions of files in Veeva document extract flow.
    #[builder(into, default)]
    #[serde(rename = "includeAllVersions")]
    pub r#include_all_versions: Box<Option<bool>>,
    /// Boolean value to include file renditions in Veeva document extract flow.
    #[builder(into, default)]
    #[serde(rename = "includeRenditions")]
    pub r#include_renditions: Box<Option<bool>>,
    /// Boolean value to include source files in Veeva document extract flow.
    #[builder(into, default)]
    #[serde(rename = "includeSourceFiles")]
    pub r#include_source_files: Box<Option<bool>>,
    #[builder(into)]
    #[serde(rename = "object")]
    pub r#object: Box<String>,
}
