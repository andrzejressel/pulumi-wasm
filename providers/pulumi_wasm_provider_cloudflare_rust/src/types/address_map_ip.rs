#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct AddressMapIp {
    /// An IPv4 or IPv6 address.
    #[serde(rename = "ip")]
    pub r#ip: Box<String>,
}
