#[derive(serde::Serialize)]
pub struct ManagedHeadersManagedRequestHeader {
    #[serde(rename = "enabled")]
    pub r#enabled: Box<bool>,
    #[serde(rename = "id")]
    pub r#id: Box<String>,
}
