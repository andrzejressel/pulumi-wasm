#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct WindowsFileSystemDiskIopsConfiguration {
    /// The total number of SSD IOPS provisioned for the file system.
    #[builder(into, default)]
    #[serde(rename = "iops")]
    pub r#iops: Box<Option<i32>>,
    /// Specifies whether the number of IOPS for the file system is using the system. Valid values are `AUTOMATIC` and `USER_PROVISIONED`. Default value is `AUTOMATIC`.
    #[builder(into, default)]
    #[serde(rename = "mode")]
    pub r#mode: Box<Option<String>>,
}
