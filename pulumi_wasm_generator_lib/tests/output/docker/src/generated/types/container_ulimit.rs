#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct ContainerUlimit {
    /// The hard limit
    #[builder(into)]
    #[serde(rename = "hard")]
    pub r#hard: Box<i32>,
    /// The name of the ulimit
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The soft limit
    #[builder(into)]
    #[serde(rename = "soft")]
    pub r#soft: Box<i32>,
}
