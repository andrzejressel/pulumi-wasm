#[derive(serde::Serialize)]
pub struct ServiceTaskSpecPlacementPlatform {
    /// The architecture, e.g. `amd64`
    #[serde(rename = "architecture")]
    pub r#architecture: Box<String>,
    /// The operation system, e.g. `linux`
    #[serde(rename = "os")]
    pub r#os: Box<String>,
}
