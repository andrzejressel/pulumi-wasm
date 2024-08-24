#[derive(serde::Serialize)]
pub struct RemoteImageBuildUlimit {
    /// soft limit
    #[serde(rename = "hard")]
    pub r#hard: Box<i32>,
    /// type of ulimit, e.g. `nofile`
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// hard limit
    #[serde(rename = "soft")]
    pub r#soft: Box<i32>,
}
