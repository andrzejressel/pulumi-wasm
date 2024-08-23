#[derive(serde::Serialize)]
pub struct ZoneLockdownConfiguration {
    #[serde(rename = "target")]
    pub r#target: Box<String>,
    #[serde(rename = "value")]
    pub r#value: Box<String>,
}
