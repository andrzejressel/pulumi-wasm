#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct TeamsRuleRuleSettingsDnsResolversIpv6 {
    /// The IPv4 or IPv6 address of the upstream resolver.
    #[builder(into)]
    #[serde(rename = "ip")]
    pub r#ip: Box<String>,
    /// A port number to use for the upstream resolver. Defaults to `53`.
    #[builder(into, default)]
    #[serde(rename = "port")]
    pub r#port: Box<Option<i32>>,
    /// Whether to connect to this resolver over a private network. Must be set when `vnet_id` is set.
    #[builder(into, default)]
    #[serde(rename = "routeThroughPrivateNetwork")]
    pub r#route_through_private_network: Box<Option<bool>>,
    /// specify a virtual network for this resolver. Uses default virtual network id if omitted.
    #[builder(into, default)]
    #[serde(rename = "vnetId")]
    pub r#vnet_id: Box<Option<String>>,
}
