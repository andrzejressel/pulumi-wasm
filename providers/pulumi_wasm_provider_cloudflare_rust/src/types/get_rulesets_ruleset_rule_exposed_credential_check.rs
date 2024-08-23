#[derive(serde::Serialize)]
pub struct GetRulesetsRulesetRuleExposedCredentialCheck {
    #[serde(rename = "passwordExpression")]
    pub r#password_expression: Box<Option<String>>,
    #[serde(rename = "usernameExpression")]
    pub r#username_expression: Box<Option<String>>,
}
