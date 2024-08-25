#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct GetRulesetsRulesetRuleActionParametersFromValueTargetUrl {
    /// Use a value dynamically determined by the Firewall Rules expression language based on Wireshark display filters. Refer to the [Firewall Rules language](https://developers.cloudflare.com/firewall/cf-firewall-language) documentation for all available fields, operators, and functions. Conflicts with `"value"`.
    #[serde(rename = "expression")]
    pub r#expression: Box<Option<String>>,
    /// Static value to provide as the HTTP request header value. Conflicts with `"expression"`.
    #[serde(rename = "value")]
    pub r#value: Box<Option<String>>,
}
