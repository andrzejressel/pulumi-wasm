#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct RulesetRuleActionParametersUriPath {
    /// Use a value dynamically determined by the Firewall Rules expression language based on Wireshark display filters. Refer to the [Firewall Rules language](https://developers.cloudflare.com/firewall/cf-firewall-language) documentation for all available fields, operators, and functions.
    #[serde(rename = "expression")]
    pub r#expression: Box<Option<String>>,
    /// Status code edge TTL value.
    #[serde(rename = "value")]
    pub r#value: Box<Option<String>>,
}
