#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct RulesetRuleActionParametersHeader {
    /// Use a value dynamically determined by the Firewall Rules expression language based on Wireshark display filters. Refer to the [Firewall Rules language](https://developers.cloudflare.com/firewall/cf-firewall-language) documentation for all available fields, operators, and functions.
    #[serde(rename = "expression")]
    pub r#expression: Box<Option<String>>,
    /// Name of the compression algorithm to use. Available values: `gzip`, `brotli`, `auto`, `default`, `none`
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    /// Action to perform on the HTTP request header. Available values: `remove`, `set`, `add`.
    #[serde(rename = "operation")]
    pub r#operation: Box<Option<String>>,
    /// Status code edge TTL value.
    #[serde(rename = "value")]
    pub r#value: Box<Option<String>>,
}
