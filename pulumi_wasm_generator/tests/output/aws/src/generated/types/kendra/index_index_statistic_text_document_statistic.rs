#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct IndexIndexStatisticTextDocumentStatistic {
    /// The total size, in bytes, of the indexed documents.
    #[builder(into, default)]
    #[serde(rename = "indexedTextBytes")]
    pub r#indexed_text_bytes: Box<Option<i32>>,
    /// The number of text documents indexed.
    #[builder(into, default)]
    #[serde(rename = "indexedTextDocumentsCount")]
    pub r#indexed_text_documents_count: Box<Option<i32>>,
}