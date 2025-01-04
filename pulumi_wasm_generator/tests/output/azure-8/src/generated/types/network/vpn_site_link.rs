#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct VpnSiteLink {
    /// A `bgp` block as defined above.
    /// 
    /// > **NOTE:** The `link.bgp` has to be set when the `address_cidrs` isn't specified.
    #[builder(into, default)]
    #[serde(rename = "bgp")]
    pub r#bgp: Box<Option<super::super::types::network::VpnSiteLinkBgp>>,
    /// The FQDN of this VPN Site Link.
    #[builder(into, default)]
    #[serde(rename = "fqdn")]
    pub r#fqdn: Box<Option<String>>,
    /// The ID of the VPN Site Link.
    #[builder(into, default)]
    #[serde(rename = "id")]
    pub r#id: Box<Option<String>>,
    /// The IP address of this VPN Site Link.
    /// 
    /// > **NOTE:** Either `fqdn` or `ip_address` should be specified.
    #[builder(into, default)]
    #[serde(rename = "ipAddress")]
    pub r#ip_address: Box<Option<String>>,
    /// The name which should be used for this VPN Site Link.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The name of the physical link at the VPN Site. Example: `ATT`, `Verizon`.
    #[builder(into, default)]
    #[serde(rename = "providerName")]
    pub r#provider_name: Box<Option<String>>,
    /// The speed of the VPN device at the branch location in unit of mbps. Defaults to `0`.
    #[builder(into, default)]
    #[serde(rename = "speedInMbps")]
    pub r#speed_in_mbps: Box<Option<i32>>,
}
