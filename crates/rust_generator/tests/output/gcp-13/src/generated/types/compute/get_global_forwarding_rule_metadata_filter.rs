#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetGlobalForwardingRuleMetadataFilter {
    /// The list of label value pairs that must match labels in the
    /// provided metadata based on filterMatchCriteria
    /// 
    /// This list must not be empty and can have at the most 64 entries.
    #[builder(into)]
    #[serde(rename = "filterLabels")]
    pub r#filter_labels: Box<Vec<super::super::types::compute::GetGlobalForwardingRuleMetadataFilterFilterLabel>>,
    /// Specifies how individual filterLabel matches within the list of
    /// filterLabels contribute towards the overall metadataFilter match.
    /// 
    /// MATCH_ANY - At least one of the filterLabels must have a matching
    /// label in the provided metadata.
    /// MATCH_ALL - All filterLabels must have matching labels in the
    /// provided metadata. Possible values: ["MATCH_ANY", "MATCH_ALL"]
    #[builder(into)]
    #[serde(rename = "filterMatchCriteria")]
    pub r#filter_match_criteria: Box<String>,
}
