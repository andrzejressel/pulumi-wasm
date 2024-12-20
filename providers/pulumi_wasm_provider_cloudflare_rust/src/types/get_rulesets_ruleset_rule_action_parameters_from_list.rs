#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
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
