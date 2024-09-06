#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct UserAgentBlockingRuleConfiguration {
    /// The configuration target for this rule. You must set the target to ua for User Agent Blocking rules.
    #[serde(rename = "target")]
    pub r#target: Box<String>,
    /// The exact user agent string to match. This value will be compared to the received User-Agent HTTP header value.
    #[serde(rename = "value")]
    pub r#value: Box<String>,
}
