#[derive(serde::Serialize)]
pub struct GetZonesZone {
    #[serde(rename = "id")]
    pub r#id: Box<Option<String>>,
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
}
