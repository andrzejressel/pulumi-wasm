#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct GetRulesetsRulesetRuleActionParametersUriQuery {
    /// Expression that defines the updated (dynamic) value of the URI path or query string component. Uses the Firewall Rules expression language based on Wireshark display filters. Refer to the [Firewall Rules language](https://developers.cloudflare.com/firewall/cf-firewall-language) documentation for all available fields, operators, and functions
    #[serde(rename = "expression")]
    pub r#expression: Box<Option<String>>,
    /// Static string value of the updated URI path or query string component.
    #[serde(rename = "value")]
    pub r#value: Box<Option<String>>,
}
