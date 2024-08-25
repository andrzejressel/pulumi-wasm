#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct AccessRuleConfiguration {
    /// The request property to target. Available values: `ip`, `ip6`, `ip_range`, `asn`, `country`. **Modifying this attribute will force creation of a new resource.**
    #[serde(rename = "target")]
    pub r#target: Box<String>,
    /// The value to target. Depends on target's type. **Modifying this attribute will force creation of a new resource.**
    #[serde(rename = "value")]
    pub r#value: Box<String>,
}
