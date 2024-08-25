#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct ManagedHeadersManagedRequestHeader {
    /// Whether the headers rule is active.
    #[serde(rename = "enabled")]
    pub r#enabled: Box<bool>,
    /// Unique headers rule identifier.
    #[serde(rename = "id")]
    pub r#id: Box<String>,
}
