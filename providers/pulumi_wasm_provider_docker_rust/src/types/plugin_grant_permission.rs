#[derive(serde::Serialize)]
pub struct PluginGrantPermission {
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    #[serde(rename = "values")]
    pub r#values: Box<Vec<String>>,
}
