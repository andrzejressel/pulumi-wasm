#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct InfrastructureAccessTargetIp {
    /// The target's IPv4 address.
    #[builder(into, default)]
    #[serde(rename = "ipv4")]
    pub r#ipv_4: Box<Option<crate::types::InfrastructureAccessTargetIpIpv4>>,
    /// The target's IPv6 address.
    #[builder(into, default)]
    #[serde(rename = "ipv6")]
    pub r#ipv_6: Box<Option<crate::types::InfrastructureAccessTargetIpIpv6>>,
}
