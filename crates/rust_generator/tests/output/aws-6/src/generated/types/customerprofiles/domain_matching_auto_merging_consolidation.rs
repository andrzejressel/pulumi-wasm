#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct DomainMatchingAutoMergingConsolidation {
    /// A list of matching criteria.
    #[builder(into)]
    #[serde(rename = "matchingAttributesLists")]
    pub r#matching_attributes_lists: Box<Vec<Vec<String>>>,
}
