#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetIndexIndexStatistic {
    /// Block that specifies the number of question and answer topics in the index. Documented below.
    #[builder(into)]
    #[serde(rename = "faqStatistics")]
    pub r#faq_statistics: Box<Vec<super::super::types::kendra::GetIndexIndexStatisticFaqStatistic>>,
    /// A block that specifies the number of text documents indexed.
    #[builder(into)]
    #[serde(rename = "textDocumentStatistics")]
    pub r#text_document_statistics: Box<Vec<super::super::types::kendra::GetIndexIndexStatisticTextDocumentStatistic>>,
}