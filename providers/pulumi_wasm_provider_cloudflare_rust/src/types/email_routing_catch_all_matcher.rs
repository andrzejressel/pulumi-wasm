#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
pub struct EmailRoutingCatchAllMatcher {
    /// Type of matcher. Available values: `all`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type: Box<String>,
}
