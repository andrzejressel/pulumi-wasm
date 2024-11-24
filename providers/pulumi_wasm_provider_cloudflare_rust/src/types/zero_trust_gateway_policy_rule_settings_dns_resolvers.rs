#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
pub struct ZeroTrustGatewayPolicyRuleSettingsDnsResolvers {
    /// IPv4 resolvers.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "ipv4s")]
    pub r#ipv_4_s: Box<Option<Vec<crate::types::ZeroTrustGatewayPolicyRuleSettingsDnsResolversIpv4>>>,
    /// IPv6 resolvers.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "ipv6s")]
    pub r#ipv_6_s: Box<Option<Vec<crate::types::ZeroTrustGatewayPolicyRuleSettingsDnsResolversIpv6>>>,
}
