#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PreventionJobTriggerInspectJobInspectConfigRuleSetRuleExclusionRuleRegex {
    /// The index of the submatch to extract as findings. When not specified, the entire match is returned. No more than 3 may be included.
    #[builder(into, default)]
    #[serde(rename = "groupIndexes")]
    pub r#group_indexes: Box<Option<Vec<i32>>>,
    /// Pattern defining the regular expression.
    /// Its syntax (https://github.com/google/re2/wiki/Syntax) can be found under the google/re2 repository on GitHub.
    #[builder(into)]
    #[serde(rename = "pattern")]
    pub r#pattern: Box<String>,
}
