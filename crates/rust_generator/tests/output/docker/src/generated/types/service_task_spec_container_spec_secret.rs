#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ServiceTaskSpecContainerSpecSecret {
    /// Represents the file GID. Defaults to `0`
    #[builder(into, default)]
    #[serde(rename = "fileGid")]
    pub r#file_gid: Box<Option<String>>,
    /// Represents represents the FileMode of the file. Defaults to `0o444`
    #[builder(into, default)]
    #[serde(rename = "fileMode")]
    pub r#file_mode: Box<Option<i32>>,
    /// Represents the final filename in the filesystem
    #[builder(into)]
    #[serde(rename = "fileName")]
    pub r#file_name: Box<String>,
    /// Represents the file UID. Defaults to `0`
    #[builder(into, default)]
    #[serde(rename = "fileUid")]
    pub r#file_uid: Box<Option<String>>,
    /// ID of the specific secret that we're referencing
    #[builder(into)]
    #[serde(rename = "secretId")]
    pub r#secret_id: Box<String>,
    /// Name of the secret that this references, but this is just provided for lookup/display purposes. The config in the reference will be identified by its ID
    #[builder(into, default)]
    #[serde(rename = "secretName")]
    pub r#secret_name: Box<Option<String>>,
}
