#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct PluginGrantPermission {
    /// The name of the permission
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The value of the permission
    #[serde(rename = "values")]
    pub r#values: Box<Vec<String>>,
}
