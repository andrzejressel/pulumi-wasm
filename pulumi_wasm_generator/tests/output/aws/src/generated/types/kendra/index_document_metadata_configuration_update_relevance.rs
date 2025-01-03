#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct IndexDocumentMetadataConfigurationUpdateRelevance {
    /// Specifies the time period that the boost applies to. For more information, refer to [Duration](https://docs.aws.amazon.com/kendra/latest/dg/API_Relevance.html#Kendra-Type-Relevance-Duration).
    #[builder(into, default)]
    #[serde(rename = "duration")]
    pub r#duration: Box<Option<String>>,
    /// Indicates that this field determines how "fresh" a document is. For more information, refer to [Freshness](https://docs.aws.amazon.com/kendra/latest/dg/API_Relevance.html#Kendra-Type-Relevance-Freshness).
    #[builder(into, default)]
    #[serde(rename = "freshness")]
    pub r#freshness: Box<Option<bool>>,
    /// The relative importance of the field in the search. Larger numbers provide more of a boost than smaller numbers. Minimum value of 1. Maximum value of 10.
    #[builder(into, default)]
    #[serde(rename = "importance")]
    pub r#importance: Box<Option<i32>>,
    /// Determines how values should be interpreted. For more information, refer to [RankOrder](https://docs.aws.amazon.com/kendra/latest/dg/API_Relevance.html#Kendra-Type-Relevance-RankOrder).
    #[builder(into, default)]
    #[serde(rename = "rankOrder")]
    pub r#rank_order: Box<Option<String>>,
    /// A list of values that should be given a different boost when they appear in the result list. For more information, refer to [ValueImportanceMap](https://docs.aws.amazon.com/kendra/latest/dg/API_Relevance.html#Kendra-Type-Relevance-ValueImportanceMap).
    #[builder(into, default)]
    #[serde(rename = "valuesImportanceMap")]
    pub r#values_importance_map: Box<Option<std::collections::HashMap<String, i32>>>,
}
