#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AuthzPolicyHttpRuleFromSourceResourceTagValueIdSet {
    /// A list of resource tag value permanent IDs to match against the resource manager tags value associated with the source VM of a request. The match follows AND semantics which means all the ids must match.
    /// Limited to 5 matches.
    #[builder(into, default)]
    #[serde(rename = "ids")]
    pub r#ids: Box<Option<Vec<String>>>,
}
