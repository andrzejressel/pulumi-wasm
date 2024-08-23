#[derive(serde::Serialize)]
pub struct ServiceTaskSpecPlacementPlatform {
    #[serde(rename = "architecture")]
    pub r#architecture: Box<String>,
    #[serde(rename = "os")]
    pub r#os: Box<String>,
}
