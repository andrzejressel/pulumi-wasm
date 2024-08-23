#[derive(serde::Serialize)]
pub struct GetNetworkIpamConfig {
    #[serde(rename = "auxAddress")]
    pub r#aux_address: Box<Option<std::collections::HashMap<String, String>>>,
    #[serde(rename = "gateway")]
    pub r#gateway: Box<Option<String>>,
    #[serde(rename = "ipRange")]
    pub r#ip_range: Box<Option<String>>,
    #[serde(rename = "subnet")]
    pub r#subnet: Box<Option<String>>,
}
