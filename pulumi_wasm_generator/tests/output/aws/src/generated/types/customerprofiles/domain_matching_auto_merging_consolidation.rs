#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DomainMatchingAutoMergingConsolidation {
    /// A list of matching criteria.
    #[builder(into)]
    #[serde(rename = "matchingAttributesLists")]
    pub r#matching_attributes_lists: Box<Vec<Vec<String>>>,
}