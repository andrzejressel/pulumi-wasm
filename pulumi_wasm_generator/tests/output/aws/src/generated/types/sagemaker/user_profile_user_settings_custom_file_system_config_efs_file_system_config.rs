#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct UserProfileUserSettingsCustomFileSystemConfigEfsFileSystemConfig {
    /// The ID of your Amazon EFS file system.
    #[builder(into)]
    #[serde(rename = "fileSystemId")]
    pub r#file_system_id: Box<String>,
    /// The path to the file system directory that is accessible in Amazon SageMaker Studio. Permitted users can access only this directory and below.
    #[builder(into, default)]
    #[serde(rename = "fileSystemPath")]
    pub r#file_system_path: Box<Option<String>>,
}