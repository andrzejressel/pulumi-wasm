#[derive(serde::Serialize)]
pub struct RulesetRuleActionParametersSni {
    /// Status code edge TTL value.
    #[serde(rename = "value")]
    pub r#value: Box<Option<String>>,
}
