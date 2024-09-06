#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct GetZonesZone {
    /// The zone ID.
    #[serde(rename = "id")]
    pub r#id: Box<Option<String>>,
    /// Zone name.
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
}
