#[derive(serde::Serialize)]
pub struct ApiShieldAuthIdCharacteristic {
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    #[serde(rename = "type")]
    pub r#type: Box<Option<String>>,
}
