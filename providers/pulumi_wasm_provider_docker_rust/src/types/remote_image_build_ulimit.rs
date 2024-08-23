#[derive(serde::Serialize)]
pub struct RemoteImageBuildUlimit {
    #[serde(rename = "hard")]
    pub r#hard: Box<i32>,
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    #[serde(rename = "soft")]
    pub r#soft: Box<i32>,
}
