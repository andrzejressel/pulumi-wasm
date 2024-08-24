#[derive(serde::Serialize)]
pub struct ContainerUlimit {
    /// The hard limit
    #[serde(rename = "hard")]
    pub r#hard: Box<i32>,
    /// The name of the ulimit
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The soft limit
    #[serde(rename = "soft")]
    pub r#soft: Box<i32>,
}
