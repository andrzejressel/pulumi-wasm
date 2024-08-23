#[derive(serde::Serialize)]
pub struct RulesetRuleActionParametersMatchedData {
    #[serde(rename = "publicKey")]
    pub r#public_key: Box<Option<String>>,
}
