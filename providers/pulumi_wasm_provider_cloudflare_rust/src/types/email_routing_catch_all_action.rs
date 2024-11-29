#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
pub struct EmailRoutingCatchAllAction {
    /// Type of supported action. Available values: `drop`, `forward`, `worker`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type: Box<String>,
    /// A list with items in the following form.
    #[builder(into)]
    #[serde(rename = "values")]
    pub r#values: Box<Vec<String>>,
}
