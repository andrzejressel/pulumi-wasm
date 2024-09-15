#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
pub struct AddressMapIp {
    /// An IPv4 or IPv6 address.
    #[builder(into)]
    #[serde(rename = "ip")]
    pub r#ip: Box<String>,
}
