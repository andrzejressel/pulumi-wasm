#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct EnvironmentStorageConfigurationEfs {
    /// Id of the EFS filesystem to mount.
    #[builder(into)]
    #[serde(rename = "fileSystemId")]
    pub r#file_system_id: Box<String>,
    /// Path to mount the filesystem on, must start with `/m2/mount/`.
    #[builder(into)]
    #[serde(rename = "mountPoint")]
    pub r#mount_point: Box<String>,
}
