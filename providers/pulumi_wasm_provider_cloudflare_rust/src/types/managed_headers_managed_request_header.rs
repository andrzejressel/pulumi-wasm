#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct ManagedHeadersManagedRequestHeader {
    /// Whether the headers rule is active.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<bool>,
    /// Unique headers rule identifier.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Box<String>,
}
