#[derive(serde::Serialize)]
pub struct TeamsAccountExtendedEmailMatching {
    #[serde(rename = "enabled")]
    pub r#enabled: Box<bool>,
}
