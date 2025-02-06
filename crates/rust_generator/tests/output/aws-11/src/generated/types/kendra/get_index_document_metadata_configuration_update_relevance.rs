#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetIndexDocumentMetadataConfigurationUpdateRelevance {
    /// Time period that the boost applies to. For more information, refer to [Duration](https://docs.aws.amazon.com/kendra/latest/APIReference/API_Relevance.html#Kendra-Type-Relevance-Duration).
    #[builder(into)]
    #[serde(rename = "duration")]
    pub r#duration: Box<String>,
    /// How "fresh" a document is. For more information, refer to [Freshness](https://docs.aws.amazon.com/kendra/latest/APIReference/API_Relevance.html#Kendra-Type-Relevance-Freshness).
    #[builder(into)]
    #[serde(rename = "freshness")]
    pub r#freshness: Box<bool>,
    /// Relative importance of the field in the search. Larger numbers provide more of a boost than smaller numbers. Minimum value of 1. Maximum value of 10.
    #[builder(into)]
    #[serde(rename = "importance")]
    pub r#importance: Box<i32>,
    /// Determines how values should be interpreted. For more information, refer to [RankOrder](https://docs.aws.amazon.com/kendra/latest/APIReference/API_Relevance.html#Kendra-Type-Relevance-RankOrder).
    #[builder(into)]
    #[serde(rename = "rankOrder")]
    pub r#rank_order: Box<String>,
    /// A list of values that should be given a different boost when they appear in the result list. For more information, refer to [ValueImportanceMap](https://docs.aws.amazon.com/kendra/latest/APIReference/API_Relevance.html#Kendra-Type-Relevance-ValueImportanceMap).
    #[builder(into)]
    #[serde(rename = "valuesImportanceMap")]
    pub r#values_importance_map: Box<std::collections::HashMap<String, i32>>,
}
