#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
pub struct GetZeroTrustInfrastructureAccessTargetsTargetIp {
    /// The target's IPv4 address.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "ipv4")]
    pub r#ipv_4: Box<Option<crate::types::GetZeroTrustInfrastructureAccessTargetsTargetIpIpv4>>,
    /// The target's IPv6 address.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "ipv6")]
    pub r#ipv_6: Box<Option<crate::types::GetZeroTrustInfrastructureAccessTargetsTargetIpIpv6>>,
}