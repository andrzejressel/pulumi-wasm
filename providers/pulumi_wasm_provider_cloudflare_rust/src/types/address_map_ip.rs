#[derive(serde::Serialize)]
pub struct AddressMapIp {
    #[serde(rename = "ip")]
    pub r#ip: Box<String>,
}
