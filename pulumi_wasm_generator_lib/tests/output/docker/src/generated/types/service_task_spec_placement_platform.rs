#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct ServiceTaskSpecPlacementPlatform {
    /// The architecture, e.g. `amd64`
    #[builder(into)]
    #[serde(rename = "architecture")]
    pub r#architecture: Box<String>,
    /// The operation system, e.g. `linux`
    #[builder(into)]
    #[serde(rename = "os")]
    pub r#os: Box<String>,
}
