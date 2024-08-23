#[derive(serde::Serialize)]
pub struct GetRulesetsRulesetRuleActionParametersCacheKeyCustomKeyCookie {
    #[serde(rename = "checkPresences")]
    pub r#check_presences: Box<Option<Vec<String>>>,
    #[serde(rename = "includes")]
    pub r#includes: Box<Option<Vec<String>>>,
}
