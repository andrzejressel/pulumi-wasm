#[derive(serde::Serialize)]
pub struct ServiceTaskSpecContainerSpecSecret {
    #[serde(rename = "fileGid")]
    pub r#file_gid: Box<Option<String>>,
    #[serde(rename = "fileMode")]
    pub r#file_mode: Box<Option<i32>>,
    #[serde(rename = "fileName")]
    pub r#file_name: Box<String>,
    #[serde(rename = "fileUid")]
    pub r#file_uid: Box<Option<String>>,
    #[serde(rename = "secretId")]
    pub r#secret_id: Box<String>,
    #[serde(rename = "secretName")]
    pub r#secret_name: Box<Option<String>>,
}
