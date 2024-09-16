#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
pub struct PluginGrantPermission {
    /// The name of the permission
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The value of the permission
    #[builder(into)]
    #[serde(rename = "values")]
    pub r#values: Box<Vec<String>>,
}
