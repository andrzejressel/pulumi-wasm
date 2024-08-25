#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct EmailRoutingCatchAllAction {
    /// Type of supported action. Available values: `drop`, `forward`, `worker`.
    #[serde(rename = "type")]
    pub r#type: Box<String>,
    /// A list with items in the following form.
    #[serde(rename = "values")]
    pub r#values: Box<Vec<String>>,
}
