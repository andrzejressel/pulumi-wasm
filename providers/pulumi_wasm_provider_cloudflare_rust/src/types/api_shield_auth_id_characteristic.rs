#[derive(serde::Serialize)]
pub struct ApiShieldAuthIdCharacteristic {
    /// The name of the characteristic.
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    /// The type of characteristic. Available values: `header`, `cookie`.
    #[serde(rename = "type")]
    pub r#type: Box<Option<String>>,
}
