#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct ServiceTaskSpecContainerSpecSecret {
    /// Represents the file GID. Defaults to `0`
    #[serde(rename = "fileGid")]
    pub r#file_gid: Box<Option<String>>,
    /// Represents represents the FileMode of the file. Defaults to `0o444`
    #[serde(rename = "fileMode")]
    pub r#file_mode: Box<Option<i32>>,
    /// Represents the final filename in the filesystem
    #[serde(rename = "fileName")]
    pub r#file_name: Box<String>,
    /// Represents the file UID. Defaults to `0`
    #[serde(rename = "fileUid")]
    pub r#file_uid: Box<Option<String>>,
    /// ID of the specific secret that we're referencing
    #[serde(rename = "secretId")]
    pub r#secret_id: Box<String>,
    /// Name of the secret that this references, but this is just provided for lookup/display purposes. The config in the reference will be identified by its ID
    #[serde(rename = "secretName")]
    pub r#secret_name: Box<Option<String>>,
}
