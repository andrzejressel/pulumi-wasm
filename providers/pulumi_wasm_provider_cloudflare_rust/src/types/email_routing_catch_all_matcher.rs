#[derive(serde::Serialize)]
pub struct EmailRoutingCatchAllMatcher {
    #[serde(rename = "type")]
    pub r#type: Box<String>,
}
