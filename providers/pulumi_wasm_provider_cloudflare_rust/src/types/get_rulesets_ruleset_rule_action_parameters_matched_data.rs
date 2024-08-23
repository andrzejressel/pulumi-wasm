#[derive(serde::Serialize)]
pub struct GetRulesetsRulesetRuleActionParametersMatchedData {
    #[serde(rename = "publicKey")]
    pub r#public_key: Box<Option<String>>,
}
