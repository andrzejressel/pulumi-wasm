#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct GetRulesetsRulesetRuleExposedCredentialCheck {
    /// Firewall Rules expression language based on Wireshark display filters for where to check for the "password" value. Refer to the [Firewall Rules language](https://developers.cloudflare.com/firewall/cf-firewall-language).
    #[builder(into, default)]
    #[serde(rename = "passwordExpression")]
    pub r#password_expression: Box<Option<String>>,
    /// Firewall Rules expression language based on Wireshark display filters for where to check for the "username" value. Refer to the [Firewall Rules language](https://developers.cloudflare.com/firewall/cf-firewall-language).
    #[builder(into, default)]
    #[serde(rename = "usernameExpression")]
    pub r#username_expression: Box<Option<String>>,
}