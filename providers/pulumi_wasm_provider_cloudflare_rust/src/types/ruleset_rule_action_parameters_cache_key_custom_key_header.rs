#[derive(serde::Serialize)]
pub struct RulesetRuleActionParametersCacheKeyCustomKeyHeader {
    #[serde(rename = "checkPresences")]
    pub r#check_presences: Box<Option<Vec<String>>>,
    #[serde(rename = "excludeOrigin")]
    pub r#exclude_origin: Box<Option<bool>>,
    #[serde(rename = "includes")]
    pub r#includes: Box<Option<Vec<String>>>,
}
