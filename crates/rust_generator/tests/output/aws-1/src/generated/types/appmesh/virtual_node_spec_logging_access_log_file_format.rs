#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct VirtualNodeSpecLoggingAccessLogFileFormat {
    /// The logging format for JSON.
    #[builder(into, default)]
    #[serde(rename = "jsons")]
    pub r#jsons: Box<Option<Vec<super::super::types::appmesh::VirtualNodeSpecLoggingAccessLogFileFormatJson>>>,
    /// The logging format for text. Must be between 1 and 1000 characters in length.
    #[builder(into, default)]
    #[serde(rename = "text")]
    pub r#text: Box<Option<String>>,
}
