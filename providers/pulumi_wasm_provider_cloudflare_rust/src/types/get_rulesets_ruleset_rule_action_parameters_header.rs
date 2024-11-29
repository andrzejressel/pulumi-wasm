#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
pub struct GetRulesetsRulesetRuleActionParametersHeader {
    /// Use a value dynamically determined by the Firewall Rules expression language based on Wireshark display filters. Refer to the [Firewall Rules language](https://developers.cloudflare.com/firewall/cf-firewall-language) documentation for all available fields, operators, and functions. Conflicts with `"value"`.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "expression")]
    pub r#expression: Box<Option<String>>,
    /// Name of the HTTP request header to target.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    /// Action to perform on the HTTP request header. Available values: `remove`, `set`, `add`
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "operation")]
    pub r#operation: Box<Option<String>>,
    /// Static value to provide as the HTTP request header value. Conflicts with `"expression"`.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "value")]
    pub r#value: Box<Option<String>>,
}
