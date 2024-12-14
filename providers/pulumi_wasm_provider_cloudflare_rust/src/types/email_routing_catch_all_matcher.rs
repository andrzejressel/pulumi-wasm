#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct EmailRoutingCatchAllMatcher {
    /// Type of matcher. Available values: `all`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type: Box<String>,
}
