#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
pub struct ServiceTaskSpecContainerSpecConfig {
    /// ID of the specific config that we're referencing
    #[builder(into)]
    #[serde(rename = "configId")]
    pub r#config_id: Box<String>,
    /// Name of the config that this references, but this is just provided for lookup/display purposes. The config in the reference will be identified by its ID
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "configName")]
    pub r#config_name: Box<Option<String>>,
    /// Represents the file GID. Defaults to `0`.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "fileGid")]
    pub r#file_gid: Box<Option<String>>,
    /// Represents represents the FileMode of the file. Defaults to `0o444`.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "fileMode")]
    pub r#file_mode: Box<Option<i32>>,
    /// Represents the final filename in the filesystem
    #[builder(into)]
    #[serde(rename = "fileName")]
    pub r#file_name: Box<String>,
    /// Represents the file UID. Defaults to `0`.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "fileUid")]
    pub r#file_uid: Box<Option<String>>,
}
