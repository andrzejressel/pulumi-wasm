#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
pub struct RulesetRuleExposedCredentialCheck {
    /// Firewall Rules expression language based on Wireshark display filters for where to check for the "password" value. Refer to the [Firewall Rules language](https://developers.cloudflare.com/firewall/cf-firewall-language).
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "passwordExpression")]
    pub r#password_expression: Box<Option<String>>,
    /// Firewall Rules expression language based on Wireshark display filters for where to check for the "username" value. Refer to the [Firewall Rules language](https://developers.cloudflare.com/firewall/cf-firewall-language).
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "usernameExpression")]
    pub r#username_expression: Box<Option<String>>,
}
