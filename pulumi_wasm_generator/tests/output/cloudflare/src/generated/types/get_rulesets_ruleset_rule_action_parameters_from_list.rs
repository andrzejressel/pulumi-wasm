#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetRulesetsRulesetRuleActionParametersFromList {
    /// Expression to use for the list lookup.
    #[builder(into)]
    #[serde(rename = "key")]
    pub r#key: Box<String>,
    /// Name of the list.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}