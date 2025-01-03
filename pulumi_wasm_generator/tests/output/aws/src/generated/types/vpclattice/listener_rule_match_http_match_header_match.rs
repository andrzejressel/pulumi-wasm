#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ListenerRuleMatchHttpMatchHeaderMatch {
    /// Indicates whether the match is case sensitive. Defaults to false.
    #[builder(into, default)]
    #[serde(rename = "caseSensitive")]
    pub r#case_sensitive: Box<Option<bool>>,
    /// The header match type.
    #[builder(into)]
    #[serde(rename = "match")]
    pub r#match_: Box<super::super::types::vpclattice::ListenerRuleMatchHttpMatchHeaderMatchMatch>,
    /// The name of the header.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}
