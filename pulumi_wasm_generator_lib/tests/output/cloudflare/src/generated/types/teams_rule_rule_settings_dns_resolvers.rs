#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct TeamsRuleRuleSettingsDnsResolvers {
    /// IPv4 resolvers.
    #[builder(into, default)]
    #[serde(rename = "ipv4s")]
    pub r#ipv_4_s: Box<Option<Vec<super::types::TeamsRuleRuleSettingsDnsResolversIpv4>>>,
    /// IPv6 resolvers.
    #[builder(into, default)]
    #[serde(rename = "ipv6s")]
    pub r#ipv_6_s: Box<Option<Vec<super::types::TeamsRuleRuleSettingsDnsResolversIpv6>>>,
}
