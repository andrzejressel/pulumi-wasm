#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct NetworkIpamConfig {
    /// Auxiliary IPv4 or IPv6 addresses used by Network driver
    #[serde(rename = "auxAddress")]
    pub r#aux_address: Box<Option<std::collections::HashMap<String, String>>>,
    /// The IP address of the gateway
    #[serde(rename = "gateway")]
    pub r#gateway: Box<Option<String>>,
    /// The ip range in CIDR form
    #[serde(rename = "ipRange")]
    pub r#ip_range: Box<Option<String>>,
    /// The subnet in CIDR form
    #[serde(rename = "subnet")]
    pub r#subnet: Box<Option<String>>,
}
