#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AppImageConfigKernelGatewayImageConfigFileSystemConfig {
    /// The default POSIX group ID (GID). If not specified, defaults to `100`. Valid values are `0` and `100`.
    #[builder(into, default)]
    #[serde(rename = "defaultGid")]
    pub r#default_gid: Box<Option<i32>>,
    /// The default POSIX user ID (UID). If not specified, defaults to `1000`. Valid values are `0` and `1000`.
    #[builder(into, default)]
    #[serde(rename = "defaultUid")]
    pub r#default_uid: Box<Option<i32>>,
    /// The path within the image to mount the user's EFS home directory. The directory should be empty. If not specified, defaults to `/home/sagemaker-user`.
    /// 
    /// > **Note:** When specifying `default_gid` and `default_uid`, Valid value pairs are [`0`, `0`] and [`100`, `1000`].
    #[builder(into, default)]
    #[serde(rename = "mountPath")]
    pub r#mount_path: Box<Option<String>>,
}
