#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
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
