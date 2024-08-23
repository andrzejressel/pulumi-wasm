#[derive(serde::Serialize)]
pub struct ServiceTaskSpecContainerSpecConfig {
    #[serde(rename = "configId")]
    pub r#config_id: Box<String>,
    #[serde(rename = "configName")]
    pub r#config_name: Box<Option<String>>,
    #[serde(rename = "fileGid")]
    pub r#file_gid: Box<Option<String>>,
    #[serde(rename = "fileMode")]
    pub r#file_mode: Box<Option<i32>>,
    #[serde(rename = "fileName")]
    pub r#file_name: Box<String>,
    #[serde(rename = "fileUid")]
    pub r#file_uid: Box<Option<String>>,
}
