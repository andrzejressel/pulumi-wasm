#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetIndexIndexStatisticTextDocumentStatistic {
    /// Total size, in bytes, of the indexed documents.
    #[builder(into)]
    #[serde(rename = "indexedTextBytes")]
    pub r#indexed_text_bytes: Box<i32>,
    /// The number of text documents indexed.
    #[builder(into)]
    #[serde(rename = "indexedTextDocumentsCount")]
    pub r#indexed_text_documents_count: Box<i32>,
}
