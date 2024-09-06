#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct EmailRoutingCatchAllMatcher {
    /// Type of matcher. Available values: `all`.
    #[serde(rename = "type")]
    pub r#type: Box<String>,
}
