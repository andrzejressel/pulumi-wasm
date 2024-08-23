#[derive(serde::Serialize)]
pub struct DockerBuild {
    #[serde(rename = "addHosts")]
    pub r#add_hosts: Box<Option<Vec<String>>>,
    #[serde(rename = "args")]
    pub r#args: Box<Option<std::collections::HashMap<String, String>>>,
    #[serde(rename = "builderVersion")]
    pub r#builder_version: Box<Option<crate::types::BuilderVersion>>,
    #[serde(rename = "cacheFrom")]
    pub r#cache_from: Box<Option<crate::types::CacheFrom>>,
    #[serde(rename = "context")]
    pub r#context: Box<Option<String>>,
    #[serde(rename = "dockerfile")]
    pub r#dockerfile: Box<Option<String>>,
    #[serde(rename = "network")]
    pub r#network: Box<Option<String>>,
    #[serde(rename = "platform")]
    pub r#platform: Box<Option<String>>,
    #[serde(rename = "target")]
    pub r#target: Box<Option<String>>,
}
