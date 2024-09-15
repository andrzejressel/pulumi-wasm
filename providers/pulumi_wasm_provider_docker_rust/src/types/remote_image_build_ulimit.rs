#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
pub struct RemoteImageBuildUlimit {
    /// soft limit
    #[builder(into)]
    #[serde(rename = "hard")]
    pub r#hard: Box<i32>,
    /// type of ulimit, e.g. `nofile`
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// hard limit
    #[builder(into)]
    #[serde(rename = "soft")]
    pub r#soft: Box<i32>,
}
