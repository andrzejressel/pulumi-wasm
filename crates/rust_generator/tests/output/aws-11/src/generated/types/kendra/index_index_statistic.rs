#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct IndexIndexStatistic {
    /// A block that specifies the number of question and answer topics in the index. Detailed below.
    #[builder(into, default)]
    #[serde(rename = "faqStatistics")]
    pub r#faq_statistics: Box<Option<Vec<super::super::types::kendra::IndexIndexStatisticFaqStatistic>>>,
    /// A block that specifies the number of text documents indexed. Detailed below.
    #[builder(into, default)]
    #[serde(rename = "textDocumentStatistics")]
    pub r#text_document_statistics: Box<Option<Vec<super::super::types::kendra::IndexIndexStatisticTextDocumentStatistic>>>,
}
