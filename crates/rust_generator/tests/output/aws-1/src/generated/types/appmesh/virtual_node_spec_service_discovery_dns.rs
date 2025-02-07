#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct VirtualNodeSpecServiceDiscoveryDns {
    /// DNS host name for your virtual node.
    #[builder(into)]
    #[serde(rename = "hostname")]
    pub r#hostname: Box<String>,
    /// The preferred IP version that this virtual node uses. Valid values: `IPv6_PREFERRED`, `IPv4_PREFERRED`, `IPv4_ONLY`, `IPv6_ONLY`.
    #[builder(into, default)]
    #[serde(rename = "ipPreference")]
    pub r#ip_preference: Box<Option<String>>,
    /// The DNS response type for the virtual node. Valid values: `LOADBALANCER`, `ENDPOINTS`.
    #[builder(into, default)]
    #[serde(rename = "responseType")]
    pub r#response_type: Box<Option<String>>,
}
