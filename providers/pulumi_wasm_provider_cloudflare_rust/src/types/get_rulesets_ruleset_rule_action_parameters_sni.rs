#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct GetRulesetsRulesetRuleActionParametersSni {
    /// Value to define for SNI.
    #[serde(rename = "value")]
    pub r#value: Box<Option<String>>,
}
